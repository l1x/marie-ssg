// src/redirect.rs

use std::path::PathBuf;

/// Generates HTML content for a redirect page.
///
/// The generated HTML uses meta refresh for instant redirect, includes a canonical
/// link for SEO, and provides a fallback anchor link for users.
///
/// # Arguments
/// * `target_path` - The URL path to redirect to (e.g., "/articles/2025-12-29-my-post/")
/// * `domain` - The site domain for the canonical URL (e.g., "example.com")
///
/// # Returns
/// A string containing the complete HTML redirect page
pub(crate) fn generate_redirect_html(target_path: &str, domain: &str) -> String {
    let canonical_url = format!("https://{}{}", domain, target_path);

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; url={target_path}">
  <link rel="canonical" href="{canonical_url}">
  <title>Redirecting...</title>
</head>
<body>
  <p>Redirecting to <a href="{target_path}">{target_path}</a>...</p>
</body>
</html>
"#,
        target_path = target_path,
        canonical_url = canonical_url
    )
}

/// Converts a URL path to an output file path.
///
/// Handles different path formats:
/// - Paths ending with `/` → append `index.html`
/// - Paths ending with `.html` → use as-is
/// - Other paths → append `/index.html` (assume clean URL)
///
/// # Arguments
/// * `from_path` - The source URL path (e.g., "/articles/old-slug/")
/// * `output_dir` - The output directory (e.g., "dist")
///
/// # Returns
/// A PathBuf representing the output file location
pub(crate) fn get_redirect_output_path(from_path: &str, output_dir: &str) -> PathBuf {
    // Strip leading slash for path joining
    let path = from_path.trim_start_matches('/');

    let file_path = if from_path.ends_with('/') {
        // Clean URL ending with slash: /articles/slug/ → articles/slug/index.html
        format!("{}index.html", path)
    } else if from_path.ends_with(".html") {
        // Already has .html extension: /articles/slug.html → articles/slug.html
        path.to_string()
    } else {
        // No trailing slash or extension: /articles/slug → articles/slug/index.html
        format!("{}/index.html", path)
    };

    PathBuf::from(output_dir).join(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_redirect_html_basic() {
        let html = generate_redirect_html("/articles/new-post/", "example.com");

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains(r#"content="0; url=/articles/new-post/""#));
        assert!(html.contains(r#"href="https://example.com/articles/new-post/""#));
        assert!(html.contains(r#"<a href="/articles/new-post/">"#));
    }

    #[test]
    fn test_generate_redirect_html_with_date_prefix() {
        let html = generate_redirect_html("/articles/2025-12-29-my-article/", "www.vectorian.be");

        assert!(html.contains(r#"url=/articles/2025-12-29-my-article/"#));
        assert!(html.contains("https://www.vectorian.be/articles/2025-12-29-my-article/"));
    }

    #[test]
    fn test_get_redirect_output_path_trailing_slash() {
        let result = get_redirect_output_path("/articles/old-slug/", "dist");
        assert_eq!(result, PathBuf::from("dist/articles/old-slug/index.html"));
    }

    #[test]
    fn test_get_redirect_output_path_html_extension() {
        let result = get_redirect_output_path("/articles/old-slug.html", "dist");
        assert_eq!(result, PathBuf::from("dist/articles/old-slug.html"));
    }

    #[test]
    fn test_get_redirect_output_path_no_trailing_slash() {
        let result = get_redirect_output_path("/articles/old-slug", "dist");
        assert_eq!(result, PathBuf::from("dist/articles/old-slug/index.html"));
    }

    #[test]
    fn test_get_redirect_output_path_root() {
        let result = get_redirect_output_path("/old-page/", "output");
        assert_eq!(result, PathBuf::from("output/old-page/index.html"));
    }

    #[test]
    fn test_get_redirect_output_path_nested() {
        let result = get_redirect_output_path("/blog/2024/january/post/", "out");
        assert_eq!(
            result,
            PathBuf::from("out/blog/2024/january/post/index.html")
        );
    }

    #[test]
    fn test_get_redirect_output_path_different_output_dir() {
        let result = get_redirect_output_path("/page/", "public/html");
        assert_eq!(result, PathBuf::from("public/html/page/index.html"));
    }
}
