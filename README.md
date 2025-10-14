# Marie SSG

Actually Marie SSSSG (super simple static site generator)

## Version History

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

## Code organization

```
src/
├── main.rs              # CLI entry point
├── error.rs             # All error types (RunError, ConfigError, ContentError, StaticError, WriteError)
├── config.rs            # Config types + loading (Config, ContentTypeConfig, load_config)
├── content.rs           # Content types + processing (Content, ContentMeta, ContentItem, load_content, load_metadata, convert_content)
├── template.rs          # Template rendering (render_html, render_index_with_contents, get_content_by_type)
├── utils.rs             # Utility functions (find_markdown_files, get_output_path, add_date_prefix, get_content_type, get_content_type_template)
└── output.rs            # Output operations (write_output_file, copy_static_files)
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
