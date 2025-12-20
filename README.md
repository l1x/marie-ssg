# Marie SSG

Actually Marie SSSSG (super simple static site generator)

## Usage

```bash
# Show version
marie-ssg -V
marie-ssg --version

# Build site with default config (site.toml)
marie-ssg build

# Build site with custom config file
marie-ssg build -c mysite.toml
marie-ssg build --config-file mysite.toml

# Watch for changes and rebuild automatically (macOS only)
marie-ssg watch
marie-ssg watch -c mysite.toml

# Show help
marie-ssg --help
marie-ssg build --help
marie-ssg watch --help
```

## Version History

### v0.5.0 (2025-12-20) - CLI Improvements

**CLI Changes:**

- **Added version flag**: `-V` / `--version` prints the version from Cargo.toml
- **Introduced subcommand structure**: Build functionality moved to `build` subcommand
- **Config option on build**: `-c` / `--config-file` option now on the `build` subcommand
- **Cleaner output**: Removed unnecessary "Starting up..." and "ok" log messages
- **Watch mode (macOS)**: New `watch` subcommand monitors content, templates, and static directories for changes and triggers automatic rebuilds with 500ms debouncing

**Dependencies Added:**

- `fsevent` - macOS file system events for watch mode (conditional, macOS only)

### v0.4.0 (2025-12-15) - Testing & Quality Improvements

**Testing Infrastructure:**

- **Comprehensive integration testing**: Added CLI-based integration tests with HTML DOM validation using `scraper` library
- **Test coverage increased**: From 57.93% to 86.35% (exceeding 80% goal)
- **HTML parsing validation**: Tests now validate actual HTML structure and content, not just file existence
- **Complete test fixtures**: Created realistic example site in `tests/fixtures/simple_site/` demonstrating all features
- **Template rendering tests**: Added unit tests for template functions with proper DOM validation

**Code Quality Improvements:**

- **Refactored template rendering**: Changed from implicit `OnceLock` access to explicit `&Environment` parameter passing
- **Improved testability**: Template functions now accept environment as parameter, enabling isolated unit testing
- **64 total tests**: 54 unit tests + 10 integration tests, all passing
- **Module coverage**:
  - src/main.rs: 0% → 95.6%
  - src/template.rs: 83.9% → 100%
  - src/output.rs: 68.1% → 75.5%

**Dependencies Added:**

- `scraper` - HTML parsing with CSS selectors for integration test validation
- `assert_cmd` - CLI application testing framework
- `predicates` - Assertion library for CLI output validation

### v0.3.0 (2025-10-14) - Configuration Changes

**Performance Improvements:**

- **Implemented template caching**: Added `OnceLock<Environment>` to reuse template environment instead of recreating for each render
- **Added build tooling**: Introduced `mise.toml` with standardized build tasks (lint, test, build-dev/prod)
- **Removed manual progress markers**: Cleaned up debug comments from parallel processing implementation

**Configuration Architecture:**

- **Introduced nested config structure**: Created `SiteConfig` struct to organize site settings under `config.site.*`
- **Added dynamic variables**: New `config.dynamic` HashMap for custom template variables
- **Changed content_types handling**: Switched from `#[serde(flatten)]` to `#[serde(default)]` for better default handling

**Documentation Transformation:**

- **Completely restructured README**:
  - Added comprehensive version history with detailed v0.2.0 performance improvements
  - Added development workflow section showing mise tasks
  - Removed 50+ line code review todo list (moved to internal documentation)
- **Added project description**: "Actually Marie SSSSG (super simple static site generator)"

### v0.2.0 (2025-10-10) - Performance & Observability

**Performance Improvements:**

- **Single-pass content loading** - Eliminated duplicate file reads (66% fewer I/O operations)
- **Parallel processing with Rayon** - Content loading now runs in parallel across CPU cores
- **Optimized architecture** - Content loaded once, reused for page rendering and indexes
- **Build timing instrumentation** - Added performance metrics logging

**Observability:**

- **Enhanced logging** - Full source → output path tracing for all content
- **Progress indicators** - Shows which files are being processed
- **Better error context** - Improved error messages with file paths

**Code Quality:**

- **Removed dead code** - All struct fields now actively used
- **Production-ready error handling** - Proper Result propagation throughout
- **Clean compilation** - No warnings, all tests passing

### v0.1.0 - Initial Release

**Core Features:**

- Static site generation from markdown files
- TOML-based metadata system (`.meta.toml` files)
- Flexible content type system
- Template-based rendering with Jinja-style templates
- Automatic index generation per content type
- Site-wide index page
- Static asset copying (CSS, images, fonts)
- Date-prefixed output naming (configurable per content type)
- Clean separation of concerns across modules

## Running tasks in the repo

```bash
➜  mise tasks --all
Name                     Description
build-dev                Building the development version
build-prod               Building the development version
build-prod-with-timings  Building the development version
lint                     Running linting
tests                    Running tests
```

## Testing

Marie SSG has comprehensive test coverage (86.35%):

```bash
# Run all tests (unit + integration)
cargo test

# Run only integration tests
cargo test --test integration_test

# Run with coverage report
cargo tarpaulin --out Stdout

# Run specific test
cargo test test_blog_index_sorting
```

**Test Types:**
- **Unit tests** (54): Test individual functions and modules
- **Integration tests** (10): End-to-end CLI testing with HTML validation
- **Test fixtures**: Complete example site in `tests/fixtures/simple_site/`

**What's Tested:**
- ✅ Configuration loading and validation
- ✅ Markdown to HTML conversion
- ✅ Template rendering with metadata
- ✅ Multiple content types (blog, pages)
- ✅ Index generation and date-based sorting
- ✅ Static file copying
- ✅ Excerpt extraction
- ✅ HTML structure validation via DOM parsing
- ✅ Error handling

## Code organization

```
src/
├── main.rs              # CLI entry point & application logic
├── error.rs             # Error types (RunError, ConfigError, ContentError, StaticError, WriteError)
├── config.rs            # Config types + loading (Config, SiteConfig, ContentTypeConfig)
├── content.rs           # Content processing (Content, ContentMeta, ContentItem, load_content, convert_content)
├── template.rs          # Template rendering (init_environment, render_html, render_index_from_loaded)
├── utils.rs             # Utility functions (find_markdown_files, get_output_path, content type handling)
└── output.rs            # Output operations (write_output_file, copy_static_files)

tests/
├── integration_test.rs  # CLI integration tests with HTML validation
└── fixtures/
    └── simple_site/     # Complete example site for testing and demonstration
        ├── content/     # Sample markdown files with metadata
        ├── templates/   # Example templates (blog, pages, indexes)
        ├── static/      # Static assets (CSS, favicon, robots.txt)
        └── site.toml    # Full configuration example
```

## Content types

```
content/
├── posts/
│ ├── my-first-post.md
│ ├── my-first-post.meta.toml
│ ├── another-post.md
│ └── another-post.meta.toml
└── pages/
  ├── about.md
  └── about.meta.toml
```

## Templates

```
templates/
├── post.html               # For individual posts
├── post_index.html         # For posts listing
├── page.html               # For individual pages
├── page_index.html         # For pages listing
├── site_index.html         # For main site index
├── head.html               # Common head section
└── footer.html             # Common footer
```

- Dynamically loads all templates: Scans the template directory and loads all .html files, using the filename (without extension) as the template name.
- Content-type-based templates: Uses the content type directory name as the default template name (e.g., content in posts/ uses post.html template).
- Per-content template override: Allows specifying a different template in the metadata of individual content files.
- Automatic index generation: Creates index pages for each content type using [content_type]\_index.html templates (e.g., posts_index.html).
- Site-wide index: Generates a main site index using site_index.html template.
- Flexible structure: No hardcoded template names - everything is determined by the file structure and optional metadata.

## Project management

We track work with `bd`, the Birdseye CLI. Helpful commands:

```bash
bd list --status open         # show open tickets
bd ready                      # list work with no blockers
bd update <id> --status X     # move ticket through the workflow
```

### Active tickets

- mar-ex5 [P2] [task] open - Stream markdown processing in parallel

### Recently completed

- mar-3jw [P2] [task] closed - Increase test coverage from 46% to 80% (achieved 86.35%)
- mar-tap [P2] [task] closed - Reuse cached template environment
- mar-3xs [P2] [task] closed - Skip copying unchanged static assets
