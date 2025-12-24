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

## Syntax Highlighting

Marie includes built-in syntax highlighting for code blocks using the [Autumnus](https://crates.io/crates/autumnus) library. To enable syntax highlighting, add these fields to your `[site]` configuration:

```toml
[site]
# ... other fields ...
syntax_highlighting_enabled = true
syntax_highlighting_theme = "github_dark"
```

### Supported Languages

Marie supports syntax highlighting for these languages:

[Autumnus language support](https://github.com/leandrocp/autumnus?tab=readme-ov-file#selective-language-support)

### Themes

The default theme is `github_dark`. Autumnus supports many themes including:

- `github_dark`, `github_light`
- `monokai`
- `solarized_dark`, `solarized_light`
- `dracula`
- And many more...

### Usage in Markdown

Use standard markdown code blocks with language identifiers:

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```

```python
def hello():
    print("Hello, world!")
```
````

The code blocks will be automatically highlighted in the generated HTML.

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
mise run unit-tests            # Run unit tests only
mise run integration-tests     # Run integration tests only
mise run build-dev             # Debug build
mise run build-prod            # Release build (optimized)
```

### Running Tests

Marie has 86% test coverage with unit and integration tests.

You can use the mise tasks `unit-tests` and `integration-tests` for more targeted testing.

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

## Dependencies

```
-> cargo tree --depth 1
marie-ssg v0.8.1 (/.../marie-ssg)
├── argh v0.1.13
├── autumnus v0.7.8
├── basic-toml v0.1.10
├── chrono v0.4.41
├── fsevent v2.3.0
├── markdown v1.0.0
├── minijinja v2.14.0
├── minijinja-contrib v2.14.0
├── rayon v1.11.0
├── serde v1.0.228
├── thiserror v2.0.16
├── tracing v0.1.41
├── tracing-subscriber v0.3.19
└── walkdir v2.5.0
[dev-dependencies]
├── assert_cmd v2.1.1
├── criterion v0.5.1
├── html-escape v0.2.13
├── predicates v3.1.3
├── scraper v0.25.0
└── tempfile v3.21.0
```

## Version History

### v0.8.1 (2025-12-24)

- Added `sitemap_enabled` configuration option
- Added retroactive PRDs and workflow documentation
- Added comprehensive `examples/site.toml`
- Added performance benchmarks for HTML unescaping

### v0.8.0 (2025-12-24)

- Added automatic `sitemap.xml` generation

### v0.7.1 (2025-12-24)

- Added `url` filter to prevent forward slash escaping in templates

### v0.7.0 (2025-12-23)

- Added syntax highlighting for code blocks using Autumnus
- Supported languages: Rust, Python, JavaScript, TypeScript, HTML, CSS, Bash, JSON, TOML, YAML
- Configurable themes with `github_dark` as default
- Enable via `syntax_highlighting_enabled` and `syntax_highlighting_theme` in site config

### v0.6.0 (2025-12-22)

- Added `all_content` variable to index templates for tag counting

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
