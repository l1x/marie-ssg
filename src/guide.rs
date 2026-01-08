// src/guide.rs

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prints the Marie SSG guide to stdout
pub(crate) fn print_guide() {
    print!(
        r####"# Marie SSG Guide

Marie is a static site generator that converts markdown files with TOML metadata into HTML pages.

## Quick Start

```bash
marie-ssg build              # Build the site
marie-ssg build -c prod.toml # Build with custom config
marie-ssg watch              # Watch and rebuild on changes (macOS)
marie-ssg guide              # Show this guide
```

## Project Structure

```
my-site/
├── site.toml           # Site configuration
├── content/            # Markdown content files
│   ├── blog/
│   │   ├── hello.md
│   │   └── hello.meta.toml
│   └── pages/
│       ├── about.md
│       └── about.meta.toml
├── templates/          # Jinja-style templates
│   ├── base.html
│   ├── post.html
│   └── blog_index.html
├── static/             # Static assets (CSS, images, fonts)
│   ├── css/
│   └── images/
└── output/             # Generated site (created by build)
```

## Configuration (site.toml)

```toml
[site]
title = "My Website"
tagline = "A personal blog"
domain = "example.com"
author = "Your Name"
content_dir = "content"
output_dir = "output"
template_dir = "templates"
static_dir = "static"
site_index_template = "index.html"

# Optional features (defaults shown)
syntax_highlighting_enabled = true   # Enable code block highlighting
syntax_highlighting_theme = "github_dark"
sitemap_enabled = true               # Generate sitemap.xml
rss_enabled = true                   # Generate feed.xml
allow_dangerous_html = false         # Allow raw HTML in markdown (for <figure>, inline SVGs, etc.)
header_uri_fragment = false          # Add anchor links to headers for URL fragment navigation
clean_urls = false                   # Output as slug/index.html for SEO-friendly URLs (/blog/post/ instead of /blog/post.html)
asset_hashing_enabled = false        # Hash CSS/JS files for cache busting (style.css → style.a1b2c3d4.css)

# Files copied to output root (e.g., favicon)
[site.root_static]
"favicon.ico" = "favicon.ico"
"robots.txt" = "robots.txt"

# Content type configurations
[content.blog]
index_template = "blog_index.html"
content_template = "post.html"
output_naming = "date"      # Prefix output with date (YYYY-MM-DD-slug.html)
rss_include = true          # Include in RSS feed (default: true)

[content.pages]
index_template = "pages_index.html"
content_template = "page.html"
rss_include = false         # Exclude from RSS feed

# Custom variables for templates
[dynamic]
github_url = "https://github.com/user"
twitter = "@username"
```

## Content Files

Each markdown file needs a companion `.meta.toml` file:

**content/blog/hello.md:**
```markdown
# Hello World

This is my first post.

## Context

This section becomes the excerpt for RSS feeds and index pages.

## Main Content

The rest of your article...
```

**content/blog/hello.meta.toml:**
```toml
title = "Hello World"
date = "2024-01-15T10:00:00+00:00"  # RFC 3339 format
author = "Your Name"
tags = ["intro", "blog"]
template = "custom.html"             # Optional: override default template
cover = "/images/hello-cover.jpg"    # Optional: cover image for social sharing

[extra]
reading_time = "5 min"               # Custom fields go in [extra] section
category = "tutorials"
```

### Metadata Fields

| Field    | Required | Description                                              |
|----------|----------|----------------------------------------------------------|
| title    | Yes      | Article title                                            |
| date     | Yes      | Publication date (RFC 3339: `YYYY-MM-DDTHH:MM:SS+00:00`) |
| author   | Yes      | Author name                                              |
| tags     | Yes      | Array of tags (can be empty: `[]`)                       |
| template | No       | Override the content type's default template             |
| cover    | No       | Cover image URL/path for social sharing                  |
| [extra]  | No       | Custom key-value fields (access via `meta.extra.key`)    |

## Templates (Jinja2/Minijinja)

Templates use Jinja2 syntax via the Minijinja library.

### Available Context

**In content templates (`post.html`):**
- `content` - Rendered HTML content
- `meta.title`, `meta.date`, `meta.author`, `meta.tags`
- `config.site.title`, `config.site.author`, etc.
- `config.dynamic.github_url`, etc.

**In index templates (`blog_index.html`):**
- `contents` - List of ContentItem for this content type
- `all_content` - List of all ContentItem across all types
- `config` - Full site configuration

### ContentItem Properties

```jinja
{{% for item in contents %}}
  <h2>{{{{ item.meta.title }}}}</h2>
  <time>{{{{ item.formatted_date }}}}</time>
  <p>{{{{ item.excerpt | safe }}}}</p>
  <a href="/{{{{ item.filename | url }}}}">Read more</a>
{{% endfor %}}
```

| Property              | Description                                        |
|-----------------------|----------------------------------------------------|
| `item.html`           | Full rendered HTML content                         |
| `item.meta.title`     | Article title                                      |
| `item.meta.date`      | Date object                                        |
| `item.meta.author`    | Author name                                        |
| `item.meta.tags`      | List of tags                                       |
| `item.meta.cover`     | Cover image URL/path (if set)                      |
| `item.meta.extra.*`   | Custom fields (e.g., `item.meta.extra.reading_time`) |
| `item.formatted_date` | Human-readable date (e.g., "January 15, 2024")     |
| `item.filename`       | Output path (e.g., `blog/hello/` with clean_urls)  |
| `item.content_type`   | Content type (e.g., "blog")                        |
| `item.excerpt`        | HTML excerpt from "## Context" section             |

### Filters

- `| safe` - Render HTML without escaping
- `| url` - URL-encode for href attributes
- `| datetimeformat("%Y-%m-%d")` - Format dates
- `| asset_hash` - Resolve asset path to hashed version (requires `asset_hashing_enabled = true`)

### Template Example

```html
{{% extends "base.html" %}}

{{% block content %}}
<article>
  <h1>{{{{ meta.title }}}}</h1>
  <time>{{{{ meta.date | datetimeformat("%B %d, %Y") }}}}</time>
  <div class="content">{{{{ content | safe }}}}</div>
</article>
{{% endblock %}}
```

## Features

### Syntax Highlighting

Code blocks with language hints are highlighted automatically.

Supported languages: bash, css, html, javascript, json, python, rust, toml, typescript, yaml

Themes: `github_dark` (default), `monokai`, and others from Autumnus.

### Sitemap Generation

Automatically generates `sitemap.xml` with all pages when `sitemap_enabled = true`.

### RSS Feed Generation

Generates `feed.xml` with RSS 2.0 format when `rss_enabled = true`.
- Control per content type with `rss_include = true/false`
- Uses "## Context" section as excerpt

### Header Anchor Links

When `header_uri_fragment = true`, headers (h1-h6) get anchor links for URL fragment navigation.

**Before:** `<h2>My Section</h2>`
**After:** `<h2 id="my-section"><a href="#my-section">My Section</a></h2>`

This enables:
- Direct linking to sections: `https://example.com/page#my-section`
- Clickable headers for easy link copying

### Clean URLs

When `clean_urls = true`, content is output with SEO-friendly directory structure:

**Before (clean_urls = false):**
- `content/blog/2024-01-15-hello.md` → `output/blog/2024-01-15-hello.html`
- URL: `/blog/2024-01-15-hello.html`

**After (clean_urls = true):**
- `content/blog/2024-01-15-hello.md` → `output/blog/hello/index.html`
- URL: `/blog/hello/`

Benefits:
- Cleaner, more shareable URLs
- Date prefix stripped from URL (kept in metadata for sorting)
- Trailing slash convention (modern SSG standard)
- Sitemap and RSS URLs automatically updated

### Asset Hashing (Cache Busting)

When `asset_hashing_enabled = true`, CSS and JS files get content-based hashes in their filenames for cache busting.

**How it works:**
1. Marie computes an 8-character BLAKE3 hash from each CSS/JS file's content
2. Files are copied with hashed names: `style.css` → `style.a1b2c3d4.css`
3. A manifest maps original paths to hashed paths
4. Old hashed files are cleaned up on rebuild

**Usage in templates:**

```html
<!-- Before (hardcoded, cache problems) -->
<link rel="stylesheet" href="/static/css/style.css" />
<script src="/static/js/app.js"></script>

<!-- After (using asset_hash filter) -->
<link rel="stylesheet" href="{{{{ "static/css/style.css" | asset_hash }}}}" />
<script src="{{{{ "static/js/app.js" | asset_hash }}}}"></script>
```

**Output:**
```html
<link rel="stylesheet" href="/static/css/style.a1b2c3d4.css" />
<script src="/static/js/app.b5c6d7e8.js"></script>
```

**Benefits:**
- Set long cache headers (e.g., `Cache-Control: max-age=31536000`)
- Browsers automatically fetch new versions when content changes
- No manual cache busting (no `?v=123` query strings needed)
- Only CSS and JS files are hashed; images and fonts are unchanged

**Note:** If `asset_hashing_enabled = false` (default), the `asset_hash` filter returns the original path unchanged.

### Watch Mode (macOS)

Automatically rebuilds when files change:
```bash
marie-ssg watch
```

## Output

After build, your site is in the output directory:

```
output/
├── index.html          # Site homepage
├── sitemap.xml         # Sitemap (if enabled)
├── feed.xml            # RSS feed (if enabled)
├── favicon.ico         # Root static files
├── static/             # Copied static assets
├── blog/
│   ├── index.html      # Blog index
│   └── 2024-01-15-hello.html
└── pages/
    ├── index.html
    └── about.html
```

## Tips

1. **Date prefix**: Use `output_naming = "date"` to prefix files with publication date
2. **Excerpts**: Add a "## Context" section for RSS/index excerpts
3. **Custom templates**: Override per-article with `template` in metadata
4. **Dynamic vars**: Add custom variables in `[dynamic]` for use in templates

---
Generated by marie-ssg {version}
"####,
        version = VERSION
    );
}
