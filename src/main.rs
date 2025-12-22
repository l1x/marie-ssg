// src/main.rs
use argh::FromArgs;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;
use tracing::{debug, instrument};
use tracing::{error, info};

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
#[instrument(skip_all)]
pub(crate) fn build(config_file: &str) -> Result<(), RunError> {
    let config = Config::load_from_file(config_file).expect("Failed to load configuration");
    let env = init_environment(&config.site.template_dir);
    run_build(config_file, &config, env)
}

/// Build with a fresh template environment (for watch mode).
#[instrument(skip_all)]
pub(crate) fn build_fresh(config_file: &str) -> Result<(), RunError> {
    let config = Config::load_from_file(config_file).expect("Failed to load configuration");
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
#[instrument(skip_all)]
fn run_build(
    config_file: &str,
    config: &Config,
    env: &minijinja::Environment,
) -> Result<(), RunError> {
    info!("Config file: {}", config_file);

    // 0. Copy static files first
    //
    copy_static_files(config)?;

    // 1. Find all markdown files in `config.content_dir`.
    //
    let files = find_markdown_files(&config.site.content_dir);
    debug!("{:?}", files);

    // 2. Loading all content
    //
    let start = std::time::Instant::now();

    let loaded_contents: Vec<LoadedContent> = files
        .par_iter() // Parallel iterator
        .map(|file| -> Result<LoadedContent, RunError> {
            info!("Loading: {}", file.display());

            let content_type = get_content_type(file, &config.site.content_dir);
            let content = load_content(file)?;
            let html = convert_content_with_highlighting(
                &content,
                file.clone(),
                config.site.syntax_highlighting_enabled,
                &config.site.syntax_highlighting_theme,
            )?;

            let mut output_path =
                get_output_path(file, &config.site.content_dir, &config.site.output_dir);
            if let Some(ct_config) = config.content.get(&content_type)
                && ct_config.output_naming.as_deref() == Some("date")
            {
                output_path = add_date_prefix(output_path, &content.meta.date);
            }

            Ok(LoadedContent {
                path: file.clone(),
                content,
                html,
                content_type,
                output_path,
            })
        })
        .collect::<Result<Vec<_>, _>>()?; // Collect Results, fail fast on error

    info!(
        "Loaded {} files in {:?}",
        loaded_contents.len(),
        start.elapsed()
    );

    // 3. Write individual pages
    //
    for loaded in &loaded_contents {
        info!(
            "Rendering '{}' ({} -> {})",
            loaded.content.meta.title,
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
        info!(
            "Content type: {} -> Index Template: {:?}",
            content_type, v.index_template
        );

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

    info!("Process completed successfully.");
    Ok(())
}

/// Watch for file changes and rebuild automatically (macOS only)
#[cfg(target_os = "macos")]
fn watch(config_file: &str) -> Result<(), RunError> {
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::{Duration, Instant};

    // Load config to get directories to watch
    let config = Config::load_from_file(config_file).expect("Failed to load configuration");

    let paths_to_watch = get_paths_to_watch(config_file, &config);

    info!("Watching directories: {:?}", paths_to_watch);
    info!("Press Ctrl+C to stop");

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
                    debug!("Debouncing, skipping rebuild");
                    continue;
                }

                info!("Changes detected: {:?}", events);
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

fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

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
            }
        }
        Some(SubCommand::Watch(args)) => {
            if let Err(e) = watch(&args.config_file) {
                error!("{:?}", e);
            }
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
