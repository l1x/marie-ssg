// src/main.rs

use clap::Parser;
use std::path::PathBuf;
use tracing::{debug, instrument};
use tracing::{error, info};

use crate::config::load_config;
use crate::content::{convert_content, load_content};
use crate::error::RunError;
use crate::output::{copy_static_files, write_output_file};
use crate::template::{get_content_by_type, render_html, render_index_with_contents};
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the config file
    #[arg(short, long, default_value = "site.toml")]
    config: String,
}

// Application Logic

/// The main entry point for the application logic.
#[instrument(name = "run", skip_all, fields(config_path = %cli.config))]
pub(crate) fn run(cli: Cli) -> Result<(), RunError> {
    // loading config
    let config = load_config(&cli)?;
    info!(?config, "Configuration loaded successfully");

    // 0. Copy static files first
    copy_static_files(&config)?;

    // 1. Find all markdown files in `config.content_dir`.
    let files = find_markdown_files(&config.content_dir);
    debug!("{:?}", files);

    // 2. For each file parse `ContentMeta` (toml) and Content (md) and render the content type specific template.
    for file in &files {
        info!("Processing file: {:?}", file);
        let content_type = get_content_type(&file, &config.content_dir);
        info!("Content type is: {:?} for {:?}", content_type, file);
        let content_template = get_content_type_template(&config, &content_type);
        let content = load_content(&file)?;
        let html = convert_content(&content, file.clone())?;
        // 2.a Rendering HTML
        let rendered = render_html(
            &html,
            &content.meta,
            &config,
            &config.template_dir,
            &content_template,
        )?;

        //2.b. Writing out the rendered HTML
        let mut output_path = get_output_path(&file, &config.content_dir, &config.output_dir);

        // Check if this content type uses date prefix for output naming
        if let Some(content_type_config) = config.content_types.get(&content_type) {
            if content_type_config.output_naming.as_deref() == Some("date") {
                // Apply date prefix
                output_path = add_date_prefix(output_path, &content.meta.date);
            }
        };

        info!("Output path: {:?} for {:?}", output_path, file);
        write_output_file(&output_path, &rendered)?;
    }

    // 3. Rendering contet type indexes

    for (content_type, v) in config.content_types.iter() {
        info!(
            "Content type: {} -> Index Template: {:?}",
            content_type, v.index_template
        );

        let contentz = get_content_by_type(&files, content_type);
        let index_rendered = render_index_with_contents(&config, &v.index_template, contentz)?;
        // Determine the output path for this index
        let output_path = PathBuf::from(&config.output_dir)
            .join(content_type)
            .join("index.html");

        // Write the rendered content to the output file
        write_output_file(&output_path, &index_rendered)?;
    }
    // 4. Rendering site index

    let site_index_rendered =
        render_index_with_contents(&config, &config.site_index_template, files.iter().collect())?;

    write_output_file(
        &PathBuf::from(&config.output_dir).join("index.html"),
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
    let cli = Cli::parse();

    match run(cli) {
        Ok(_ok) => info!("ok"),
        Err(e) => error!("{:?}", e),
    }
}
