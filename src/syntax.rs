// src/syntax.rs

use autumnus::formatter::Formatter;
use autumnus::languages::Language;
use autumnus::{HtmlInlineBuilder, themes};
use thiserror::Error;

/// Errors that can occur during syntax highlighting
#[derive(Error, Debug)]
pub(crate) enum SyntaxError {
    #[error("Failed to highlight code: {0}")]
    Highlight(String),

    #[error("Invalid theme '{0}': {1}")]
    InvalidTheme(String, String),

    #[error("I/O error during highlighting: {0}")]
    Io(#[from] std::io::Error),
}

/// Maps markdown language identifiers to Autumnus Language variants
fn map_lang_to_autumnus(lang: &str) -> Option<Language> {
    // Normalize the language identifier (lowercase, trim)
    let lang = lang.trim().to_lowercase();

    // Common language mappings
    match lang.as_str() {
        "rust" => Some(Language::Rust),
        "python" | "py" => Some(Language::Python),
        "javascript" | "js" => Some(Language::JavaScript),
        "typescript" | "ts" => Some(Language::TypeScript),
        "html" => Some(Language::HTML),
        "css" => Some(Language::CSS),
        "bash" | "sh" | "shell" => Some(Language::Bash),
        "json" => Some(Language::JSON),
        "toml" => Some(Language::Toml),
        "yaml" | "yml" => Some(Language::YAML),
        "plaintext" | "text" | "txt" => Some(Language::PlainText),
        _ => None,
    }
}

/// Highlights a single code block with the given language and theme
pub(crate) fn highlight_code_block(
    code: &str,
    lang: Option<&str>,
    theme_name: &str,
) -> Result<String, SyntaxError> {
    // Get the theme
    let theme = themes::get(theme_name)
        .map_err(|e| SyntaxError::InvalidTheme(theme_name.to_string(), e.to_string()))?;

    // Determine language
    let autumnus_lang = lang
        .and_then(map_lang_to_autumnus)
        .unwrap_or(Language::PlainText);

    // Build the formatter
    let formatter = HtmlInlineBuilder::new()
        .source(code)
        .lang(autumnus_lang)
        .theme(Some(theme))
        .pre_class(Some("code-block"))
        .build()
        .map_err(|e| SyntaxError::Highlight(e.to_string()))?;

    // Format to string
    let mut output = Vec::new();
    formatter
        .format(&mut output)
        .map_err(|e| SyntaxError::Highlight(e.to_string()))?;
    String::from_utf8(output)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e).into())
}

/// Extracts language from code block class attribute
fn extract_language_from_class(class: &str) -> Option<&str> {
    // Look for language-* patterns in the class attribute
    class
        .split_whitespace()
        .find(|c| c.starts_with("language-"))
        .map(|c| &c[9..]) // Skip "language-" prefix
}

/// Highlights all code blocks in HTML content
pub(crate) fn highlight_html(html: &str, theme_name: &str) -> Result<String, SyntaxError> {
    // If there are no <pre><code> blocks, return early
    if !html.contains("<pre><code") && !html.contains("<pre>\n<code") {
        return Ok(html.to_string());
    }

    // We'll use a simple regex-like approach to find and replace code blocks
    // This is simpler than a full HTML parser and works for the expected markdown output
    let mut result = String::with_capacity(html.len() * 2);
    let mut remaining = html;

    while let Some(start_idx) = remaining.find("<pre><code") {
        // Add everything before the code block
        result.push_str(&remaining[..start_idx]);

        // Find the end of the opening tag
        let tag_end = remaining[start_idx..].find('>').ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Malformed HTML: missing '>' in <code> tag",
            )
        })? + start_idx
            + 1;

        // Extract the opening tag
        let opening_tag = &remaining[start_idx..tag_end];

        // Check for language class
        let lang = if let Some(class_start) = opening_tag.find("class=\"") {
            let class_start = class_start + 7; // Skip "class=\""
            if let Some(class_end) = opening_tag[class_start..].find('"') {
                let class_str = &opening_tag[class_start..class_start + class_end];
                extract_language_from_class(class_str)
            } else {
                None
            }
        } else {
            None
        };

        // Find the closing </code></pre>
        let code_end_pattern = "</code></pre>";
        let code_end = remaining[tag_end..].find(code_end_pattern).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Malformed HTML: missing closing </code></pre>",
            )
        })?;

        let code_content = &remaining[tag_end..tag_end + code_end];
        let block_end = tag_end + code_end + code_end_pattern.len();

        // Highlight the code block
        let highlighted = highlight_code_block(code_content, lang, theme_name)?;

        // Add the highlighted block
        result.push_str(&highlighted);

        // Move past this block
        remaining = &remaining[block_end..];
    }

    // Add any remaining content
    result.push_str(remaining);

    Ok(result)
}

/// Default theme to use if none is specified
pub(crate) const DEFAULT_THEME: &str = "github_dark";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_lang_to_autumnus() {
        assert_eq!(map_lang_to_autumnus("rust"), Some(Language::Rust));
        assert_eq!(map_lang_to_autumnus("python"), Some(Language::Python));
        assert_eq!(map_lang_to_autumnus("py"), Some(Language::Python));
        assert_eq!(
            map_lang_to_autumnus("javascript"),
            Some(Language::JavaScript)
        );
        assert_eq!(map_lang_to_autumnus("js"), Some(Language::JavaScript));
        assert_eq!(
            map_lang_to_autumnus("typescript"),
            Some(Language::TypeScript)
        );
        assert_eq!(map_lang_to_autumnus("ts"), Some(Language::TypeScript));
        assert_eq!(map_lang_to_autumnus("html"), Some(Language::HTML));
        assert_eq!(map_lang_to_autumnus("css"), Some(Language::CSS));
        assert_eq!(map_lang_to_autumnus("bash"), Some(Language::Bash));
        assert_eq!(map_lang_to_autumnus("json"), Some(Language::JSON));
        assert_eq!(map_lang_to_autumnus("toml"), Some(Language::Toml));
        assert_eq!(map_lang_to_autumnus("yaml"), Some(Language::YAML));
        assert_eq!(map_lang_to_autumnus("yml"), Some(Language::YAML));
        assert_eq!(map_lang_to_autumnus("plaintext"), Some(Language::PlainText));
        assert_eq!(map_lang_to_autumnus("unknown"), None);
    }

    #[test]
    fn test_extract_language_from_class() {
        assert_eq!(extract_language_from_class("language-rust"), Some("rust"));
        assert_eq!(
            extract_language_from_class("hljs language-python"),
            Some("python")
        );
        assert_eq!(
            extract_language_from_class("language-javascript highlight"),
            Some("javascript")
        );
        assert_eq!(extract_language_from_class("no-language-here"), None);
        assert_eq!(extract_language_from_class(""), None);
    }

    #[test]
    fn test_highlight_code_block_basic() {
        let code = "fn main() {\n    println!(\"Hello\");\n}";
        let result = highlight_code_block(code, Some("rust"), DEFAULT_THEME);
        assert!(result.is_ok());
        let html = result.unwrap();
        // Should contain the code wrapped in <pre><code>
        assert!(html.contains("<pre"));
        assert!(html.contains("<code"));
        assert!(html.contains("language-rust"));
        // Check that code content is present (may be split across span tags)
        assert!(html.contains("fn"));
        assert!(html.contains("main"));
        assert!(html.contains("println"));
        assert!(!html.is_empty());
    }

    #[test]
    fn test_highlight_code_block_unknown_language() {
        let code = "some code";
        let result = highlight_code_block(code, Some("unknownlang"), DEFAULT_THEME);
        assert!(result.is_ok()); // Should fall back to plain text
    }

    #[test]
    fn test_highlight_code_block_no_language() {
        let code = "just plain text";
        let result = highlight_code_block(code, None, DEFAULT_THEME);
        assert!(result.is_ok());
    }

    #[test]
    fn test_highlight_code_block_empty() {
        let code = "";
        let result = highlight_code_block(code, Some("rust"), DEFAULT_THEME);
        assert!(result.is_ok());
        let html = result.unwrap();
        // Empty code block should still produce valid HTML
        assert!(html.contains("<pre"));
        assert!(html.contains("<code"));
        assert!(html.contains("language-rust"));
    }

    #[test]
    fn test_highlight_html_no_code_blocks() {
        let html = "<p>Some text</p><h1>Heading</h1>";
        let result = highlight_html(html, DEFAULT_THEME);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), html);
    }

    #[test]
    fn test_highlight_html_with_code_block() {
        let html = r#"<p>Before</p>
<pre><code class="language-rust">fn main() {
    println!("test");
}</code></pre>
<p>After</p>"#;

        let result = highlight_html(html, DEFAULT_THEME);
        assert!(result.is_ok());
        let highlighted = result.unwrap();

        // Should contain the original structure
        assert!(highlighted.contains("<p>Before</p>"));
        assert!(highlighted.contains("<p>After</p>"));
        // Should have highlighted the code block
        assert!(highlighted.contains("fn"));
        assert!(highlighted.contains("main"));
        assert!(highlighted.contains("println"));
        // Should preserve the language class
        assert!(highlighted.contains("language-rust"));
    }

    #[test]
    fn test_highlight_html_multiple_blocks() {
        let html = r#"<pre><code class="language-python">print("hello")</code></pre>
<pre><code>plain text</code></pre>"#;

        let result = highlight_html(html, DEFAULT_THEME);
        assert!(result.is_ok());
        let highlighted = result.unwrap();

        // Should process both blocks
        assert!(!highlighted.is_empty());
        assert!(highlighted.contains("print"));
        assert!(highlighted.contains("language-python"));
        assert!(highlighted.contains("plain text"));
    }

    #[test]
    fn test_highlight_html_with_empty_code_block() {
        let html = r#"<p>Before</p>
<pre><code class="language-rust"></code></pre>
<p>After</p>"#;

        let result = highlight_html(html, DEFAULT_THEME);
        assert!(result.is_ok());
        let highlighted = result.unwrap();

        // Should contain the original structure
        assert!(highlighted.contains("<p>Before</p>"));
        assert!(highlighted.contains("<p>After</p>"));
        // Should have empty code block with language class
        assert!(highlighted.contains("language-rust"));
        assert!(highlighted.contains("<pre"));
        assert!(highlighted.contains("<code"));
    }
}
