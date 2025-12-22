// src/template.rs

use minijinja::{Environment, context, path_loader};
use minijinja_contrib::add_to_environment;
use std::sync::OnceLock;

use crate::{
    config::Config,
    content::{ContentItem, ContentMeta, get_excerpt_html},
};

static ENV: OnceLock<Environment<'static>> = OnceLock::new();

/// Initialize and return the global template environment (cached, for single builds)
pub(crate) fn init_environment(template_dir: &str) -> &'static Environment<'static> {
    ENV.get_or_init(|| {
        let mut env = Environment::new();
        env.set_loader(path_loader(template_dir));
        add_to_environment(&mut env);
        env
    })
}

/// Create a fresh template environment (uncached, for watch mode)
pub(crate) fn create_environment(template_dir: &str) -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader(template_dir));
    add_to_environment(&mut env);
    env
}

pub(crate) fn render_index_from_loaded(
    env: &Environment,
    config: &Config,
    index_template_name: &str,
    loaded: Vec<&crate::LoadedContent>,
    all_content: Vec<&crate::LoadedContent>,
) -> Result<String, minijinja::Error> {
    let tmpl = env.get_template(index_template_name)?;

    let mut contents: Vec<ContentItem> = loaded
        .iter()
        .map(|lc| {
            let filename = lc
                .output_path
                .strip_prefix(&config.site.output_dir)
                .unwrap_or(&lc.output_path)
                .to_string_lossy()
                .to_string();

            let excerpt = get_excerpt_html(&lc.content.data, "## Context");

            ContentItem {
                html: lc.html.clone(),
                meta: lc.content.meta.clone(),
                formatted_date: lc.content.meta.date.format("%B %d, %Y").to_string(),
                filename,
                content_type: lc.content_type.clone(),
                excerpt,
            }
        })
        .collect();

    contents.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));

    let mut all_contents: Vec<ContentItem> = all_content
        .iter()
        .map(|lc| {
            let filename = lc
                .output_path
                .strip_prefix(&config.site.output_dir)
                .unwrap_or(&lc.output_path)
                .to_string_lossy()
                .to_string();

            let excerpt = get_excerpt_html(&lc.content.data, "## Context");

            ContentItem {
                html: lc.html.clone(),
                meta: lc.content.meta.clone(),
                formatted_date: lc.content.meta.date.format("%B %d, %Y").to_string(),
                filename,
                content_type: lc.content_type.clone(),
                excerpt,
            }
        })
        .collect();

    all_contents.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));

    let context = context! {
        config => config,
        contents => contents,
        all_content => all_contents,
    };

    tmpl.render(context)
}

pub(crate) fn render_html(
    env: &Environment,
    html: &str,
    meta: &ContentMeta,
    config: &Config,
    content_template: &str,
) -> Result<String, minijinja::Error> {
    let tmpl = env.get_template(content_template)?;

    let context = context! {
        content => html,
        meta => meta,
        config => config
    };

    tmpl.render(context)
}

// Note: Template rendering functions accept an Environment parameter for testability.
// Production code uses init_environment() to get a static singleton, while tests
// can create custom environments with test-specific template directories.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LoadedContent;
    use crate::config::{Config, SiteConfig};
    use crate::content::ContentMeta;
    use chrono::DateTime;
    use minijinja::Environment;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Helper to create a test Config
    fn create_test_config(template_dir: &str, output_dir: &str) -> Config {
        Config {
            site: SiteConfig {
                title: "Test Site".to_string(),
                tagline: "A test tagline".to_string(),
                domain: "example.com".to_string(),
                author: "Test Author".to_string(),
                content_dir: "content".to_string(),
                output_dir: output_dir.to_string(),
                template_dir: template_dir.to_string(),
                static_dir: "static".to_string(),
                site_index_template: "index.html".to_string(),
                root_static: HashMap::new(),
            },
            content: HashMap::new(),
            dynamic: HashMap::new(),
        }
    }

    /// Helper to create a test ContentMeta
    fn create_test_meta() -> ContentMeta {
        ContentMeta {
            title: "Test Article".to_string(),
            date: DateTime::parse_from_rfc3339("2024-01-15T10:00:00-05:00").unwrap(),
            author: "Test Author".to_string(),
            tags: vec!["rust".to_string(), "testing".to_string()],
            template: None,
        }
    }

    #[test]
    fn test_render_html_with_simple_template() {
        // Create a temporary directory for templates
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("test.html");

        // Write a simple template - use 'safe' filter to render unescaped HTML
        std::fs::write(
            &template_path,
            "<h1>{{ meta.title }}</h1><div>{{ content | safe }}</div>",
        )
        .unwrap();

        // Create environment and config
        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        add_to_environment(&mut env);
        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        // Test rendering
        let meta = create_test_meta();
        let html = "<p>Test content</p>";
        let result = render_html(&env, html, &meta, &config, "test.html");

        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("<h1>Test Article</h1>"));
        assert!(rendered.contains("<p>Test content</p>"));
    }

    #[test]
    fn test_render_html_with_metadata_fields() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("full.html");

        std::fs::write(
            &template_path,
            r#"<article>
                <h1>{{ meta.title }}</h1>
                <p>By {{ meta.author }}</p>
                <div>{{ content | safe }}</div>
                <p>Tags: {{ meta.tags | join(", ") }}</p>
            </article>"#,
        )
        .unwrap();

        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        add_to_environment(&mut env);
        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        let meta = create_test_meta();
        let result = render_html(&env, "<p>Body</p>", &meta, &config, "full.html");

        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("Test Article"));
        assert!(rendered.contains("Test Author"));
        assert!(rendered.contains("rust, testing"));
    }

    #[test]
    fn test_datetimeformat_filter_available() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("date.html");

        std::fs::write(&template_path, "<p>{{ meta.date | datetimeformat }}</p>").unwrap();

        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        add_to_environment(&mut env);

        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        let meta = create_test_meta();
        let result = render_html(&env, "<p>Body</p>", &meta, &config, "date.html");

        let rendered = result.expect("datetimeformat filter should render");
        assert!(rendered.contains("Jan 15 2024"));
    }

    #[test]
    fn test_render_html_missing_template() {
        let temp_dir = TempDir::new().unwrap();

        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        let meta = create_test_meta();
        let result = render_html(&env, "<p>Test</p>", &meta, &config, "nonexistent.html");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("nonexistent.html"));
    }

    #[test]
    fn test_render_index_from_loaded_empty_list() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("index.html");

        std::fs::write(
            &template_path,
            "<h1>{{ config.site.title }}</h1><p>{{ contents | length }} items</p>",
        )
        .unwrap();

        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        let result = render_index_from_loaded(&env, &config, "index.html", vec![], vec![]);

        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("Test Site"));
        assert!(rendered.contains("0 items"));
    }

    #[test]
    fn test_render_index_from_loaded_with_content() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("index.html");

        std::fs::write(
            &template_path,
            r#"<h1>{{ config.site.title }}</h1>
            {% for item in contents %}
            <article>
                <h2>{{ item.meta.title }}</h2>
                <p>{{ item.formatted_date }}</p>
            </article>
            {% endfor %}"#,
        )
        .unwrap();

        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        // Create test LoadedContent
        let loaded = LoadedContent {
            path: PathBuf::from("test.md"),
            content: crate::content::Content {
                meta: create_test_meta(),
                data: "# Test".to_string(),
            },
            html: "<h1>Test</h1>".to_string(),
            content_type: "blog".to_string(),
            output_path: PathBuf::from("output/blog/test.html"),
        };

        let result =
            render_index_from_loaded(&env, &config, "index.html", vec![&loaded], vec![&loaded]);

        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("Test Site"));
        assert!(rendered.contains("Test Article"));
        assert!(rendered.contains("January 15, 2024"));
    }

    #[test]
    fn test_render_index_sorts_by_date_descending() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("index.html");

        std::fs::write(
            &template_path,
            r#"{% for item in contents %}{{ item.meta.title }},{% endfor %}"#,
        )
        .unwrap();

        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        // Create three LoadedContent items with different dates
        let mut meta_old = create_test_meta();
        meta_old.title = "Old Post".to_string();
        meta_old.date = DateTime::parse_from_rfc3339("2024-01-01T10:00:00-05:00").unwrap();

        let mut meta_new = create_test_meta();
        meta_new.title = "New Post".to_string();
        meta_new.date = DateTime::parse_from_rfc3339("2024-12-15T10:00:00-05:00").unwrap();

        let mut meta_mid = create_test_meta();
        meta_mid.title = "Mid Post".to_string();
        meta_mid.date = DateTime::parse_from_rfc3339("2024-06-15T10:00:00-05:00").unwrap();

        let loaded_old = LoadedContent {
            path: PathBuf::from("old.md"),
            content: crate::content::Content {
                meta: meta_old,
                data: "# Old".to_string(),
            },
            html: "<h1>Old</h1>".to_string(),
            content_type: "blog".to_string(),
            output_path: PathBuf::from("output/blog/old.html"),
        };

        let loaded_new = LoadedContent {
            path: PathBuf::from("new.md"),
            content: crate::content::Content {
                meta: meta_new,
                data: "# New".to_string(),
            },
            html: "<h1>New</h1>".to_string(),
            content_type: "blog".to_string(),
            output_path: PathBuf::from("output/blog/new.html"),
        };

        let loaded_mid = LoadedContent {
            path: PathBuf::from("mid.md"),
            content: crate::content::Content {
                meta: meta_mid,
                data: "# Mid".to_string(),
            },
            html: "<h1>Mid</h1>".to_string(),
            content_type: "blog".to_string(),
            output_path: PathBuf::from("output/blog/mid.html"),
        };

        // Pass in non-sorted order
        let result = render_index_from_loaded(
            &env,
            &config,
            "index.html",
            vec![&loaded_old, &loaded_new, &loaded_mid],
            vec![&loaded_old, &loaded_new, &loaded_mid],
        );

        assert!(result.is_ok());
        let rendered = result.unwrap();
        // Should be sorted newest first: New, Mid, Old
        assert_eq!(rendered, "New Post,Mid Post,Old Post,");
    }

    #[test]
    fn test_render_index_with_excerpt() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("index.html");

        std::fs::write(
            &template_path,
            r#"{% for item in contents %}<div>{{ item.excerpt }}</div>{% endfor %}"#,
        )
        .unwrap();

        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        let loaded = LoadedContent {
            path: PathBuf::from("test.md"),
            content: crate::content::Content {
                meta: create_test_meta(),
                data: "# Title\n\n## Context\n\nThis is the excerpt.".to_string(),
            },
            html: "<h1>Title</h1>".to_string(),
            content_type: "blog".to_string(),
            output_path: PathBuf::from("output/blog/test.html"),
        };

        let result =
            render_index_from_loaded(&env, &config, "index.html", vec![&loaded], vec![&loaded]);

        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("This is the excerpt"));
    }

    #[test]
    fn test_render_index_with_all_content() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("index.html");

        std::fs::write(
            &template_path,
            r#"<h1>{{ config.site.title }}</h1>
            <p>Filtered: {{ contents | length }} items</p>
            <p>All: {{ all_content | length }} items</p>
            {% for item in all_content %}
            <span class="all-item">{{ item.meta.title }}</span>
            {% endfor %}"#,
        )
        .unwrap();

        let mut env = Environment::new();
        env.set_loader(path_loader(temp_dir.path()));
        let config = create_test_config(temp_dir.path().to_str().unwrap(), "output");

        // Create two test LoadedContent items
        let mut meta1 = create_test_meta();
        meta1.title = "First Post".to_string();
        meta1.date = DateTime::parse_from_rfc3339("2024-01-15T10:00:00-05:00").unwrap();

        let mut meta2 = create_test_meta();
        meta2.title = "Second Post".to_string();
        meta2.date = DateTime::parse_from_rfc3339("2024-02-15T10:00:00-05:00").unwrap();

        let loaded1 = LoadedContent {
            path: PathBuf::from("first.md"),
            content: crate::content::Content {
                meta: meta1,
                data: "# First".to_string(),
            },
            html: "<h1>First</h1>".to_string(),
            content_type: "blog".to_string(),
            output_path: PathBuf::from("output/blog/first.html"),
        };

        let loaded2 = LoadedContent {
            path: PathBuf::from("second.md"),
            content: crate::content::Content {
                meta: meta2,
                data: "# Second".to_string(),
            },
            html: "<h1>Second</h1>".to_string(),
            content_type: "blog".to_string(),
            output_path: PathBuf::from("output/blog/second.html"),
        };

        // Pass only first item in filtered list, but both in all_content
        let result = render_index_from_loaded(
            &env,
            &config,
            "index.html",
            vec![&loaded1],
            vec![&loaded1, &loaded2],
        );

        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("Filtered: 1 items"));
        assert!(rendered.contains("All: 2 items"));
        assert!(rendered.contains("First Post"));
        assert!(rendered.contains("Second Post"));
    }
}
