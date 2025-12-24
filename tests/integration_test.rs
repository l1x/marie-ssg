// Integration tests for Marie SSG
//
// These tests run the CLI binary against test fixtures and validate:
// - Output file generation
// - HTML structure and content via DOM parsing
// - Static file copying
// - Multiple content types
// - Template rendering
// - Date-based sorting

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Helper to create a temporary working directory with test fixtures
fn setup_test_site() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let fixture_src = PathBuf::from("tests/fixtures/simple_site");

    // Copy fixture to temp directory
    copy_dir_recursive(&fixture_src, temp_dir.path()).unwrap();

    temp_dir
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if path.is_dir() {
            copy_dir_recursive(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
    }

    Ok(())
}

/// Helper to run marie-ssg CLI with a config file
/// Sets the current directory to the site directory for relative paths to work
fn run_ssg(site_dir: &Path) -> assert_cmd::assert::Assert {
    let config_path = site_dir.join("site.toml");

    cargo_bin_cmd!("marie-ssg")
        .current_dir(site_dir)
        .arg("build")
        .arg("-c")
        .arg(config_path.file_name().unwrap())
        .assert()
        .success()
}

/// Helper to parse HTML and select elements
fn parse_html_file(path: &Path) -> Html {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read HTML file: {}", path.display()));
    Html::parse_document(&content)
}

/// Helper to select a single element and get its text
fn select_text(html: &Html, selector: &str) -> String {
    let sel = Selector::parse(selector).unwrap();
    html.select(&sel)
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Helper to count matching elements
fn count_elements(html: &Html, selector: &str) -> usize {
    let sel = Selector::parse(selector).unwrap();
    html.select(&sel).count()
}

#[test]
fn test_cli_runs_successfully() {
    let temp_site = setup_test_site();

    run_ssg(temp_site.path())
        .success()
        .stdout(predicate::str::contains("Process completed successfully"));
}

#[test]
fn test_generates_all_expected_files() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    // Check main index
    assert!(
        output_dir.join("index.html").exists(),
        "Site index should exist"
    );

    // Check content type indexes
    assert!(
        output_dir.join("blog/index.html").exists(),
        "Blog index should exist"
    );
    assert!(
        output_dir.join("pages/index.html").exists(),
        "Pages index should exist"
    );

    // Check individual blog posts (date-prefixed)
    let blog_dir = output_dir.join("blog");
    assert!(blog_dir.exists(), "Blog directory should exist");

    // Find generated blog posts (they have date prefixes)
    let blog_entries: Vec<_> = fs::read_dir(&blog_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "html"))
        .collect();

    assert!(blog_entries.len() >= 2, "Should have at least 2 blog posts");

    // Check pages
    assert!(
        output_dir.join("pages/about.html").exists(),
        "About page should exist"
    );

    // Check static files are copied to static/ subdirectory
    assert!(
        output_dir.join("static/style.css").exists(),
        "CSS file should be copied"
    );
    assert!(
        output_dir.join("favicon.ico").exists(),
        "Favicon should be copied to root"
    );
    assert!(
        output_dir.join("robots.txt").exists(),
        "Robots.txt should be copied to root"
    );
}

#[test]
fn test_site_index_content() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    let index_html = parse_html_file(&output_dir.join("index.html"));

    // Validate site metadata
    let title = select_text(&index_html, "h1.site-title");
    assert_eq!(title, "Test Blog");

    let tagline = select_text(&index_html, "p.tagline");
    assert_eq!(tagline, "A test site for integration testing");

    // Check dynamic variables in footer
    let footer = select_text(&index_html, "footer");
    assert!(
        footer.contains("@testuser"),
        "Should render dynamic twitter handle"
    );

    // Check content count (2 blog posts + 1 page = 3 items)
    let welcome_section = select_text(&index_html, "section.welcome");
    assert!(
        welcome_section.contains("3"),
        "Should show 3 total content items"
    );

    // Check recent content section exists
    let content_items = count_elements(&index_html, ".content-item");
    assert_eq!(content_items, 3, "Should display all 3 content items");
}

#[test]
fn test_blog_index_sorting() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    let blog_index = parse_html_file(&output_dir.join("blog/index.html"));

    // Count posts
    let post_count = count_elements(&blog_index, ".post-summary");
    assert_eq!(post_count, 2, "Should have 2 blog posts");

    // Check sorting (newest first)
    let sel = Selector::parse(".post-summary h3").unwrap();
    let titles: Vec<String> = blog_index
        .select(&sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .collect();

    assert_eq!(titles.len(), 2);
    assert_eq!(titles[0], "Second Blog Post", "Newer post should be first");
    assert_eq!(titles[1], "My First Post", "Older post should be second");

    // Validate post count in footer
    let footer = select_text(&blog_index, "footer");
    assert!(
        footer.contains("2 blog posts"),
        "Footer should show post count"
    );
}

#[test]
fn test_blog_post_rendering() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    // Find one of the blog posts (they have date prefixes)
    let blog_dir = output_dir.join("blog");
    let blog_post = fs::read_dir(&blog_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .find(|e| {
            e.path().is_file()
                && e.path().extension().map_or(false, |ext| ext == "html")
                && e.file_name().to_string_lossy().contains("first-post")
        })
        .expect("Should find first-post HTML file");

    let post_html = parse_html_file(&blog_post.path());

    // Check title
    let title = select_text(&post_html, "h1.post-title");
    assert_eq!(title, "My First Post");

    // Check author
    let author = select_text(&post_html, ".author");
    assert!(author.contains("Test Author"));

    // Check formatted date via datetimeformat filter
    let date = select_text(&post_html, ".date");
    assert_eq!(date, "January 15 2024 10:00:00");

    // Check tags
    let tags = count_elements(&post_html, ".tags .tag");
    assert_eq!(tags, 3, "Should have 3 tags");

    // Check content rendered
    let content = select_text(&post_html, ".content");
    assert!(
        content.contains("My First Post"),
        "Content should be rendered"
    );
    assert!(
        content.contains("Markdown support"),
        "List items should be rendered"
    );

    // Check dynamic config in footer
    let footer = select_text(&post_html, "footer");
    assert!(
        footer.contains("github.com/testuser"),
        "Should render GitHub URL from dynamic config"
    );
}

#[test]
fn test_page_rendering() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    let about_html = parse_html_file(&output_dir.join("pages/about.html"));

    // Check page title
    let title = select_text(&about_html, "h1.page-title");
    assert_eq!(title, "About");

    // Check content
    let content = select_text(&about_html, ".content");
    assert!(
        content.contains("About This Site"),
        "Page content should be rendered"
    );
    assert!(
        content.contains("Marie SSG"),
        "Should contain expected text"
    );

    // Check domain in footer
    let footer = select_text(&about_html, "footer");
    assert!(
        footer.contains("test.example.com"),
        "Should show domain from config"
    );
}

#[test]
fn test_static_files_copied() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    // Check CSS file in static subdirectory
    let css_content = fs::read_to_string(output_dir.join("static/style.css")).unwrap();
    assert!(
        css_content.contains("font-family"),
        "CSS should be copied correctly"
    );
    assert!(
        css_content.contains(".site-title"),
        "CSS should contain expected styles"
    );

    // Check robots.txt
    let robots_content = fs::read_to_string(output_dir.join("robots.txt")).unwrap();
    assert!(
        robots_content.contains("User-agent"),
        "Robots.txt should be copied"
    );

    // Check favicon exists
    assert!(
        output_dir.join("favicon.ico").exists(),
        "Favicon should be copied to root"
    );
}

#[test]
fn test_excerpt_in_blog_index() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    let blog_index = parse_html_file(&output_dir.join("blog/index.html"));

    // Check excerpts are present
    let excerpts = count_elements(&blog_index, ".excerpt");
    assert_eq!(excerpts, 2, "Should have excerpts for both posts");

    // Check excerpt content
    let sel = Selector::parse(".excerpt").unwrap();
    let excerpt_texts: Vec<String> = blog_index
        .select(&sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .collect();

    // First post excerpt (after sorting - this is the second post)
    assert!(
        excerpt_texts[0].contains("Testing date sorting"),
        "Should extract excerpt from Context section"
    );
}

#[test]
fn test_invalid_config_fails_gracefully() {
    let temp_dir = TempDir::new().unwrap();
    let invalid_config = temp_dir.path().join("invalid.toml");

    // Create invalid config
    fs::write(&invalid_config, "this is not valid toml [[[").unwrap();

    cargo_bin_cmd!("marie-ssg")
        .arg("build")
        .arg("-c")
        .arg(&invalid_config)
        .assert()
        .failure();
}

#[test]
fn test_html_structure_valid() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    let index_html = parse_html_file(&output_dir.join("index.html"));

    // Validate HTML structure
    assert_eq!(
        count_elements(&index_html, "html"),
        1,
        "Should have one html element"
    );
    assert_eq!(
        count_elements(&index_html, "head"),
        1,
        "Should have one head element"
    );
    assert_eq!(
        count_elements(&index_html, "body"),
        1,
        "Should have one body element"
    );
    assert_eq!(
        count_elements(&index_html, "title"),
        1,
        "Should have one title element"
    );

    // Check title content
    let title = select_text(&index_html, "title");
    assert!(
        title.contains("Test Blog"),
        "Title should contain site title"
    );
    assert!(
        title.contains("A test site"),
        "Title should contain tagline"
    );
}

#[test]
fn test_sitemap_generated() {
    let temp_site = setup_test_site();
    let output_dir = temp_site.path().join("output");

    run_ssg(temp_site.path()).success();

    // Check sitemap.xml exists
    let sitemap_path = output_dir.join("sitemap.xml");
    assert!(sitemap_path.exists(), "sitemap.xml should be generated");

    // Read and validate sitemap content
    let sitemap_content = fs::read_to_string(&sitemap_path).unwrap();

    // Check XML declaration and structure
    assert!(
        sitemap_content.contains(r#"<?xml version="1.0" encoding="UTF-8"?>"#),
        "Should have XML declaration"
    );
    assert!(
        sitemap_content.contains(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#),
        "Should have urlset with sitemap namespace"
    );
    assert!(
        sitemap_content.contains("</urlset>"),
        "Should have closing urlset tag"
    );

    // Check homepage URL
    assert!(
        sitemap_content.contains("<loc>https://test.example.com/</loc>"),
        "Should include homepage URL"
    );

    // Check content type index pages
    assert!(
        sitemap_content.contains("<loc>https://test.example.com/blog/</loc>"),
        "Should include blog index URL"
    );
    assert!(
        sitemap_content.contains("<loc>https://test.example.com/pages/</loc>"),
        "Should include pages index URL"
    );

    // Check individual content pages exist
    assert!(
        sitemap_content.contains("blog/") && sitemap_content.contains(".html"),
        "Should include blog post URLs"
    );
    assert!(
        sitemap_content.contains("<loc>https://test.example.com/pages/about.html</loc>"),
        "Should include about page URL"
    );

    // Check lastmod dates are present
    assert!(
        sitemap_content.contains("<lastmod>"),
        "Should include lastmod dates"
    );
    assert!(
        sitemap_content.contains("2024-01-15") || sitemap_content.contains("2024-02-20"),
        "Should have content dates"
    );
}
