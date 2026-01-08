// src/config.rs

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use thiserror::Error;
use tracing::debug;

use crate::syntax::DEFAULT_THEME;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    /// Site
    pub site: SiteConfig,

    /// Content type configurations (e.g., posts, pages, projects)
    #[serde(default)]
    pub content: HashMap<String, ContentTypeConfig>,

    /// Custom variables accessible in templates
    #[serde(default)]
    pub dynamic: HashMap<String, String>,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, ConfigError> {
        debug!("io::read ← {:?}", path);
        let content = fs::read_to_string(path)?;
        debug!("io::read {} bytes", content.len());
        Self::from_str(&content)
    }

    // Helper for tests - parses TOML from string
    pub(crate) fn from_str(content: &str) -> Result<Self, ConfigError> {
        Ok(toml::from_str(content)?)
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
    /// Enable syntax highlighting for code blocks
    #[serde(default = "default_true")]
    pub syntax_highlighting_enabled: bool,
    /// Theme to use for syntax highlighting (e.g., "github_dark", "monokai")
    #[serde(default = "default_syntax_theme")]
    pub syntax_highlighting_theme: String,
    /// Static files that should be copied to the output root (e.g., favicon.ico, robots.txt)
    #[serde(default)]
    pub root_static: HashMap<String, String>,
    /// Enable sitemap.xml generation
    #[serde(default = "default_true")]
    pub sitemap_enabled: bool,
    /// Enable RSS feed generation (feed.xml)
    #[serde(default = "default_true")]
    pub rss_enabled: bool,
    /// Allow raw HTML in markdown content (security: only enable for trusted content)
    #[serde(default)]
    pub allow_dangerous_html: bool,
    /// Generate anchor links for headers (h1-h6) enabling URL fragment navigation
    #[serde(default)]
    pub header_uri_fragment: bool,
    /// Enable clean URL structure: output as <type>/<slug>/index.html instead of <type>/<slug>.html
    #[serde(default)]
    pub clean_urls: bool,
    /// Enable content-based asset hashing for CSS/JS files (cache busting)
    #[serde(default)]
    pub asset_hashing_enabled: bool,
}

fn default_true() -> bool {
    true
}

fn default_syntax_theme() -> String {
    DEFAULT_THEME.to_string()
}

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("IO error reading config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error in config file: {0}")]
    TomlParse(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ContentTypeConfig {
    pub index_template: String,
    pub content_template: String,
    #[serde(default)]
    pub output_naming: Option<String>, // Options: "default" or "date"
    #[serde(default)]
    pub rss_include: Option<bool>, // Include in RSS feed (default: true if None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    fn minimal_config_toml() -> &'static str {
        r#"
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
"#
    }

    #[test]
    fn test_config_from_str_minimal() {
        let config = Config::from_str(minimal_config_toml()).unwrap();

        assert_eq!(config.site.title, "Test Site");
        assert_eq!(config.site.tagline, "A test tagline");
        assert_eq!(config.site.domain, "example.com");
        assert_eq!(config.site.author, "Test Author");
        assert_eq!(config.site.output_dir, "output");
        assert_eq!(config.site.content_dir, "content");
        assert!(config.content.is_empty());
        assert!(config.site.root_static.is_empty());
        assert!(config.dynamic.is_empty());
    }

    #[test]
    fn test_config_from_str_with_content_types() {
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

[content.posts]
index_template = "posts_index.html"
content_template = "post.html"
output_naming = "date"

[content.pages]
index_template = "pages_index.html"
content_template = "page.html"
"#;

        let config = Config::from_str(toml).unwrap();

        assert_eq!(config.content.len(), 2);

        let posts = config.content.get("posts").unwrap();
        assert_eq!(posts.index_template, "posts_index.html");
        assert_eq!(posts.content_template, "post.html");
        assert_eq!(posts.output_naming, Some("date".to_string()));

        let pages = config.content.get("pages").unwrap();
        assert_eq!(pages.index_template, "pages_index.html");
        assert_eq!(pages.content_template, "page.html");
        assert_eq!(pages.output_naming, None);
    }

    #[test]
    fn test_config_from_str_with_dynamic() {
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

[dynamic]
github_url = "https://github.com/user"
twitter_handle = "@user"
"#;

        let config = Config::from_str(toml).unwrap();

        assert_eq!(config.dynamic.len(), 2);
        assert_eq!(
            config.dynamic.get("github_url").unwrap(),
            "https://github.com/user"
        );
        assert_eq!(config.dynamic.get("twitter_handle").unwrap(), "@user");
    }

    #[test]
    fn test_config_from_str_invalid_toml() {
        let invalid = "this is not valid toml [[[";
        let result = Config::from_str(invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_str_missing_required_field() {
        let incomplete = r#"
[site]
title = "Test Site"
"#;
        let result = Config::from_str(incomplete);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_load_from_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("site.toml");

        let mut file = std::fs::File::create(&config_path).unwrap();
        file.write_all(minimal_config_toml().as_bytes()).unwrap();

        let config = Config::load_from_file(config_path.to_str().unwrap()).unwrap();

        assert_eq!(config.site.title, "Test Site");
        assert_eq!(config.site.domain, "example.com");
    }

    #[test]
    fn test_config_load_from_file_not_found() {
        let result = Config::load_from_file("/nonexistent/path/config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_with_root_static() {
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

[site.root_static]
"favicon.ico" = "favicon.ico"
"robots.txt" = "seo/robots.txt"
"#;

        let config = Config::from_str(toml).unwrap();

        assert_eq!(config.site.root_static.len(), 2);
        assert_eq!(
            config.site.root_static.get("favicon.ico").unwrap(),
            "favicon.ico"
        );
        assert_eq!(
            config.site.root_static.get("robots.txt").unwrap(),
            "seo/robots.txt"
        );
    }

    #[test]
    fn test_config_sitemap_enabled_default() {
        let config = Config::from_str(minimal_config_toml()).unwrap();

        // sitemap_enabled should default to true
        assert!(
            config.site.sitemap_enabled,
            "sitemap_enabled should default to true"
        );
    }

    #[test]
    fn test_config_sitemap_disabled() {
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
sitemap_enabled = false
"#;

        let config = Config::from_str(toml).unwrap();

        assert!(
            !config.site.sitemap_enabled,
            "sitemap_enabled should be false when explicitly set"
        );
    }

    #[test]
    fn test_config_rss_enabled_default() {
        let config = Config::from_str(minimal_config_toml()).unwrap();

        assert!(
            config.site.rss_enabled,
            "rss_enabled should default to true"
        );
    }

    #[test]
    fn test_config_rss_disabled() {
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
rss_enabled = false
"#;

        let config = Config::from_str(toml).unwrap();

        assert!(
            !config.site.rss_enabled,
            "rss_enabled should be false when explicitly set"
        );
    }

    #[test]
    fn test_config_rss_include_default() {
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

[content.posts]
index_template = "posts_index.html"
content_template = "post.html"
"#;

        let config = Config::from_str(toml).unwrap();
        let posts = config.content.get("posts").unwrap();

        assert!(
            posts.rss_include.is_none(),
            "rss_include should default to None"
        );
    }

    #[test]
    fn test_config_rss_include_explicit() {
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

[content.posts]
index_template = "posts_index.html"
content_template = "post.html"
rss_include = true

[content.pages]
index_template = "pages_index.html"
content_template = "page.html"
rss_include = false
"#;

        let config = Config::from_str(toml).unwrap();

        let posts = config.content.get("posts").unwrap();
        assert_eq!(posts.rss_include, Some(true));

        let pages = config.content.get("pages").unwrap();
        assert_eq!(pages.rss_include, Some(false));
    }

    #[test]
    fn test_config_allow_dangerous_html_default() {
        let config = Config::from_str(minimal_config_toml()).unwrap();

        assert!(
            !config.site.allow_dangerous_html,
            "allow_dangerous_html should default to false"
        );
    }

    #[test]
    fn test_config_allow_dangerous_html_enabled() {
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
allow_dangerous_html = true
"#;

        let config = Config::from_str(toml).unwrap();

        assert!(
            config.site.allow_dangerous_html,
            "allow_dangerous_html should be true when explicitly set"
        );
    }

    #[test]
    fn test_config_header_uri_fragment_default() {
        let config = Config::from_str(minimal_config_toml()).unwrap();

        assert!(
            !config.site.header_uri_fragment,
            "header_uri_fragment should default to false"
        );
    }

    #[test]
    fn test_config_header_uri_fragment_enabled() {
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
header_uri_fragment = true
"#;

        let config = Config::from_str(toml).unwrap();

        assert!(
            config.site.header_uri_fragment,
            "header_uri_fragment should be true when explicitly set"
        );
    }

    #[test]
    fn test_config_clean_urls_default() {
        let config = Config::from_str(minimal_config_toml()).unwrap();

        assert!(
            !config.site.clean_urls,
            "clean_urls should default to false"
        );
    }

    #[test]
    fn test_config_clean_urls_enabled() {
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
clean_urls = true
"#;

        let config = Config::from_str(toml).unwrap();

        assert!(
            config.site.clean_urls,
            "clean_urls should be true when explicitly set"
        );
    }
}
