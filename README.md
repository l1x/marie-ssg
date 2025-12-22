# Marie SSG

A static site generator that does one thing well: convert markdown files into HTML pages.

Marie follows a pipeline architecture. You write content in markdown with TOML metadata. Marie loads everything in parallel, renders through Jinja-style templates, and writes the output. Just files in, files out.

## Quick Start

```bash

# Build with a custom config
marie-ssg build -c mysite.toml

# Watch for changes and rebuild (macOS only)
marie-ssg watch -c mysite.toml

# Show version
marie-ssg -V
```

## Project Structure

Marie expects this layout:

```
your-site/
├── site.toml           # Configuration
├── content/            # Markdown files + metadata
│   ├── posts/
│   │   ├── hello.md
│   │   └── hello.meta.toml
│   └── pages/
│       ├── about.md
│       └── about.meta.toml
├── templates/          # Jinja-style HTML templates
│   ├── post.html
│   ├── posts_index.html
│   ├── page.html
│   └── site_index.html
├── static/             # CSS, images, fonts (copied as-is)
└── output/             # Generated site
```

**Content types** come from directory names. Files in `content/posts/` use the "posts" content type. Files in `content/pages/` use "pages". Each type can have its own template and index page.

## Configuration

Create a `site.toml` in your project root:

```toml
[site]
# Mandatory fields - metadata
title = "My Site"
tagline = "A blog about things"
domain = "example.com"
author = "Your Name"
# Mandatory fields - directories & files
output_dir = "output"
content_dir = "content"
template_dir = "templates"
static_dir = "static"
site_index_template = "site_index.html"

# Content type configuration
[content.posts]
index_template = "posts_index.html"
content_template = "post.html"
output_naming = "date"              # Prefix output files with date

[content.pages]
index_template = "pages_index.html"
content_template = "page.html"

# Files copied to output root
[site.root_static]
"favicon.ico" = "favicon.ico"
"robots.txt" = "robots.txt"

# Custom variables available in templates
[dynamic]
github_url = "https://github.com/you"
```

## Templates

Marie uses [minijinja](https://github.com/mitsuhiko/minijinja) for templating. Templates receive the full config plus content-specific data.

```html
<!-- templates/post.html -->
<html>
  <head>
    <title>{{ title }} | {{ config.site.title }}</title>
  </head>
  <body>
    <h1>{{ title }}</h1>
    <time>{{ date }}</time>
    {{ content }}
  </body>
</html>
```

**Template naming convention:**

- `post.html` renders individual posts
- `posts_index.html` renders the posts listing
- `site_index.html` renders the homepage

Include shared partials with `{% include "header.html" %}`.

## Development

### Prerequisites

This project uses [mise](https://mise.jdx.dev/) for task running and Rust toolchain management.

```bash
# Install mise (if needed)
brew install mise

# Install project dependencies
mise install
```

### Build Tasks

```bash
mise run lint                  # Clippy with warnings as errors
mise run tests                 # Run all tests
mise run build-dev             # Debug build
mise run build-prod            # Release build (optimized)
```

### Running Tests

Marie has 86% test coverage with unit and integration tests.

```bash
cargo test                           # All tests
cargo test --test integration_test   # Integration tests only
cargo tarpaulin --out Stdout         # Coverage report
```

### Issue Tracking

We track work with [beads](https://github.com/beads-project/beads), a git-native issue tracker.

```bash
mise run show-issues           # List open issues
mise run show-ready            # Work with no blockers
mise run show-blocked          # Blocked issues
mise run show-issue-stats      # Project statistics

# Or use bd directly
bd create --title="Fix bug" --type=bug --priority=2
bd update mar-xxx --status=in_progress
bd close mar-xxx
```

## How It Works

1. **Config** — Parses `site.toml` into typed structs
2. **Discovery** — Finds all `.md` files under `content/`
3. **Parallel loading** — Uses Rayon to load and convert markdown concurrently
4. **Template caching** — Initializes templates once, reuses for all renders
5. **Output** — Writes HTML files and copies static assets

The key data structure is `LoadedContent`: it holds the markdown, converted HTML, content type, and output path. Content loads once and feeds both individual pages and index generation.

## Code Layout

```
src/
├── main.rs       # CLI and orchestration
├── config.rs     # Configuration types and loading
├── content.rs    # Markdown processing and metadata
├── template.rs   # Template rendering with minijinja
├── output.rs     # File writing and static copying
├── utils.rs      # Path handling and content type detection
└── error.rs      # Error types with thiserror
```

## Version History

### v0.5.0 (2025-12-20)

- Added `watch` subcommand for automatic rebuilds (macOS)
- Moved build to subcommand structure (`marie-ssg build`)
- Added `-V` / `--version` flag

### v0.4.0 (2025-12-15)

- Reached 86% test coverage with integration tests
- Added HTML DOM validation in tests using `scraper`
- Refactored template rendering for testability

### v0.3.0 (2025-10-14)

- Added template caching with `OnceLock`
- Introduced nested config structure
- Added `[dynamic]` section for custom template variables

### v0.2.0 (2025-10-10)

- Parallel content loading with Rayon
- Single-pass content loading (66% fewer I/O operations)
- Build timing instrumentation

### v0.1.0

- Initial release with core SSG functionality
