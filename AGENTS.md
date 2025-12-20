# Marie SSG - Agent Guidelines

## Project Overview

Marie SSG (Super Simple Static Site Generator) is a Rust-based static site generator that converts markdown files with TOML metadata into HTML pages using Jinja-style templates.

## Build Commands

```bash
mise run lint                       # Run cargo lint (formatting + style checks)
mise run tests                      # Run all tests with --nocapture
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

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
