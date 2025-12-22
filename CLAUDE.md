# Marie SSG - Agent Guidelines

## Project Overview

Marie SSG (Super Simple Static Site Generator) is a Rust-based static site generator that converts markdown files with TOML metadata into HTML pages using Jinja-style templates.

## Build Commands

```bash
mise run verify                     # Full verification (lint + tests) before commit
mise run lint                       # Run cargo lint (formatting + style checks)
mise run tests                      # Run all tests with --nocapture
mise run unit-tests                 # Run unit tests only
mise run integration-tests          # Run integration tests only
mise run build-dev                  # Development build
mise run build-prod                 # Release build (optimized for size)
cargo test <test_name>              # Run a single test
bd list                             # List open tickets
bd ready                            # to find available work
bd create                           # for new issues,
bd update <id> --status=in_progress # to claim work
bd close <id>                       # closing ticket
```

## Architecture

The SSG follows a pipeline architecture:

1. **Config loading** (`config.rs`) - Parses `site.toml` into `Config` struct with nested `SiteConfig`
2. **Content discovery** (`utils.rs`) - Finds markdown files in content directory
3. **Parallel content loading** (`content.rs` + `main.rs`) - Uses Rayon to load/convert markdown in parallel
4. **Template rendering** (`template.rs`) - Renders HTML using minijinja with template caching via `OnceLock`
5. **Output writing** (`output.rs`) - Writes rendered HTML and copies static files

Key data flow:

- `LoadedContent` struct (in `main.rs`) holds the loaded content, converted HTML, content type, and output path
- Content is loaded once and reused for both individual pages and index generation
- Template environment is cached globally to avoid recreation per render

## Content Type System

Content types are determined by directory structure under `content/`:

- `content/posts/*.md` → content type "posts"
- `content/pages/*.md` → content type "pages"

Each content type can have:

- A content template (e.g., `post.html`)
- An index template (e.g., `posts_index.html`)
- Optional date-prefix output naming

## Syntax Highlighting

Marie includes syntax highlighting for code blocks using the Autumnus library:

- Enabled by default (`syntax_highlighting_enabled = true`)
- Supports Rust, Python, JavaScript, TypeScript, HTML, CSS, Bash, JSON, TOML, YAML
- Theme configurable via `syntax_highlighting_theme` (default: "github_dark")
- Code blocks in markdown are automatically highlighted during HTML conversion
- Highlighting applied via `convert_content_with_highlighting()` in content pipeline

## Configuration Structure

```toml
# site.toml
author = "..."
title = "..."
domain = "..."
tagline = "..."
content_dir = "content"
template_dir = "templates"
static_dir = "static"
output_dir = "output"
site_index_template = "site_index.html"
syntax_highlighting_enabled = true          # Optional, defaults to true
syntax_highlighting_theme = "github_dark"   # Optional, defaults to "github_dark"

[site.root_static]
"favicon.ico" = "favicon.ico"
"robots.txt" = "robots.txt"

[dynamic]
# Custom template variables
```

## Code Style

- Group imports: std first, external crates, then internal modules (`use crate::*`)
- Use `thiserror` for custom error types with `#[source]` for error chaining
- Use `tracing` for logging with `#[instrument(skip_all)]` on key functions
- Prefer `pub(crate)` for internal visibility
- Use Rayon for parallel processing of independent operations

## Verification

- Run `mise run verify` before commits (runs lint + tests)
