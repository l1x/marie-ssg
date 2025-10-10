# Marie SSG

Actually Marie SSSSG

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

## Code Review Action Items

### Critical Issues

- [ ] **Content Type Index Bug** (Location: `get_content_by_type` function)
      Incorrectly uses `.any()` to check path components, causing files to appear in multiple content type indexes (e.g., `blog` index showing `projects` content). Fix by using `get_content_type()` for accurate filtering.

- [ ] **Duplicate File Processing** (Location: `run()` → `render_index_with_contents()`)
      Processes markdown files twice (during page rendering and index generation), causing significant performance degradation. Fix by storing processed content during main loop.

### Major Improvements

- [ ] **Break down the large `run()` function** into smaller, focused functions
      Create dedicated functions for `process_content_files()`, `generate_content_type_indexes()`, and `generate_site_index()`

- [ ] **Error Handling in Index Rendering** (Location: `render_index_with_contents`)
      Converts file errors to generic `minijinja::Error`, losing context. Should return `Result<String, ContentError>` and propagate errors to `run()`

- [ ] **Implement template caching** to avoid recreating `Environment` multiple times
      Reuse template Environment instead of creating new one for each render

### Performance Optimizations

- [ ] **Implement parallel processing** using `rayon` for file processing
- [ ] **Add memory-efficient batch processing** for large sites to avoid loading all files into memory simultaneously
- [ ] **Stream large files** instead of loading entirely into memory
- [ ] **Use `Cow<str>` for borrowed/owned string handling** where appropriate
- [ ] **Add content caching** to avoid re-processing unchanged files

### Error Handling & Validation

- [ ] **Add configuration validation** to ensure required directories exist during config loading
- [ ] **Improve error context** with more descriptive messages using `.with_context()`
- [ ] **Make content type detection more robust** with proper error handling
- [ ] **Add path traversal protection** for file operations (ensure output paths don't escape output directory)
- [ ] **Add specific error variants** for missing metadata files and invalid date formats

### Code Structure & Quality

- [ ] **Extract shared markdown processing logic** into a common `process_markdown_file()` function
- [ ] **Use `Path` instead of `&str` for paths** - functions should take `&Path` instead of `&str` to avoid repeated conversions
- [ ] **Avoid `PathBuf` in function signatures** - prefer `&Path` for inputs; reserve `PathBuf` for owned paths
- [ ] **Simplify `add_date_prefix`** with cleaner path construction
- [ ] **Remove unnecessary clones** (e.g., in `load_content()` metadata file path handling)

### Architecture Improvements

- [ ] **Implement trait-based content processing system**
- [ ] **Design plugin architecture** for extensibility
- [ ] **Add support for different content processors** beyond markdown
- [ ] **Create a ContentProcessor struct** to manage state and caching

### Testing Enhancements

- [ ] **Add integration tests** for full site generation pipeline
- [ ] **Add error condition testing** (missing templates, invalid configs, etc.)
- [ ] **Add performance benchmarks** for large site processing
- [ ] **Test edge cases** for path handling and content type detection
- [ ] **Fix redundant tests** - merge `test_get_output_path_converts_md_to_html` and `test_get_output_path`
- [ ] **Add missing test for `add_date_prefix`** function

### Template System

- [ ] **Enrich template context** with common variables (current_year, base_url, etc.)
- [ ] **Add support for template inheritance** or partials
- [ ] **Remove hardcoded fallback template** - move `"default.html"` to config (e.g., `config.default_template`)

### Content & Static File Processing

- [ ] **Add support for content preprocessing** (syntax highlighting, etc.)
- [ ] **Implement content sorting and filtering options**
- [ ] **Add pagination support** for index pages
- [ ] **Implement checksum-based copying** for static files to avoid unnecessary operations
- [ ] **Add file watching and incremental static file copying**

### Security

- [ ] **Add input sanitization** for template variables
- [ ] **Validate template safety** (no arbitrary code execution)
- [ ] **Add content sanitization options** for HTML output

### Nice-to-Have Features

- [ ] **Add watch mode** for development
- [ ] **Implement incremental builds**
- [ ] **Add RSS/Atom feed generation**
- [ ] **Implement sitemap.xml generation**
- [ ] **Add support for custom taxonomies** (categories, tags)
- [ ] **Create configuration file validation tool**
- [ ] **Add progress bars** for long-running operations

### Documentation

- [ ] **Add example configurations and templates**
- [ ] **Document content type system and template variables**
- [ ] **Create troubleshooting guide** for common issues
- [ ] **Improve code documentation** with more examples
