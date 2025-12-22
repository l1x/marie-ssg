// src/utils.rs

use chrono::{DateTime, FixedOffset};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;

/// Extracts the content type from a file path relative to the content directory.
///
/// The content type is determined by the first directory component after stripping
/// the content directory prefix from the file path. If the path cannot be stripped
/// or has no directory components, it defaults to "page".
///
/// # Arguments
/// * `file` - The full path to the content file
/// * `content_dir` - The base content directory path
///
/// # Returns
/// A string representing the content type (e.g., "projects", "blog", "page")
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use your_crate::get_content_type;
///
/// let file = PathBuf::from("src/content/projects/local-rs.md");
/// let content_type = get_content_type(&file, "src/content");
/// assert_eq!(content_type, "projects");
/// ```
///
/// # Behavior
/// - If the file is not under the content directory, returns "page"
/// - If the file is directly in the content directory (no subdirectory), returns "page"
/// - The content directory prefix is stripped case-sensitively
#[rustfmt::skip]
pub(crate) fn get_content_type(file: &Path, content_dir: &str) -> String {
    file.strip_prefix(content_dir)                              // removes src/content
        .ok()                                                   // Convert Result to Option
        .and_then(|rel_path| rel_path.components().next())      // gets the next dir (projects)
        .and_then(|comp| comp.as_os_str().to_str())             // converts  that to &str
        .unwrap_or("page")                                      // or gets "page"
        .to_string()                                            // as  string
}

/// Recursively finds all markdown files in the specified content directory.
///
/// This function traverses the directory tree starting from `content_dir` and
/// collects paths to all files with `.md` or `.markdown` extensions. The search
/// is performed recursively through all subdirectories.
///
/// # Arguments
/// * `content_dir` - The root directory path to search for markdown files
///
/// # Returns
/// A vector of `PathBuf` objects representing the absolute paths to all
/// found markdown files.
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use your_crate::find_markdown_files;
///
/// // Assuming directory structure:
/// // content/
/// //   index.md
/// //   blog/
/// //     post1.md
/// //     post2.markdown
/// let files = find_markdown_files("content");
/// assert!(files.iter().any(|p| p.ends_with("index.md")));
/// assert!(files.iter().any(|p| p.ends_with("post1.md")));
/// assert!(files.iter().any(|p| p.ends_with("post2.markdown")));
/// ```
///
/// # Notes
/// - The search is case-sensitive for file extensions
/// - Only regular files are considered (symlinks and directories are ignored)
/// - The function returns an empty vector if the directory doesn't exist or
///   contains no markdown files
/// - Hidden files and directories (starting with `.`) are included in the search
pub(crate) fn find_markdown_files(content_dir: &str) -> Vec<PathBuf> {
    let mut markdown_files = Vec::new();

    let walkdir = WalkDir::new(content_dir);

    for entry in walkdir.into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if entry.file_type().is_file()
            && let Some(ext) = path.extension()
            && (ext == "md" || ext == "markdown")
        {
            markdown_files.push(path.to_path_buf());
        }
    }

    markdown_files
}

/// Adds a date prefix to a file path in the format: YYYY-MM-DD-filename
///
/// # Arguments
/// * `output_path` - The original output path (e.g., "out/posts/my-post.html")
/// * `date` - The date to use for the prefix
///
/// # Returns
/// A new PathBuf with the date prefix (e.g., "out/posts/2023-05-15-my-post.html")
pub(crate) fn add_date_prefix(output_path: PathBuf, date: &DateTime<FixedOffset>) -> PathBuf {
    // Format the date as YYYY-MM-DD
    let date_str = date.format("%Y-%m-%d").to_string();

    // Get the parent directory and file name separately
    let parent_dir = output_path.parent().unwrap_or_else(|| Path::new(""));
    let file_stem = output_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();

    // Create the new file name with date prefix
    let new_file_name = format!("{}-{}", date_str, file_stem);

    // Combine the parent directory with the new file name and add .html extension
    parent_dir.join(new_file_name).with_extension("html")
}

/// Converts a content file path to its corresponding output HTML path.
///
/// This function transforms a markdown content file path into the path where
/// the generated HTML should be written. It replaces the content directory
/// with the output directory and changes the file extension from `.md` to `.html`.
///
/// # Arguments
/// * `file` - Path to the source markdown content file
/// * `content_dir` - Root directory containing the content files
/// * `output_dir` - Target directory where HTML files should be written
///
/// # Returns
/// A `PathBuf` representing the output HTML file path. If the input file is not
/// under the content directory, returns a path to "error.html" in the output directory.
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use your_crate::get_output_path;
///
/// let input_file = PathBuf::from("src/content/blog/hello-world.md");
/// let output_path = get_output_path(&input_file, "src/content", "dist");
/// assert_eq!(output_path, PathBuf::from("dist/blog/hello-world.html"));
///
/// // File not under content directory falls back to error.html
/// let external_file = PathBuf::from("other/location/file.md");
/// let error_path = get_output_path(&external_file, "src/content", "dist");
/// assert_eq!(error_path, PathBuf::from("dist/error.html"));
/// ```
///
/// # Behavior
/// - Preserves the directory structure relative to the content directory
/// - Changes file extension from `.md`/`.markdown` to `.html`
/// - Returns "error.html" path if the file is not under the content directory
/// - Handles both relative and absolute paths correctly
pub(crate) fn get_output_path(file: &Path, content_dir: &str, output_dir: &str) -> PathBuf {
    file.strip_prefix(content_dir)
        .map(|rel_path| {
            PathBuf::from(output_dir)
                .join(rel_path)
                .with_extension("html")
        })
        .unwrap_or_else(|_| PathBuf::from(output_dir).join("error.html"))
}

/// Retrieves the template path for a specific content type from the configuration.
///
/// This function looks up the configured template for a given content type in the
/// site configuration. If the content type is not configured or doesn't exist,
/// it falls back to the default template.
///
/// # Arguments
/// * `config` - Reference to the site configuration containing content type definitions
/// * `content_type` - The content type to look up (e.g., "blog", "projects", "page")
///
/// # Returns
/// A string containing the template file path for the specified content type.
/// Returns "default.html" if the content type is not found in the configuration.
///
/// # Examples
/// ```
/// use your_crate::{get_content_type_template, Config, ContentTypeConfig};
/// use std::collections::HashMap;
///
/// // Create a test configuration
/// let mut config = Config {
///     content_types: HashMap::from([
///         ("blog".to_string(), ContentTypeConfig {
///             content_template: "blog_post.html".to_string(),
///             index_template: "blog_index.html".to_string(),
///             output_naming: Some("default".to_string()),
///         }),
///         ("projects".to_string(), ContentTypeConfig {
///             content_template: "project.html".to_string(),
///             index_template: "projects_index.html".to_string(),
///             output_naming: Some("default".to_string()),
///         }),
///     ]),
///     // ... other config fields
/// #     title: "Test".to_string(),
/// #     tagline: "Test".to_string(),
/// #     domain: "test.com".to_string(),
/// #     author: "Test".to_string(),
/// #     output_dir: "out".to_string(),
/// #     content_dir: "content".to_string(),
/// #     template_dir: "templates".to_string(),
/// #     static_dir: "static".to_string(),
/// #     site_index_template: "index.html".to_string(),
/// };
///
/// // Get template for configured content type
/// let template = get_content_type_template(&config, "blog");
/// assert_eq!(template, "blog_post.html");
///
/// // Fallback for unknown content type
/// let default_template = get_content_type_template(&config, "unknown");
/// assert_eq!(default_template, "default.html");
/// ```
///
/// # Notes
/// - The returned template path is relative to the template directory
/// - Content type lookup is case-sensitive
/// - The fallback template "default.html" should exist in the template directory
/// - This function only returns the template path; it does not validate if the template file exists
pub(crate) fn get_content_type_template(config: &Config, content_type: &str) -> String {
    config
        .content
        .get(content_type)
        .map(|ct| ct.content_template.as_str())
        .unwrap_or("default.html") // fallback template
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::config::ContentTypeConfig;

    use super::*;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_test_config() -> Config {
        let mut content_types = HashMap::new();
        content_types.insert(
            "projects".to_string(),
            ContentTypeConfig {
                content_template: "project.html".to_string(),
                index_template: "projects_index.html".to_string(),
                output_naming: Some("default".to_string()),
            },
        );
        content_types.insert(
            "blog".to_string(),
            ContentTypeConfig {
                content_template: "blog_post.html".to_string(),
                index_template: "blog_index.html".to_string(),
                output_naming: Some("default".to_string()),
            },
        );

        Config {
            site: crate::config::SiteConfig {
                title: "Test Site".to_string(),
                tagline: "Hello world".to_string(),
                domain: "test.com".to_string(),
                author: "Test Author".to_string(),
                output_dir: "out".to_string(),
                content_dir: "src/content".to_string(),
                template_dir: "templates".to_string(),
                static_dir: "static".to_string(),
                site_index_template: "site_index.html".to_string(),
                syntax_highlighting_enabled: true,
                syntax_highlighting_theme: crate::syntax::DEFAULT_THEME.to_string(),
                root_static: HashMap::new(),
            },
            content: content_types,
            dynamic: HashMap::new(),
        }
    }

    #[test]
    fn test_get_output_path_converts_md_to_html() {
        let input = PathBuf::from("src/content/projects/local-rs.md");
        let result = get_output_path(&input, "src/content", "out");
        let expected = PathBuf::from("out/projects/local-rs.html");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_content_type_extracts_directory() {
        let input = PathBuf::from("src/content/projects/local-rs.md");
        let result = get_content_type(&input, "src/content");

        assert_eq!(result, "projects");
    }

    #[test]
    fn test_get_content_type_falls_back_to_page() {
        let input = PathBuf::from("different/path/file.md");
        let result = get_content_type(&input, "src/content");

        assert_eq!(result, "page");
    }

    #[test]
    fn test_get_content_type_template_returns_configured_template() {
        let config = create_test_config();
        let result = get_content_type_template(&config, "projects");

        assert_eq!(result, "project.html");
    }

    #[test]
    fn test_get_content_type_template_falls_back_to_default() {
        let config = create_test_config();
        let result = get_content_type_template(&config, "unknown");

        assert_eq!(result, "default.html");
    }

    #[test]
    fn test_get_output_path() {
        let input = PathBuf::from("src/content/projects/local-rs.md");
        let result = get_output_path(&input, "src/content", "out");
        assert_eq!(result, PathBuf::from("out/projects/local-rs.html"));

        // Test with different structure
        let input2 = PathBuf::from("content/blog/post.md");
        let result2 = get_output_path(&input2, "content", "dist");
        assert_eq!(result2, PathBuf::from("dist/blog/post.html"));
    }

    #[test]
    fn test_get_output_path_error_case() {
        // Test file not under content directory
        let input = PathBuf::from("some/other/dir/file.md");
        let result = get_output_path(&input, "src/content", "out");
        assert_eq!(result, PathBuf::from("out/error.html"));
    }

    #[test]
    fn test_get_content_type_nested_directory() {
        // Test file in nested directory structure
        let input = PathBuf::from("src/content/blog/tech/rust/post.md");
        let result = get_content_type(&input, "src/content");
        assert_eq!(result, "blog");
    }

    #[test]
    fn test_find_markdown_files() {
        // Create temporary directory structure
        let temp_dir = tempdir().unwrap();
        let content_dir = temp_dir.path();

        // Create test files and directories
        fs::create_dir(content_dir.join("blog")).unwrap();
        fs::create_dir(content_dir.join("projects")).unwrap();

        // Create markdown files
        File::create(content_dir.join("index.md"))
            .unwrap()
            .write_all(b"# Index")
            .unwrap();
        File::create(content_dir.join("blog/post1.md"))
            .unwrap()
            .write_all(b"# Post 1")
            .unwrap();
        File::create(content_dir.join("blog/post2.markdown"))
            .unwrap()
            .write_all(b"# Post 2")
            .unwrap();
        File::create(content_dir.join("projects/readme.md"))
            .unwrap()
            .write_all(b"# Project")
            .unwrap();

        // Create non-markdown files (should be ignored)
        File::create(content_dir.join("style.css"))
            .unwrap()
            .write_all(b"body {}")
            .unwrap();
        File::create(content_dir.join("blog/script.js"))
            .unwrap()
            .write_all(b"console.log()")
            .unwrap();

        let result = find_markdown_files(content_dir.to_str().unwrap());

        assert_eq!(result.len(), 4);
        assert!(result.iter().any(|p| p.ends_with("index.md")));
        assert!(result.iter().any(|p| p.ends_with("post1.md")));
        assert!(result.iter().any(|p| p.ends_with("post2.markdown")));
        assert!(result.iter().any(|p| p.ends_with("readme.md")));

        // Verify non-markdown files are not included
        assert!(!result.iter().any(|p| p.ends_with("style.css")));
        assert!(!result.iter().any(|p| p.ends_with("script.js")));
    }

    #[test]
    fn test_find_markdown_files_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let content_dir = temp_dir.path();

        let result = find_markdown_files(content_dir.to_str().unwrap());
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_markdown_files_no_markdown() {
        let temp_dir = tempdir().unwrap();
        let content_dir = temp_dir.path();

        File::create(content_dir.join("index.txt"))
            .unwrap()
            .write_all(b"text")
            .unwrap();
        File::create(content_dir.join("style.css"))
            .unwrap()
            .write_all(b"css")
            .unwrap();

        let result = find_markdown_files(content_dir.to_str().unwrap());
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_add_date_prefix() {
        use chrono::TimeZone;

        let date = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2023, 5, 15, 0, 0, 0)
            .unwrap();

        let input = PathBuf::from("out/posts/my-post.html");
        let result = add_date_prefix(input, &date);

        assert_eq!(result, PathBuf::from("out/posts/2023-05-15-my-post.html"));
    }

    #[test]
    fn test_add_date_prefix_nested_path() {
        use chrono::TimeZone;

        let date = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2024, 12, 31, 0, 0, 0)
            .unwrap();

        let input = PathBuf::from("dist/blog/tech/rust/article.html");
        let result = add_date_prefix(input, &date);

        assert_eq!(
            result,
            PathBuf::from("dist/blog/tech/rust/2024-12-31-article.html")
        );
    }

    #[test]
    fn test_add_date_prefix_root_file() {
        use chrono::TimeZone;

        let date = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2023, 1, 1, 0, 0, 0)
            .unwrap();

        let input = PathBuf::from("output.html");
        let result = add_date_prefix(input, &date);

        assert_eq!(result, PathBuf::from("2023-01-01-output.html"));
    }
}
