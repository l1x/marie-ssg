# PRD: Sitemap Generation

## 1. Overview

Search Engine Optimization (SEO) is critical for public content sites. A `sitemap.xml` file helps search engines discover and index pages more efficiently. This feature adds automatic generation of a standard XML sitemap based on the rendered content.

## 2. Goals

1.  **SEO**: Improve site discoverability by search engines.
2.  **Compliance**: Follow the sitemap.org standard protocol.
3.  **Automation**: No manual maintenance of the sitemap file.

## 3. User Stories

- As a site owner, I want a `sitemap.xml` generated automatically so that Google/Bing can crawl my site effectively.
- As a user, I want to be able to disable sitemap generation for private or test sites.
- As a user, I want the sitemap to include the `lastmod` date based on the content's date.

## 4. Functional Requirements

### FR-1: Configuration

- Add a `sitemap_enabled` boolean to the `[site]` config section (default to false or true, TBD - let's say opt-in or opt-out).
- Require `domain` to be set in config to generate full URLs.
- **Acceptance**: Build respects the flag.

### FR-2: XML Generation

- Generate a valid XML file at `output_dir/sitemap.xml`.
- Include the Homepage (`/`).
- Include Index pages for content types (e.g., `/blog/`).
- Include all individual content pages.
- **Acceptance**: XML passes validation against sitemap schema.

### FR-3: URL Formatting

- Construct full URLs using the configured `domain` (e.g., `https://example.com/blog/post.html`).
- Ensure paths use forward slashes.
- **Acceptance**: URLs are correct and accessible.

### FR-4: Last Modified

- Use the `date` from the content frontmatter for the `<lastmod>` tag.
- Format date as `YYYY-MM-DD`.
- **Acceptance**: `<lastmod>` matches the post date.

## 5. Non-functional Requirements

- **NFR-1**: Generation should be negligible in terms of build time impact.
- **NFR-2**: Scale to thousands of URLs without memory issues (streaming not strictly necessary for static string builder, but keep in mind).

## 6. Non-Goals

- Multiple sitemap files (sitemap index) - single file is sufficient for < 50k URLs.
- Priority or Changefreq tags (these are largely ignored by modern search engines, keep it simple).

## 7. Success Metrics

- Valid `sitemap.xml` generated for the example site.
