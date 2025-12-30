## Project Overview

**Marie SSG** is a static site generator written in Rust that converts markdown files with TOML metadata into HTML pages using Jinja-style templates. It follows a pipeline architecture: load content in parallel, render through templates, write output.

**Key characteristics:**

- Single-purpose tool focused on doing one thing well
- Parallel content loading with Rayon
- Jinja-style templating with Minijinja
- Syntax highlighting with Autumnus (10 languages)
- Watch mode support on macOS

## Development Environment

### Tooling Setup

The project uses **mise** for task management and tool versioning. See `@mise.toml` for the complete task reference.

**Required tools:**

- Rust 1.90.0 (managed by mise)
- Python 3.13.10 (managed by mise)

**Key mise tasks:**

```bash
mise run fmt          # Format code with cargo fmt
mise run lint         # Lint with Clippy (fails on warnings)
mise run tests        # Run all tests with output
mise run verify       # Full verification (lint + tests)
mise run coverage     # Run tests with coverage (requires cargo-tarpaulin)
mise run build-dev    # Build development version
mise run build-prod   # Build release version
mise run audit        # Security audit on dependencies
mise run check-deps   # Run audit + find unused dependencies
```

## Examples

See [examples](examples/)

## Version History

### v0.9.0 (2025-12-30)

- Migrated from `chrono` to `time` crate for datetime handling
- Added `kiters` crate for UTC timestamp formatting in tracing
- Enhanced tracing with EnvFilter support and UTC timestamps
- Migrated from `basic-toml` to `toml` crate (~2x faster parsing)
- Reduced binary size from 80MB to 9MB (89% reduction) by limiting syntax highlighting languages
- Restored missing mise tasks (`tests`, `verify`)
- Added TOML parser benchmark and migration documentation

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
