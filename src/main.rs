// src/main.rs
use argh::FromArgs;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::PathBuf;
use tracing::debug;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::Config;
use crate::content::{Content, convert_content_with_highlighting, load_content};
use crate::error::RunError;
use crate::output::{copy_static_files, write_output_file};
use crate::template::{
    create_environment, init_environment, render_html, render_index_from_loaded,
};
use crate::utils::{
    add_date_prefix, find_markdown_files, get_content_type, get_content_type_template,
    get_output_path,
};

mod config;
mod content;
mod error;
mod output;
mod rss;
mod sitemap;
mod syntax;
mod template;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn default_config_file() -> String {
    "site.toml".to_string()
}

#[derive(FromArgs, Debug)]
/// Marie SSG - Super Simple Static Site Generator
struct Argz {
    /// print version information
    #[argh(switch, short = 'V')]
    version: bool,

    #[argh(subcommand)]
    command: Option<SubCommand>,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommand {
    Build(BuildArgs),
    Watch(WatchArgs),
    Guide(GuideArgs),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "build")]
/// Build the static site
struct BuildArgs {
    /// path to the config file
    #[argh(option, short = 'c', default = "default_config_file()")]
    config_file: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "watch")]
/// Watch for changes and rebuild automatically
struct WatchArgs {
    /// path to the config file
    #[argh(option, short = 'c', default = "default_config_file()")]
    config_file: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "guide")]
/// Print a guide explaining Marie SSG features and configuration
struct GuideArgs {}

// Application Logic
#[derive(Debug)]
pub(crate) struct LoadedContent {
    pub(crate) path: PathBuf,
    pub(crate) content: Content,
    pub(crate) html: String,
    pub(crate) content_type: String,
    pub(crate) output_path: PathBuf,
}

/// The main entry point for the application logic (uses cached templates).
pub(crate) fn build(config_file: &str) -> Result<(), RunError> {
    let config = Config::load_from_file(config_file)?;
    let env = init_environment(&config.site.template_dir);
    run_build(config_file, &config, env)
}

/// Build with a fresh template environment (for watch mode).
pub(crate) fn build_fresh(config_file: &str) -> Result<(), RunError> {
    let config = Config::load_from_file(config_file)?;
    let env = create_environment(&config.site.template_dir);
    run_build(config_file, &config, &env)
}

/// Get the list of file paths/directories to watch for changes.
pub(crate) fn get_paths_to_watch(config_file: &str, config: &Config) -> Vec<String> {
    vec![
        config_file.to_string(),
        config.site.content_dir.clone(),
        config.site.template_dir.clone(),
        config.site.static_dir.clone(),
    ]
}

/// Core build logic that accepts a template environment.
fn run_build(
    config_file: &str,
    config: &Config,
    env: &minijinja::Environment,
) -> Result<(), RunError> {
    debug!("config::load ← {}", config_file);

    // 0. Copy static files first
    //
    copy_static_files(config)?;

    // 1. Find all markdown files in `config.content_dir`.
    //
    let files = find_markdown_files(&config.site.content_dir);
    debug!("content::scan found {} files", files.len());

    // 2. Loading all content
    //
    let start = std::time::Instant::now();

    let loaded_contents: Vec<LoadedContent> = files
        .into_par_iter() // Parallel iterator - consumes Vec for owned PathBufs
        .map(|file| -> Result<LoadedContent, RunError> {
            debug!("content::load ← {}", file.display());

            let content_type = get_content_type(&file, &config.site.content_dir);
            let content = load_content(&file)?;
            let html = convert_content_with_highlighting(
                &content,
                &file, // Pass reference - no clone needed
                config.site.syntax_highlighting_enabled,
                &config.site.syntax_highlighting_theme,
                config.site.allow_dangerous_html,
            )?;

            let mut output_path =
                get_output_path(&file, &config.site.content_dir, &config.site.output_dir);
            if let Some(ct_config) = config.content.get(&content_type)
                && ct_config.output_naming.as_deref() == Some("date")
            {
                output_path = add_date_prefix(output_path, &content.meta.date);
            }

            Ok(LoadedContent {
                path: file, // Move owned PathBuf - no clone needed
                content,
                html,
                content_type,
                output_path,
            })
        })
        .collect::<Result<Vec<_>, _>>()?; // Collect Results, fail fast on error

    info!(
        "content::load {} files in {:.2?}",
        loaded_contents.len(),
        start.elapsed()
    );

    // 3. Write individual pages
    //
    for loaded in &loaded_contents {
        debug!(
            "content::render {} → {}",
            loaded.path.display(),
            loaded.output_path.display()
        );

        let content_template = get_content_type_template(config, &loaded.content_type);
        let rendered = render_html(
            env,
            &loaded.html,
            &loaded.content.meta,
            config,
            &content_template,
        )?;
        write_output_file(&loaded.output_path, &rendered)?;
    }

    // 4. Render content type indexes
    //
    for (content_type, v) in config.content.iter() {
        debug!("index::render {} → {}", content_type, v.index_template);

        let filtered: Vec<_> = loaded_contents
            .iter()
            .filter(|lc| &lc.content_type == content_type)
            .collect();

        let index_rendered = render_index_from_loaded(
            env,
            config,
            &v.index_template,
            filtered,
            loaded_contents.iter().collect(),
        )?;

        let output_path = PathBuf::from(&config.site.output_dir)
            .join(content_type)
            .join("index.html");

        write_output_file(&output_path, &index_rendered)?;
    }

    // 5. Render site index
    //
    let site_index_rendered = render_index_from_loaded(
        env,
        config,
        &config.site.site_index_template,
        loaded_contents.iter().collect(),
        loaded_contents.iter().collect(),
    )?;

    write_output_file(
        &PathBuf::from(&config.site.output_dir).join("index.html"),
        &site_index_rendered,
    )?;

    // 6. Generate sitemap.xml (if enabled)
    //
    if config.site.sitemap_enabled {
        let sitemap_xml = sitemap::generate_sitemap(config, &loaded_contents);
        write_output_file(
            &PathBuf::from(&config.site.output_dir).join("sitemap.xml"),
            &sitemap_xml,
        )?;
        info!("sitemap::write → sitemap.xml");
    }

    // 7. Generate RSS feed (if enabled)
    //
    if config.site.rss_enabled {
        let rss_xml = rss::generate_rss(config, &loaded_contents);
        write_output_file(
            &PathBuf::from(&config.site.output_dir).join("feed.xml"),
            &rss_xml,
        )?;
        info!("rss::write → feed.xml");
    }

    info!("build::complete ✓");
    Ok(())
}

/// Watch for file changes and rebuild automatically (macOS only)
#[cfg(target_os = "macos")]
fn watch(config_file: &str) -> Result<(), RunError> {
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::{Duration, Instant};

    // Load config to get directories to watch
    let config = Config::load_from_file(config_file)?;

    let paths_to_watch = get_paths_to_watch(config_file, &config);

    info!("watch::start {:?}", paths_to_watch);
    info!("watch::info press Ctrl+C to stop");

    // Initial build (use fresh environment from the start)
    if let Err(e) = build_fresh(config_file) {
        error!("Initial build failed: {:?}", e);
    }

    let (sender, receiver) = channel();

    let _watcher_thread = thread::spawn(move || {
        let fsevent = fsevent::FsEvent::new(paths_to_watch);
        fsevent.observe(sender);
    });

    // Debounce: track last build time
    let mut last_build = Instant::now();
    let debounce_duration = Duration::from_millis(500);

    loop {
        match receiver.recv() {
            Ok(events) => {
                // Check debounce
                if last_build.elapsed() < debounce_duration {
                    debug!("watch::debounce skipping rebuild");
                    continue;
                }

                // Log event_id at INFO, full details at DEBUG
                info!("watch::change event_id: {}", events.event_id);
                debug!("watch::change {:?}", events);
                last_build = Instant::now();

                if let Err(e) = build_fresh(config_file) {
                    error!("Build failed: {:?}", e);
                }
            }
            Err(e) => {
                error!("Watch error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn watch(_config_file: &str) -> Result<(), RunError> {
    eprintln!("Watch mode is only supported on macOS");
    std::process::exit(1);
}

/// Prints the Marie SSG guide to stdout
fn print_guide() {
    print!(
        r####"# Marie SSG Guide

Marie is a static site generator that converts markdown files with TOML metadata into HTML pages.

## Quick Start

```bash
marie-ssg build              # Build the site
marie-ssg build -c prod.toml # Build with custom config
marie-ssg watch              # Watch and rebuild on changes (macOS)
marie-ssg guide              # Show this guide
```

## Project Structure

```
my-site/
├── site.toml           # Site configuration
├── content/            # Markdown content files
│   ├── blog/
│   │   ├── hello.md
│   │   └── hello.meta.toml
│   └── pages/
│       ├── about.md
│       └── about.meta.toml
├── templates/          # Jinja-style templates
│   ├── base.html
│   ├── post.html
│   └── blog_index.html
├── static/             # Static assets (CSS, images, fonts)
│   ├── css/
│   └── images/
└── output/             # Generated site (created by build)
```

## Configuration (site.toml)

```toml
[site]
title = "My Website"
tagline = "A personal blog"
domain = "example.com"
author = "Your Name"
content_dir = "content"
output_dir = "output"
template_dir = "templates"
static_dir = "static"
site_index_template = "index.html"

# Optional features (defaults shown)
syntax_highlighting_enabled = true   # Enable code block highlighting
syntax_highlighting_theme = "github_dark"
sitemap_enabled = true               # Generate sitemap.xml
rss_enabled = true                   # Generate feed.xml
allow_dangerous_html = false         # Allow raw HTML in markdown (for <figure>, inline SVGs, etc.)

# Files copied to output root (e.g., favicon)
[site.root_static]
"favicon.ico" = "favicon.ico"
"robots.txt" = "robots.txt"

# Content type configurations
[content.blog]
index_template = "blog_index.html"
content_template = "post.html"
output_naming = "date"      # Prefix output with date (YYYY-MM-DD-slug.html)
rss_include = true          # Include in RSS feed (default: true)

[content.pages]
index_template = "pages_index.html"
content_template = "page.html"
rss_include = false         # Exclude from RSS feed

# Custom variables for templates
[dynamic]
github_url = "https://github.com/user"
twitter = "@username"
```

## Content Files

Each markdown file needs a companion `.meta.toml` file:

**content/blog/hello.md:**
```markdown
# Hello World

This is my first post.

## Context

This section becomes the excerpt for RSS feeds and index pages.

## Main Content

The rest of your article...
```

**content/blog/hello.meta.toml:**
```toml
title = "Hello World"
date = "2024-01-15T10:00:00+00:00"  # RFC 3339 format
author = "Your Name"
tags = ["intro", "blog"]
template = "custom.html"             # Optional: override default template
```

### Metadata Fields

| Field    | Required | Description                                              |
|----------|----------|----------------------------------------------------------|
| title    | Yes      | Article title                                            |
| date     | Yes      | Publication date (RFC 3339: `YYYY-MM-DDTHH:MM:SS+00:00`) |
| author   | Yes      | Author name                                              |
| tags     | Yes      | Array of tags (can be empty: `[]`)                       |
| template | No       | Override the content type's default template             |

## Templates (Jinja2/Minijinja)

Templates use Jinja2 syntax via the Minijinja library.

### Available Context

**In content templates (`post.html`):**
- `content` - Rendered HTML content
- `meta.title`, `meta.date`, `meta.author`, `meta.tags`
- `config.site.title`, `config.site.author`, etc.
- `config.dynamic.github_url`, etc.

**In index templates (`blog_index.html`):**
- `contents` - List of ContentItem for this content type
- `all_content` - List of all ContentItem across all types
- `config` - Full site configuration

### ContentItem Properties

```jinja
{{% for item in contents %}}
  <h2>{{{{ item.meta.title }}}}</h2>
  <time>{{{{ item.formatted_date }}}}</time>
  <p>{{{{ item.excerpt | safe }}}}</p>
  <a href="/{{{{ item.filename | url }}}}">Read more</a>
{{% endfor %}}
```

| Property              | Description                                        |
|-----------------------|----------------------------------------------------|
| `item.html`           | Full rendered HTML content                         |
| `item.meta.title`     | Article title                                      |
| `item.meta.date`      | Date object                                        |
| `item.meta.author`    | Author name                                        |
| `item.meta.tags`      | List of tags                                       |
| `item.formatted_date` | Human-readable date (e.g., "January 15, 2024")     |
| `item.filename`       | Output path (e.g., `blog/2024-01-15-hello.html`)   |
| `item.content_type`   | Content type (e.g., "blog")                        |
| `item.excerpt`        | HTML excerpt from "## Context" section             |

### Filters

- `| safe` - Render HTML without escaping
- `| url` - URL-encode for href attributes
- `| datetimeformat("%Y-%m-%d")` - Format dates

### Template Example

```html
{{% extends "base.html" %}}

{{% block content %}}
<article>
  <h1>{{{{ meta.title }}}}</h1>
  <time>{{{{ meta.date | datetimeformat("%B %d, %Y") }}}}</time>
  <div class="content">{{{{ content | safe }}}}</div>
</article>
{{% endblock %}}
```

## Features

### Syntax Highlighting

Code blocks with language hints are highlighted automatically.

Supported languages: bash, css, html, javascript, json, python, rust, toml, typescript, yaml

Themes: `github_dark` (default), `monokai`, and others from Autumnus.

### Sitemap Generation

Automatically generates `sitemap.xml` with all pages when `sitemap_enabled = true`.

### RSS Feed Generation

Generates `feed.xml` with RSS 2.0 format when `rss_enabled = true`.
- Control per content type with `rss_include = true/false`
- Uses "## Context" section as excerpt

### Watch Mode (macOS)

Automatically rebuilds when files change:
```bash
marie-ssg watch
```

## Output

After build, your site is in the output directory:

```
output/
├── index.html          # Site homepage
├── sitemap.xml         # Sitemap (if enabled)
├── feed.xml            # RSS feed (if enabled)
├── favicon.ico         # Root static files
├── static/             # Copied static assets
├── blog/
│   ├── index.html      # Blog index
│   └── 2024-01-15-hello.html
└── pages/
    ├── index.html
    └── about.html
```

## Tips

1. **Date prefix**: Use `output_naming = "date"` to prefix files with publication date
2. **Excerpts**: Add a "## Context" section for RSS/index excerpts
3. **Custom templates**: Override per-article with `template` in metadata
4. **Dynamic vars**: Add custom variables in `[dynamic]` for use in templates

---
Generated by marie-ssg {version}
"####,
        version = VERSION
    );
}

fn main() {
    // Initialize tracing subscriber with env filter and clean format
    // Format: "2025-01-03T12:00:00Z INFO message" (no module path, no spans)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "marie_ssg=info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(tracing_subscriber::fmt::time::UtcTime::new(
                    kiters::timestamp::get_utc_formatter(),
                ))
                .with_target(false) // Remove module path (marie_ssg::output)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE), // Remove span prefixes
        )
        .init();

    // Parse CLI arguments
    let argz: Argz = argh::from_env();

    if argz.version {
        println!("marie-ssg {}", VERSION);
        return;
    }

    match argz.command {
        Some(SubCommand::Build(args)) => {
            if let Err(e) = build(&args.config_file) {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
        Some(SubCommand::Watch(args)) => {
            if let Err(e) = watch(&args.config_file) {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
        Some(SubCommand::Guide(_)) => {
            print_guide();
        }
        None => {
            println!("marie-ssg {}", VERSION);
            println!("Use --help for usage information");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paths_to_watch() {
        let toml = r#"
[site]
title = "Test Site"
tagline = "A test tagline"
domain = "example.com"
author = "Test Author"
output_dir = "output"
content_dir = "content"
template_dir = "templates"
static_dir = "static"
site_index_template = "index.html"
"#;
        let config = crate::config::Config::from_str(toml).unwrap();
        let config_file = "site.toml";

        let paths = get_paths_to_watch(config_file, &config);

        // Should contain 4 paths: config file + 3 dirs
        assert_eq!(paths.len(), 4);
        assert!(paths.contains(&"site.toml".to_string()));
        assert!(paths.contains(&"content".to_string()));
        assert!(paths.contains(&"templates".to_string()));
        assert!(paths.contains(&"static".to_string()));
    }
}
