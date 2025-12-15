// src/config.rs

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    /// Site
    pub site: SiteConfig,

    /// Custom variables accessible in templates
    #[serde(default)]
    pub dynamic: HashMap<String, String>,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    // Helper for tests - parses TOML from string
    fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(basic_toml::from_str(content)?)
    }
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
    pub root_static: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("IO error reading config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error in config file: {0}")]
    TomlParse(#[from] basic_toml::Error),
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
