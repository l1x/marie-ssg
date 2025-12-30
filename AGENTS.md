# Marie SSG - Agent Context Guide

This document provides essential context for AI agents working on the Marie SSG project.

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

### Issue Tracking

The project uses **Beads** for issue tracking. Key commands:

```bash
mise run show-issues      # List all open issues
mise run show-ready       # Show unblocked work ready to start
mise run show-blocked     # Show blocked issues
mise run show-issue-stats # Show issue statistics
```

## Project Management

See `@docs/prompts/context/project-management.md` for complete project management guidelines.

**Key concepts:**

- **Dot notation**: Organizational hierarchy (e.g., `mssg-vh1.5` is child of epic `mssg-vh1`)
- **Dependencies**: Execution order (what blocks what)
- Use `bd ready` to find unblocked tasks
- Use `bd blocked` to see what's waiting and why

## Testing Instructions

**Test structure:**

- Unit tests: Integrated in source files (`src/*.rs`)
- Integration tests: Located in `tests/` directory
- Benchmarks: Located in `benches/` directory

**Running tests:**

```bash
# Run all tests with output
mise run tests

# Run only unit tests
mise run unit-tests

# Run only integration tests
mise run integration-tests

# Run specific test
cargo test test_name -- --nocapture

# Run with coverage
mise run coverage
```

**Before committing:**

1. Run `mise run verify` (lint + tests)
2. Ensure all tests pass
3. No clippy warnings allowed
4. Add or update tests for code changes

## Code Quality Standards

**Linting:**

- Format with `mise fmt`
- Lint with `mise lint`
- All code must pass `mise run verify` before commit

**Error handling:**

- Use `thiserror` for proper error types
- Never use `.expect()` in production code paths
- Provide user-friendly error messages
- Use appropriate exit codes for CLI errors

**Performance:**

- Parallel content loading with Rayon
- Minimize cloning in hot paths
- Use `into_par_iter()` when consuming collections
- Profile with `mise run build-prod-with-timings` if needed

## Project Structure

```
marie-ssg/
├── src/                    # Source code
│   ├── main.rs            # CLI entry point
│   ├── config.rs          # Configuration loading
│   ├── content.rs         # Content parsing and metadata
│   ├── template.rs        # Template rendering
│   ├── syntax.rs          # Syntax highlighting
│   ├── utils.rs           # Utility functions
│   ├── sitemap.rs         # Sitemap generation
│   ├── output.rs          # File output
│   └── error.rs           # Error types
├── tests/                  # Integration tests
├── benches/                # Performance benchmarks
├── docs/                   # Documentation
│   └── prompts/           # AI agent prompts
├── examples/               # Example sites
├── Cargo.toml             # Rust dependencies
├── mise.toml              # Task definitions and tool versions
├── README.md              # User-facing documentation
└── AGENTS.md              # This file
```

## Key Dependencies

**Core dependencies:**

- `argh` - CLI argument parsing
- `markdown` - Markdown to HTML conversion
- `minijinja` + `minijinja-contrib` - Jinja-style templating
- `rayon` - Parallel processing
- `toml` - TOML parsing (migrated from basic-toml)
- `time` + `kiters` - Date/time handling (migrated from chrono)
- `autumnus` - Syntax highlighting (10 language features)
- `serde` - Serialization/deserialization
- `thiserror` - Error handling
- `tracing` + `tracing-subscriber` - Logging

**Dev dependencies:**

- `criterion` - Benchmarking
- `assert_cmd` - CLI testing
- `tempfile` - Test file management
- `predicates` - Test assertions

## Architecture Notes

**Content loading:**

- Parallel loading with Rayon
- TOML metadata parsed with `toml` crate
- Dates handled with `time` crate (RFC3339 format)

**Template rendering:**

- Minijinja with Jinja2-compatible syntax
- Custom filters for date formatting
- Template inheritance supported

**Syntax highlighting:**

- Autumnus with 10 languages: bash, css, html, javascript, json, python, rust, toml, typescript, yaml
- HTML entity unescaping with UTF-8 preservation
- Optimized string processing (O(n) instead of O(n²))

**Release profile:**

- Optimized for size (`opt-level = "s"`)
- LTO enabled
- Symbols stripped (debug info in separate file)
- Target: ~9MB binary (down from 80MB)

## Common Workflows

**Adding a new feature:**

1. Create or find relevant Beads ticket
2. Create failing test first
3. Implement feature
4. Run `mise run verify`
5. Update documentation if needed
6. Commit with clear message

**Bug fix:**

1. Reproduce with test
2. Fix bug
3. Verify test passes
4. Check for similar issues
5. Run `mise run verify`

**Performance work:**

1. Add benchmark in `benches/`
2. Run baseline: `cargo bench`
3. Optimize
4. Verify improvement
5. Add comments explaining optimization

## Release Process

Version is managed in `Cargo.toml`. Release checklist:

1. All tests passing (`mise run verify`)
2. Documentation updated
3. Version bumped in `Cargo.toml`
4. Changelog updated in `README.md`
5. Git tag created: `git tag -a v0.x.x -m "Release v0.x.x"`
6. Tag pushed: `git push origin v0.x.x`

See `@docs/prompts/context/project-management.md` for complete Beads workflow.

## Important Context

- **Project is Rust-only** - no Node.js/pnpm (ignore those examples from template)
- **Single binary CLI** - no libraries or multiple packages
- **Focus on simplicity** - Marie does one thing well
- **macOS watch mode** - uses fsevent for file watching (only on macOS)
- **Binary size matters** - optimized release profile targets small binaries
- **Test coverage** - maintain high coverage, use `mise run coverage` to check
