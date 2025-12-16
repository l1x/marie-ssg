// src/main.rs
use argh::FromArgs;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;
use tracing::{debug, instrument};
use tracing::{error, info};

use crate::config::Config;
use crate::content::{Content, convert_content, load_content};
use crate::error::RunError;
use crate::output::{copy_static_files, write_output_file};
use crate::template::{init_environment, render_html, render_index_from_loaded};
use crate::utils::{
    add_date_prefix, find_markdown_files, get_content_type, get_content_type_template,
    get_output_path,
};

mod config;
mod content;
mod error;
mod output;
mod template;
mod utils;

fn default_config_file() -> String {
    "site.toml".to_string()
}

#[derive(FromArgs, Debug)]
/// A command line interface
struct Argz {
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

/// The main entry point for the application logic.
#[instrument(skip_all)]
pub(crate) fn run(argz: Argz) -> Result<(), RunError> {
    // loading config
    debug!("Args: {:?}", argz);
    info!("Config file: {}", argz.config_file);

    let config = Config::load_from_file(&argz.config_file).expect("Failed to load configuration");

    // Initialize template environment once
    let env = init_environment(&config.site.template_dir);

    // 0. Copy static files first
    //
    copy_static_files(&config)?;

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
            let html = convert_content(&content, file.clone())?;

            let mut output_path =
                get_output_path(file, &config.site.content_dir, &config.site.output_dir);
            if let Some(ct_config) = config.content_types.get(&content_type)
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

        let content_template = get_content_type_template(&config, &loaded.content_type);
        let rendered = render_html(
            env,
            &loaded.html,
            &loaded.content.meta,
            &config,
            &content_template,
        )?;
        write_output_file(&loaded.output_path, &rendered)?;
    }

    // 4. Render content type indexes
    //
    for (content_type, v) in config.content_types.iter() {
        info!(
            "Content type: {} -> Index Template: {:?}",
            content_type, v.index_template
        );

        let filtered: Vec<_> = loaded_contents
            .iter()
            .filter(|lc| &lc.content_type == content_type)
            .collect();

        let index_rendered = render_index_from_loaded(env, &config, &v.index_template, filtered)?;

        let output_path = PathBuf::from(&config.site.output_dir)
            .join(content_type)
            .join("index.html");

        write_output_file(&output_path, &index_rendered)?;
    }

    // 5. Render site index
    //
    let site_index_rendered = render_index_from_loaded(
        env,
        &config,
        &config.site.site_index_template,
        loaded_contents.iter().collect(),
    )?;

    write_output_file(
        &PathBuf::from(&config.site.output_dir).join("index.html"),
        &site_index_rendered,
    )?;

    info!("Process completed successfully.");
    Ok(())
}

fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    info!("Starting up...");

    // Parse CLI arguments
    let argz: Argz = argh::from_env();

    match run(argz) {
        Ok(_ok) => info!("ok"),
        Err(e) => error!("{:?}", e),
    }
}
