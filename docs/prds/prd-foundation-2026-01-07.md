# PRD: Marie SSG Foundation

## 1. Overview

Marie SSG is a simple static site generator written in Rust. The goal is to create a lightweight, fast, and opinionated tool for building personal websites and blogs from Markdown content. It prioritizes simplicity over feature bloat, offering a clear configuration structure and standard templating.

## 2. Goals

1.  **Simplicity**: Easy to understand "inputs -> outputs" model.
2.  **Performance**: Fast build times using Rust.
3.  **Flexibility**: Support for arbitrary content types (e.g., blog posts, pages, projects) via configuration.
4.  **Developer Experience**: clear error messages and simple CLI interface.

## 3. User Stories

- As a user, I want to define my site structure in a single TOML configuration file so that I can easily manage settings.
- As a writer, I want to write content in Markdown with TOML meta data so that I can focus on writing.
- As a designer, I want to use Jinja2-style templates (MiniJinja) so that I can customize the HTML output.
- As a user, I want to define different content types (like "blog" or "projects") that get rendered to specific output directories.
- As a user, I want static assets (CSS, images) to be copied to the output directory automatically.

## 4. Functional Requirements

### FR-1: Configuration

- Accept a configuration file (default `site.toml`).
- **Acceptance**: The CLI must fail gracefully if the config is missing or invalid.

### FR-2: Content Processing

- Read Markdown files recursively from a configured `content_dir`.
- Parse TOML frontmatter for metadata (title, date, etc.).
- Convert Markdown body to HTML.
- **Acceptance**: All valid `.md` files in the content directory are processed.

### FR-3: Templating

- Use `minijinja` for rendering.
- Support specific templates for different content types (e.g., `post.html` vs `page.html`).
- Support index templates for listing content (e.g., `blog_index.html`).
- **Acceptance**: Templates must have access to page metadata and content.

### FR-4: Output Generation

- Write generated HTML files to a configured `output_dir`.
- Maintain the directory structure or flatten based on configuration (initial scope: simple mapping).
- Copy files from `static_dir` to `output_dir` unchanged.
- **Acceptance**: A build command produces a valid website structure in the output folder.

### FR-5: CLI Interface

- Provide a `build` subcommand.
- Support a `-c/--config` flag to specify the config file path.
- **Acceptance**: `marie-ssg build` triggers the build process.

### FR-6: Clean URL Structure (added 2026-01-07)

- Support optional clean URL output format via `clean_urls` config option.
- When enabled, output files as `<content-type>/<slug>/index.html` instead of `<content-type>/<slug>.html`.
- Strip date prefixes (YYYY-MM-DD-) from URL slugs while preserving date in metadata for sorting.
- Update sitemap.xml URLs to use trailing slash format (`/blog/my-post/`).
- Update RSS feed URLs to use trailing slash format.
- Template `filename` field should output trailing slash URLs for clean URL mode.
- **Acceptance**:
  - `clean_urls = false` (default): outputs `blog/my-post.html` → URL `/blog/my-post.html`
  - `clean_urls = true`: outputs `blog/my-post/index.html` → URL `/blog/my-post/`
  - Date-prefixed files like `2025-01-07-my-post.md` produce slug `my-post` (not `2025-01-07-my-post`)

## 5. Non-functional Requirements

- **NFR-1**: Build time should be under 1 second for small sites (< 100 pages).
- **NFR-2**: Binary size should be minimized (release profile optimizations).
- **NFR-3**: Error messages should be descriptive (no raw stack traces for user errors).

## 6. Non-Goals

- Plugin system.
- Complex asset processing pipeline (Sass/JS bundling) - users can do this externally.
- Dynamic server features (search, comments).

## 7. Success Metrics

- Successful build of a demo site with blog posts and pages.
- Zero panic crashes on malformed markdown (should report error).
