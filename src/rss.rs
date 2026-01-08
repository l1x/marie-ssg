// src/rss.rs

use std::path::Path;
use time::OffsetDateTime;

use crate::LoadedContent;
use crate::config::Config;
use crate::content::get_excerpt_html;

/// Generates an RSS 2.0 feed string for the site.
///
/// The feed includes content items filtered by the `rss_include` setting
/// in each content type's configuration. Items are sorted by date descending.
///
/// # Arguments
/// * `config` - The site configuration containing metadata and content type settings
/// * `loaded_contents` - All loaded content items to potentially include in the feed
///
/// # Returns
/// A string containing the complete RSS 2.0 XML feed
pub(crate) fn generate_rss(config: &Config, loaded_contents: &[LoadedContent]) -> String {
    let mut xml = String::new();
    let base_url = format!("https://{}", config.site.domain);

    // XML declaration and RSS opening tag with Atom namespace
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');
    xml.push_str(r#"<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">"#);
    xml.push('\n');
    xml.push_str("  <channel>\n");

    // Channel metadata
    xml.push_str(&format!(
        "    <title>{}</title>\n",
        xml_escape(&config.site.title)
    ));
    xml.push_str(&format!("    <link>{}</link>\n", base_url));
    xml.push_str(&format!(
        "    <description>{}</description>\n",
        xml_escape(&config.site.tagline)
    ));
    xml.push_str("    <language>en</language>\n");
    xml.push_str(&format!(
        "    <managingEditor>{}</managingEditor>\n",
        xml_escape(&config.site.author)
    ));

    // Atom self-link for feed readers
    xml.push_str(&format!(
        "    <atom:link href=\"{}/feed.xml\" rel=\"self\" type=\"application/rss+xml\"/>\n",
        base_url
    ));

    // Filter and sort content items
    let mut items: Vec<&LoadedContent> = loaded_contents
        .iter()
        .filter(|lc| should_include_in_rss(config, &lc.content_type))
        .collect();

    // Sort by date descending (newest first)
    items.sort_by(|a, b| b.content.meta.date.cmp(&a.content.meta.date));

    // Add items
    for content in items {
        xml.push_str(&format_item(config, content, &base_url));
    }

    // Close channel and rss
    xml.push_str("  </channel>\n");
    xml.push_str("</rss>\n");

    xml
}

/// Checks if a content type should be included in the RSS feed.
///
/// Returns true if:
/// - The content type is not in config (include by default)
/// - The content type's rss_include is None (include by default)
/// - The content type's rss_include is Some(true)
fn should_include_in_rss(config: &Config, content_type: &str) -> bool {
    config
        .content
        .get(content_type)
        .map(|ct| ct.rss_include.unwrap_or(true))
        .unwrap_or(true)
}

/// Formats a single RSS item entry.
fn format_item(config: &Config, content: &LoadedContent, base_url: &str) -> String {
    let mut item = String::new();
    item.push_str("    <item>\n");

    // Title
    item.push_str(&format!(
        "      <title>{}</title>\n",
        xml_escape(&content.content.meta.title)
    ));

    // Link and GUID
    let relative_path = content
        .output_path
        .strip_prefix(&config.site.output_dir)
        .unwrap_or(&content.output_path);
    let raw_path = path_to_url(relative_path);

    // For clean URLs, convert "slug/index.html" to "slug/"
    let url = if config.site.clean_urls {
        format!(
            "{}/{}",
            base_url,
            raw_path
                .strip_suffix("/index.html")
                .or_else(|| raw_path.strip_suffix("\\index.html"))
                .map(|s| format!("{}/", s))
                .unwrap_or(raw_path)
        )
    } else {
        format!("{}/{}", base_url, raw_path)
    };

    item.push_str(&format!("      <link>{}</link>\n", url));
    item.push_str(&format!("      <guid>{}</guid>\n", url));

    // Description (excerpt)
    let excerpt = get_excerpt_html(
        &content.content.data,
        "## Context",
        config.site.allow_dangerous_html,
    );
    if !excerpt.is_empty() {
        item.push_str(&format!(
            "      <description>{}</description>\n",
            xml_escape(&excerpt)
        ));
    }

    // Author
    item.push_str(&format!(
        "      <author>{}</author>\n",
        xml_escape(&content.content.meta.author)
    ));

    // Publication date in RFC 2822 format
    item.push_str(&format!(
        "      <pubDate>{}</pubDate>\n",
        format_rfc2822(&content.content.meta.date)
    ));

    item.push_str("    </item>\n");
    item
}

/// Formats a date in RFC 2822 format for RSS pubDate.
///
/// Example: "Mon, 15 Jan 2024 10:30:00 +0000"
fn format_rfc2822(date: &OffsetDateTime) -> String {
    use time::format_description::well_known::Rfc2822;
    date.format(&Rfc2822).unwrap_or_else(|_| String::new())
}

/// Converts a file path to a URL path.
///
/// Handles platform-specific path separators and ensures forward slashes.
fn path_to_url(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Escapes special XML characters in a string.
///
/// Replaces:
/// - `&` with `&amp;`
/// - `<` with `&lt;`
/// - `>` with `&gt;`
/// - `"` with `&quot;`
/// - `'` with `&apos;`
fn xml_escape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&apos;"),
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ContentTypeConfig, SiteConfig};
    use crate::content::{Content, ContentMeta};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn create_test_config() -> Config {
        let mut content = HashMap::new();
        content.insert(
            "posts".to_string(),
            ContentTypeConfig {
                index_template: "posts_index.html".to_string(),
                content_template: "post.html".to_string(),
                output_naming: None,
                rss_include: None, // Default: include
            },
        );

        Config {
            site: SiteConfig {
                title: "Test Site".to_string(),
                tagline: "A test site".to_string(),
                domain: "example.com".to_string(),
                author: "Test Author".to_string(),
                content_dir: "content".to_string(),
                output_dir: "output".to_string(),
                template_dir: "templates".to_string(),
                static_dir: "static".to_string(),
                site_index_template: "index.html".to_string(),
                syntax_highlighting_enabled: true,
                syntax_highlighting_theme: "github_dark".to_string(),
                root_static: HashMap::new(),
                sitemap_enabled: true,
                rss_enabled: true,
                allow_dangerous_html: false,
                header_uri_fragment: false,
                clean_urls: false,
                asset_hashing_enabled: false,
                asset_manifest_path: None,
            },
            content,
            dynamic: HashMap::new(),
        }
    }

    fn create_test_meta(title: &str, date_str: &str, author: &str) -> ContentMeta {
        use time::format_description::well_known::Rfc3339;
        let date = OffsetDateTime::parse(date_str, &Rfc3339).unwrap();
        ContentMeta {
            title: title.to_string(),
            date,
            author: author.to_string(),
            tags: vec![],
            template: None,
            cover: None,
            extra: std::collections::HashMap::new(),
        }
    }

    fn create_test_loaded_content(
        filename: &str,
        title: &str,
        date_str: &str,
        content_type: &str,
        markdown: &str,
    ) -> LoadedContent {
        LoadedContent {
            path: PathBuf::from(format!("content/{}/{}.md", content_type, filename)),
            content: Content {
                meta: create_test_meta(title, date_str, "Test Author"),
                data: markdown.to_string(),
            },
            html: "<h1>Test</h1>".to_string(),
            content_type: content_type.to_string(),
            output_path: PathBuf::from(format!("output/{}/{}.html", content_type, filename)),
        }
    }

    #[test]
    fn test_generate_rss_empty() {
        let config = create_test_config();
        let contents: Vec<LoadedContent> = vec![];

        let rss = generate_rss(&config, &contents);

        assert!(rss.contains(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
        assert!(rss.contains(r#"<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">"#));
        assert!(rss.contains("<title>Test Site</title>"));
        assert!(rss.contains("<link>https://example.com</link>"));
        assert!(rss.contains("<description>A test site</description>"));
        assert!(rss.contains("</rss>"));
    }

    #[test]
    fn test_generate_rss_with_content() {
        let config = create_test_config();
        let contents = vec![
            create_test_loaded_content(
                "hello-world",
                "Hello World",
                "2024-01-15T10:00:00+00:00",
                "posts",
                "# Hello\n\n## Context\n\nThis is the excerpt.",
            ),
            create_test_loaded_content(
                "second-post",
                "Second Post",
                "2024-02-20T12:00:00+00:00",
                "posts",
                "# Second\n\n## Context\n\nAnother excerpt.",
            ),
        ];

        let rss = generate_rss(&config, &contents);

        // Check items are included
        assert!(rss.contains("<title>Hello World</title>"));
        assert!(rss.contains("<title>Second Post</title>"));
        assert!(rss.contains("<link>https://example.com/posts/hello-world.html</link>"));
        assert!(rss.contains("<link>https://example.com/posts/second-post.html</link>"));
    }

    #[test]
    fn test_generate_rss_sorted_by_date() {
        let config = create_test_config();
        let contents = vec![
            create_test_loaded_content(
                "older",
                "Older Post",
                "2024-01-01T10:00:00+00:00",
                "posts",
                "# Older",
            ),
            create_test_loaded_content(
                "newer",
                "Newer Post",
                "2024-02-01T10:00:00+00:00",
                "posts",
                "# Newer",
            ),
        ];

        let rss = generate_rss(&config, &contents);

        // Newer post should appear before older post
        let newer_pos = rss.find("<title>Newer Post</title>").unwrap();
        let older_pos = rss.find("<title>Older Post</title>").unwrap();
        assert!(
            newer_pos < older_pos,
            "Newer post should appear before older post"
        );
    }

    #[test]
    fn test_generate_rss_filters_by_rss_include() {
        let mut config = create_test_config();
        config.content.insert(
            "pages".to_string(),
            ContentTypeConfig {
                index_template: "pages_index.html".to_string(),
                content_template: "page.html".to_string(),
                output_naming: None,
                rss_include: Some(false), // Exclude from RSS
            },
        );

        let contents = vec![
            create_test_loaded_content(
                "post",
                "A Post",
                "2024-01-15T10:00:00+00:00",
                "posts",
                "# Post",
            ),
            create_test_loaded_content(
                "about",
                "About Page",
                "2024-01-01T10:00:00+00:00",
                "pages",
                "# About",
            ),
        ];

        let rss = generate_rss(&config, &contents);

        // Post should be included
        assert!(rss.contains("<title>A Post</title>"));
        // Page should be excluded
        assert!(!rss.contains("<title>About Page</title>"));
    }

    #[test]
    fn test_generate_rss_with_excerpt() {
        let config = create_test_config();
        let contents = vec![create_test_loaded_content(
            "test",
            "Test Post",
            "2024-01-15T10:00:00+00:00",
            "posts",
            "# Test\n\n## Context\n\nThis is my excerpt content.\n\n## Other Section\n\nMore content.",
        )];

        let rss = generate_rss(&config, &contents);

        assert!(rss.contains("<description>"));
        assert!(rss.contains("excerpt content"));
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("hello"), "hello");
        assert_eq!(xml_escape("a & b"), "a &amp; b");
        assert_eq!(xml_escape("<tag>"), "&lt;tag&gt;");
        assert_eq!(xml_escape("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(xml_escape("it's"), "it&apos;s");
        assert_eq!(
            xml_escape("Tom & Jerry's <adventure>"),
            "Tom &amp; Jerry&apos;s &lt;adventure&gt;"
        );
    }

    #[test]
    fn test_format_rfc2822() {
        use time::format_description::well_known::Rfc3339;
        let date = OffsetDateTime::parse("2024-01-15T10:30:00+00:00", &Rfc3339).unwrap();
        let formatted = format_rfc2822(&date);

        // RFC 2822 format: "Mon, 15 Jan 2024 10:30:00 +0000"
        assert!(formatted.contains("2024"));
        assert!(formatted.contains("Jan"));
        assert!(formatted.contains("10:30:00"));
    }

    #[test]
    fn test_path_to_url() {
        let path = Path::new("posts/hello-world.html");
        assert_eq!(path_to_url(path), "posts/hello-world.html");
    }

    #[test]
    fn test_should_include_in_rss_default() {
        let config = create_test_config();

        // posts has rss_include: None -> should include
        assert!(should_include_in_rss(&config, "posts"));

        // unknown content type -> should include by default
        assert!(should_include_in_rss(&config, "unknown"));
    }

    #[test]
    fn test_should_include_in_rss_explicit() {
        let mut config = create_test_config();
        config.content.insert(
            "drafts".to_string(),
            ContentTypeConfig {
                index_template: "drafts_index.html".to_string(),
                content_template: "draft.html".to_string(),
                output_naming: None,
                rss_include: Some(false),
            },
        );

        assert!(!should_include_in_rss(&config, "drafts"));
    }

    #[test]
    fn test_rss_valid_xml_structure() {
        let config = create_test_config();
        let contents = vec![create_test_loaded_content(
            "test",
            "Test",
            "2024-01-15T10:00:00+00:00",
            "posts",
            "# Test",
        )];

        let rss = generate_rss(&config, &contents);

        // Count opening and closing tags
        let item_opens = rss.matches("<item>").count();
        let item_closes = rss.matches("</item>").count();
        assert_eq!(item_opens, item_closes);

        let channel_opens = rss.matches("<channel>").count();
        let channel_closes = rss.matches("</channel>").count();
        assert_eq!(channel_opens, channel_closes);
        assert_eq!(channel_opens, 1);
    }
}
