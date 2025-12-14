// src/config.rs

use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::{collections::HashMap, fs};
use thiserror::Error;
use tracing::{info, instrument};

use crate::Cli;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    /// Site
    pub site: SiteConfig,

    /// Custom variables accessible in templates
    #[serde(default)]
    pub dynamic: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SiteConfig {
    /// Title of the website or application
    pub title: String,
    /// Tagline message
    pub tagline: String,
    /// Domain name where the site will be hosted (e.g., "example.com")
    pub domain: String,
    /// Name of the author or site owner
    pub author: String,
    /// Directory path where generated output files will be saved
    pub output_dir: String,
    /// Directory path containing source content/markdown files
    pub content_dir: String,
    /// Directory path containing HTML templates/layouts
    pub template_dir: String,
    /// Directory path containing static assets (css, fonts, images, etc.)
    pub static_dir: String,
    /// Template for the site-wide index page
    pub site_index_template: String,
    /// After content dir is scanned this is filled up with the different content types found
    #[serde(default)]
    pub content_types: HashMap<String, ContentTypeConfig>,
    /// Static files that should be copied to the output root (e.g., favicon.ico, robots.txt)
    #[serde(default)]
    pub root_static: RootStaticConfig,
}

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("IO error reading config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error in config file: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("Config file not found: {0}")]
    FileNotFound(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ContentTypeConfig {
    pub index_template: String,
    pub content_template: String,
    #[serde(default)]
    pub output_naming: Option<String>, // Options: "default" or "date"
}

/// Configuration for static files that should be copied to the output root.
///
/// Key: Output filename (e.g., "favicon.ico", "robots.txt") - use quoted strings for filenames with dots
/// Value: Source path relative to static directory (e.g., "favicon.ico", "seo/robots.txt")
///
/// Example TOML configuration:
/// ```toml
/// [site.root_static]
/// "favicon.ico" = "favicon.ico"
/// "robots.txt" = "robots.txt"
/// "sitemap.xml" = "seo/sitemap.xml"
/// ```
pub(crate) type RootStaticConfig = HashMap<String, String>;

#[instrument(skip(cli), fields(path = %cli.config))]
pub(crate) fn load_config(cli: &Cli) -> Result<Config, ConfigError> {
    // Read the config file content, with a specific check for a missing file.
    let config_content = fs::read_to_string(&cli.config).map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
            // Provide the specific, user-friendly error.
            ConfigError::FileNotFound(cli.config.clone())
        } else {
            // For all other IO errors, use the generic `From<io::Error>` conversion.
            e.into()
        }
    })?;

    // Parse TOML config using the `?` operator for concise error handling.
    let config: Config = toml::from_str(&config_content)?;
    info!("TOML configuration parsed successfully.");
    Ok(config)
}
