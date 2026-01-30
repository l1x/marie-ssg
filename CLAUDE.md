# Marie SSG - Agent Context

A static site generator written in Rust that converts markdown with TOML metadata into HTML.

## Quick Start

```bash
# Load issue tracker context
bd prime

# See available work
mise run show-ready

# Full verification before commit
mise run verify
```

## Tooling

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.90.0 | Language runtime (managed by mise) |
| Cargo | (bundled) | Package manager and build tool |
| Clippy | (bundled) | Linter |
| cargo-tarpaulin | latest | Code coverage |
| cargo-audit | latest | Security audits |
| cargo-machete | latest | Unused dependency detection |

## Mise Tasks

All operations go through mise. Run `mise tasks` to see available commands.

### Code Quality

| Task | Command | Description |
|------|---------|-------------|
| `fmt` | `mise run fmt` | Format code with cargo fmt |
| `lint` | `mise run lint` | Lint with Clippy (fails on warnings) |
| `verify` | `mise run verify` | Full verification (lint + tests) |

### Testing

| Task | Command | Description |
|------|---------|-------------|
| `tests` | `mise run tests` | Run all tests with output |
| `unit-tests` | `mise run unit-tests` | Run unit tests only |
| `integration-tests` | `mise run integration-tests` | Run integration tests only |
| `coverage` | `mise run coverage` | Generate coverage report (HTML) |
| `bench` | `mise run bench` | Run performance benchmarks |

### Building

| Task | Command | Description |
|------|---------|-------------|
| `build-dev` | `mise run build-dev` | Development build |
| `build-prod` | `mise run build-prod` | Release build (optimized) |
| `build-prod-with-timings` | `mise run build-prod-with-timings` | Release build with timing info |

### Dependency Management

| Task | Command | Description |
|------|---------|-------------|
| `audit` | `mise run audit` | Security audit on dependencies |
| `machete` | `mise run machete` | Find unused dependencies |
| `check-deps` | `mise run check-deps` | Run audit + machete together |

### Issue Tracking (Beads)

| Task | Command | Description |
|------|---------|-------------|
| `show-issues` | `mise run show-issues` | List all open issues |
| `show-ready` | `mise run show-ready` | Show unblocked work |
| `show-blocked` | `mise run show-blocked` | Show blocked issues |
| `show-issue-stats` | `mise run show-issue-stats` | Project statistics |
| `show-issue-tree` | `mise run show-issue-tree` | Dependency tree for an issue |

### Maintenance

| Task | Command | Description |
|------|---------|-------------|
| `update-prompts` | `mise run update-prompts` | Update agent-prompts submodule |

## Project Structure

```
marie-ssg/
├── src/                    # Source code
│   ├── main.rs            # CLI entry point
│   ├── build.rs           # Site building orchestration
│   ├── config.rs          # Configuration loading
│   ├── content.rs         # Content parsing and metadata
│   ├── template.rs        # Template rendering (Minijinja)
│   ├── syntax.rs          # Syntax highlighting (Autumnus)
│   ├── output.rs          # File output
│   ├── rss.rs             # RSS feed generation
│   ├── sitemap.rs         # Sitemap generation
│   ├── redirect.rs        # URL redirect handling
│   ├── asset_hash.rs      # Asset hashing for cache busting
│   ├── utils.rs           # Utility functions
│   ├── watch.rs           # File watching (macOS only)
│   ├── flame.rs           # Flamechart profiling
│   ├── guide.rs           # Built-in guide command
│   └── error.rs           # Error types
├── tests/                  # Integration tests
│   ├── integration_test.rs
│   └── fixtures/          # Test fixtures
├── benches/                # Performance benchmarks
│   ├── simple_html_benchmark.rs
│   ├── unescape_html_benchmark.rs
│   └── toml_benchmark.rs
├── examples/               # Example configuration
│   ├── site.toml          # Complete config reference
│   └── content/           # Example content
├── docs/                   # Documentation
│   ├── diagrams/          # Architecture diagrams
│   └── prds/              # Product requirement docs
├── Cargo.toml             # Rust dependencies
├── mise.toml              # Task definitions
└── CLAUDE.md              # This file
```

## Explicit Denials

**NEVER perform these actions directly. Use mise tasks instead.**

### Forbidden Direct Commands

```bash
# DENIED - Use mise run fmt
cargo fmt

# DENIED - Use mise run lint
cargo clippy

# DENIED - Use mise run tests
cargo test

# DENIED - Use mise run build-dev or build-prod
cargo build

# DENIED - Use mise run audit
cargo audit
```

### Forbidden Destructive Actions

```bash
# DENIED - Never delete source files without explicit user request
rm src/*.rs
rm -rf src/

# DENIED - Never delete test files
rm tests/*.rs
rm -rf tests/

# DENIED - Never force push
git push --force
git push -f

# DENIED - Never reset hard without explicit request
git reset --hard

# DENIED - Never delete branches without explicit request
git branch -D
git branch -d

# DENIED - Never modify Cargo.toml dependencies without explicit request
# (adding, removing, or updating versions)
```

### If a Mise Task is Missing

If you need to perform an operation that should have a mise task but doesn't:

1. **Stop** - Do not run the command directly
2. **Propose** - Suggest adding the task to `mise.toml`
3. **Wait** - Get user approval before adding
4. **Add** - Add the task to `mise.toml`
5. **Run** - Use `mise run <task>` to execute

## Beads Workflow

### Session Start
```bash
bd prime                    # Load context after compaction/new session
```

### Finding Work
```bash
bd ready                    # Show unblocked tasks
bd list --status=open       # All open issues
bd show <id>                # Detailed issue view
```

### Working on Issues
```bash
bd update <id> --status=in_progress    # Claim work
bd close <id>                          # Mark complete
bd close <id1> <id2> ...               # Close multiple
```

### Session End Protocol

Before saying "done" or "complete":

```bash
git status                  # Check what changed
git add <files>             # Stage code changes
bd sync                     # Commit beads changes
git commit -m "..."         # Commit code
bd sync                     # Commit any new beads changes
git push                    # Push to remote
```

## Testing Requirements

- All code changes require tests
- Run `mise run verify` before every commit
- No Clippy warnings allowed
- Coverage reports: `mise run coverage` (outputs to `coverage/`)

## Error Handling

- Use `thiserror` for error types
- Never use `.expect()` or `.unwrap()` in production paths
- Provide user-friendly error messages
- Use appropriate exit codes

## Performance Guidelines

- Parallel content loading with Rayon
- Minimize cloning in hot paths
- Use `into_par_iter()` when consuming collections
- Profile with flamecharts: `marie-ssg flame`
- Run benchmarks: `mise run bench`

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `argh` | CLI argument parsing |
| `markdown` | Markdown to HTML |
| `minijinja` | Jinja-style templating |
| `rayon` | Parallel processing |
| `toml` | TOML parsing |
| `time` | Date/time handling |
| `autumnus` | Syntax highlighting |
| `tracing` | Logging |
| `thiserror` | Error handling |
