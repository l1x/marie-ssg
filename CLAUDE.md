# Marie SSG - Agent Guidelines

## Project Overview

Marie SSG (Super Simple Static Site Generator) is a Rust-based static site generator that converts markdown files with TOML metadata into HTML pages using Jinja-style templates.

## Build Commands

```bash
mise run verify                     # Full verification (lint + tests) before commit
mise run lint                       # Run cargo lint (formatting + style checks)
mise run tests                      # Run all tests with --nocapture
mise run unit-tests                 # Run unit tests only
mise run integration-tests          # Run integration tests only
mise run build-dev                  # Development build
mise run build-prod                 # Release build (optimized for size)
cargo test <test_name>              # Run a single test
bd list                             # List open tickets
bd ready                            # to find available work
bd create                           # for new issues,
bd update <id> --status=in_progress # to claim work
bd close <id>                       # closing ticket
```

## Architecture

The SSG follows a pipeline architecture:

1. **Config loading** (`config.rs`) - Parses `site.toml` into `Config` struct with nested `SiteConfig`
2. **Content discovery** (`utils.rs`) - Finds markdown files in content directory
3. **Parallel content loading** (`content.rs` + `main.rs`) - Uses Rayon to load/convert markdown in parallel
4. **Template rendering** (`template.rs`) - Renders HTML using minijinja with template caching via `OnceLock`
5. **Output writing** (`output.rs`) - Writes rendered HTML and copies static files

Key data flow:

- `LoadedContent` struct (in `main.rs`) holds the loaded content, converted HTML, content type, and output path
- Content is loaded once and reused for both individual pages and index generation
- Template environment is cached globally to avoid recreation per render

## Content Type System

Content types are determined by directory structure under `content/`:

- `content/posts/*.md` → content type "posts"
- `content/pages/*.md` → content type "pages"

Each content type can have:

- A content template (e.g., `post.html`)
- An index template (e.g., `posts_index.html`)
- Optional date-prefix output naming

## Syntax Highlighting

Marie includes syntax highlighting for code blocks using the Autumnus library:

- Enabled by default (`syntax_highlighting_enabled = true`)
- Supports Rust, Python, JavaScript, TypeScript, HTML, CSS, Bash, JSON, TOML, YAML
- Theme configurable via `syntax_highlighting_theme` (default: "github_dark")
- Code blocks in markdown are automatically highlighted during HTML conversion
- Highlighting applied via `convert_content_with_highlighting()` in content pipeline

## Configuration Structure

```toml
# site.toml
author = "..."
title = "..."
domain = "..."
tagline = "..."
content_dir = "content"
template_dir = "templates"
static_dir = "static"
output_dir = "output"
site_index_template = "site_index.html"
syntax_highlighting_enabled = true          # Optional, defaults to true
syntax_highlighting_theme = "github_dark"   # Optional, defaults to "github_dark"

[site.root_static]
"favicon.ico" = "favicon.ico"
"robots.txt" = "robots.txt"

[dynamic]
# Custom template variables
```

## Code Style

- Group imports: std first, external crates, then internal modules (`use crate::*`)
- Use `thiserror` for custom error types with `#[source]` for error chaining
- Use `tracing` for logging with `#[instrument(skip_all)]` on key functions
- Prefer `pub(crate)` for internal visibility
- Use Rayon for parallel processing of independent operations

## Common Development Patterns

### Adding a New Content Type

To add a new content type (e.g., "projects"):

1. **Create content directory**: `content/projects/`
2. **Add configuration to `site.toml`**:
```toml
[content.projects]
index_template = "projects_index.html"
content_template = "project.html"
output_naming = "default"  # or "date" for date-prefixed URLs
```
3. **Create templates** in `templates/`:
   - `project.html` - individual project page
   - `projects_index.html` - project listing page
4. **Add markdown files** to `content/projects/` with TOML frontmatter

Template variables available:
- `{{ content }}` - rendered HTML
- `{{ meta.title }}`, `{{ meta.date }}`, `{{ meta.author }}`, `{{ meta.tags }}`
- `{{ config.site.* }}` - site configuration
- `{{ config.dynamic.* }}` - custom variables

### Adding a New Template Variable

To add custom variables accessible in all templates:

1. **Add to `[dynamic]` section** in `site.toml`:
```toml
[dynamic]
github_url = "https://github.com/username"
twitter_handle = "@username"
analytics_id = "UA-XXXXX"
```

2. **Access in templates** via `{{ config.dynamic.variable_name }}`:
```html
<a href="{{ config.dynamic.github_url }}">GitHub</a>
```

### Adding a New CLI Flag

To add a new CLI flag using `argh`:

1. **Edit `src/main.rs`** - add field to appropriate struct:
```rust
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "build")]
struct BuildArgs {
    #[argh(option, short = 'c', default = "default_config_file()")]
    config_file: String,

    /// enable verbose output (NEW FLAG)
    #[argh(switch, short = 'v')]
    verbose: bool,
}
```

2. **Use the flag** in the command handler:
```rust
Some(SubCommand::Build(args)) => {
    if args.verbose {
        // handle verbose mode
    }
}
```

Flag types:
- `#[argh(switch)]` - boolean flag (present/absent)
- `#[argh(option)]` - flag with value
- `#[argh(positional)]` - positional argument

### Adding a New Error Type

To add a new error variant using `thiserror`:

1. **Define error in module** (e.g., `src/mymodule.rs`):
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MyError {
    #[error("Description of error: {0}")]
    VariantName(String),

    #[error("I/O error for {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
```

2. **Add to `RunError`** in `src/error.rs`:
```rust
#[derive(Error, Debug)]
pub(crate) enum RunError {
    #[error("Failed to do X")]
    MyModule(#[from] MyError),
    // ... existing variants
}
```

3. **Use `?` operator** for automatic conversion:
```rust
fn my_function() -> Result<(), RunError> {
    something_that_returns_my_error()?;  // auto-converts to RunError
    Ok(())
}
```

## Verification

- Run `mise run verify` before commits (runs lint + tests)
