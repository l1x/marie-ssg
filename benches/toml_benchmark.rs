use criterion::{Criterion, black_box, criterion_group, criterion_main};
use serde::Deserialize;
use std::collections::HashMap;

// Minimal config struct for parsing
#[derive(Debug, Deserialize)]
struct Config {
    site: SiteConfig,
    #[serde(default)]
    content: HashMap<String, ContentTypeConfig>,
    #[serde(default)]
    dynamic: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize)]
struct SiteConfig {
    title: String,
    tagline: String,
    domain: String,
    author: String,
    content_dir: String,
    output_dir: String,
    template_dir: String,
    static_dir: String,
    site_index_template: String,
    #[serde(default)]
    root_static: HashMap<String, String>,
    #[serde(default = "default_true")]
    syntax_highlighting_enabled: bool,
    #[serde(default = "default_theme")]
    syntax_highlighting_theme: String,
    #[serde(default = "default_true")]
    sitemap_enabled: bool,
}

#[derive(Debug, Deserialize)]
struct ContentTypeConfig {
    index_template: String,
    content_template: String,
    #[serde(default)]
    output_naming: Option<String>,
}

fn default_true() -> bool {
    true
}
fn default_theme() -> String {
    "github_dark".to_string()
}

// Minimal config - just required fields
const MINIMAL_TOML: &str = r#"
[site]
title = "Test"
tagline = "A test"
domain = "test.com"
author = "Tester"
content_dir = "content"
output_dir = "output"
template_dir = "templates"
static_dir = "static"
site_index_template = "index.html"
"#;

// Typical config - like a real site
const TYPICAL_TOML: &str = r#"
[site]
title = "My Awesome Site"
tagline = "Built with Marie SSG"
domain = "example.com"
author = "Jane Doe"
content_dir = "content"
template_dir = "templates"
static_dir = "static"
output_dir = "dist"
site_index_template = "index.html"
syntax_highlighting_enabled = true
syntax_highlighting_theme = "base16-ocean.dark"
sitemap_enabled = true

[site.root_static]
"favicon.ico" = "favicon.ico"
"robots.txt" = "robots.txt"

[content.blog]
index_template = "blog_index.html"
content_template = "post.html"
output_naming = "date"

[content.pages]
index_template = "page_index.html"
content_template = "page.html"

[content.projects]
index_template = "projects_index.html"
content_template = "project.html"

[dynamic]
github_url = "https://github.com/janedoe"
twitter_handle = "@janedoe"
deployment_env = "production"
"#;

// Large config - many content types and dynamic values
const LARGE_TOML: &str = r#"
[site]
title = "Large Enterprise Site"
tagline = "Built with Marie SSG - Enterprise Edition"
domain = "enterprise.example.com"
author = "Enterprise Team"
content_dir = "content"
template_dir = "templates"
static_dir = "static"
output_dir = "dist"
site_index_template = "index.html"
syntax_highlighting_enabled = true
syntax_highlighting_theme = "github_dark"
sitemap_enabled = true

[site.root_static]
"favicon.ico" = "favicon.ico"
"robots.txt" = "robots.txt"
"manifest.json" = "manifest.json"
"browserconfig.xml" = "browserconfig.xml"
"apple-touch-icon.png" = "icons/apple-touch-icon.png"

[content.blog]
index_template = "blog_index.html"
content_template = "post.html"
output_naming = "date"

[content.pages]
index_template = "page_index.html"
content_template = "page.html"

[content.projects]
index_template = "projects_index.html"
content_template = "project.html"

[content.docs]
index_template = "docs_index.html"
content_template = "doc.html"

[content.tutorials]
index_template = "tutorials_index.html"
content_template = "tutorial.html"
output_naming = "date"

[content.news]
index_template = "news_index.html"
content_template = "news_item.html"
output_naming = "date"

[content.team]
index_template = "team_index.html"
content_template = "team_member.html"

[content.products]
index_template = "products_index.html"
content_template = "product.html"

[dynamic]
github_url = "https://github.com/enterprise"
twitter_handle = "@enterprise"
linkedin_url = "https://linkedin.com/company/enterprise"
facebook_url = "https://facebook.com/enterprise"
instagram_handle = "@enterprise_official"
youtube_channel = "UCenterprise123"
deployment_env = "production"
analytics_id = "UA-12345678-1"
gtm_id = "GTM-ABCDEF"
sentry_dsn = "https://abc123@sentry.io/123456"
api_base_url = "https://api.enterprise.example.com"
cdn_url = "https://cdn.enterprise.example.com"
support_email = "support@enterprise.example.com"
copyright_year = "2024"
company_name = "Enterprise Corp"
"#;

fn bench_basic_toml_minimal(c: &mut Criterion) {
    c.bench_function("basic-toml/minimal", |b| {
        b.iter(|| {
            let _: Config = basic_toml::from_str(black_box(MINIMAL_TOML)).unwrap();
        })
    });
}

fn bench_toml_minimal(c: &mut Criterion) {
    c.bench_function("toml/minimal", |b| {
        b.iter(|| {
            let _: Config = toml::from_str(black_box(MINIMAL_TOML)).unwrap();
        })
    });
}

fn bench_basic_toml_typical(c: &mut Criterion) {
    c.bench_function("basic-toml/typical", |b| {
        b.iter(|| {
            let _: Config = basic_toml::from_str(black_box(TYPICAL_TOML)).unwrap();
        })
    });
}

fn bench_toml_typical(c: &mut Criterion) {
    c.bench_function("toml/typical", |b| {
        b.iter(|| {
            let _: Config = toml::from_str(black_box(TYPICAL_TOML)).unwrap();
        })
    });
}

fn bench_basic_toml_large(c: &mut Criterion) {
    c.bench_function("basic-toml/large", |b| {
        b.iter(|| {
            let _: Config = basic_toml::from_str(black_box(LARGE_TOML)).unwrap();
        })
    });
}

fn bench_toml_large(c: &mut Criterion) {
    c.bench_function("toml/large", |b| {
        b.iter(|| {
            let _: Config = toml::from_str(black_box(LARGE_TOML)).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_basic_toml_minimal,
    bench_toml_minimal,
    bench_basic_toml_typical,
    bench_toml_typical,
    bench_basic_toml_large,
    bench_toml_large,
);
criterion_main!(benches);
