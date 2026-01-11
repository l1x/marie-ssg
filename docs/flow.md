# Marie SSG Data Flow

This document describes how Marie SSG transforms input files into a static website.

## Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              INPUT                                          │
├─────────────────┬─────────────────┬─────────────────┬───────────────────────┤
│   site.toml     │    content/     │   templates/    │       static/         │
│   (config)      │  (md + meta)    │    (html)       │    (css, js, img)     │
└────────┬────────┴────────┬────────┴────────┬────────┴───────────┬───────────┘
         │                 │                 │                    │
         ▼                 ▼                 ▼                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           MARIE SSG BUILD                                   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │  Parse   │→ │  Load    │→ │  Render  │→ │ Generate │→ │  Write   │      │
│  │  Config  │  │ Content  │  │ Templates│  │ Sitemap  │  │  Output  │      │
│  └──────────┘  └──────────┘  └──────────┘  │ RSS Feed │  └──────────┘      │
│                                            └──────────┘                     │
└─────────────────────────────────────────────────────────────────────────────┘
         │                 │                 │                    │
         ▼                 ▼                 ▼                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              OUTPUT                                         │
├─────────────────┬─────────────────┬─────────────────┬───────────────────────┤
│   index.html    │  {type}/        │  sitemap.xml    │       static/         │
│   (homepage)    │  (content)      │  feed.xml       │    (copied assets)    │
└─────────────────┴─────────────────┴─────────────────┴───────────────────────┘
```

## Data Sources

Each piece of data in the final HTML comes from a specific source:

| Data | Source | Description |
|------|--------|-------------|
| `title` | `.meta.toml` | Article/page title |
| `date` | `.meta.toml` | Publication date (RFC 3339) |
| `author` | `.meta.toml` | Author name |
| `tags` | `.meta.toml` | Array of tags |
| `cover` | `.meta.toml` | Optional cover image path |
| `extra.*` | `.meta.toml` | Custom key-value fields |
| `extra_js` | `.meta.toml` | JavaScript files array |
| `content` | `.md` file | Rendered HTML from markdown |
| `excerpt` | `.md` file | HTML from `## Context` section |
| `content_type` | directory | Parent directory name (e.g., `blog`) |
| `slug` | filename | Filename stem (e.g., `hello-world`) |
| `filename` | computed | Output path based on `url_pattern` |

## Input Structure

```
my-site/
├── site.toml                 # Site configuration
│
├── content/                  # Markdown content
│   ├── articles/
│   │   ├── hello-world.md
│   │   └── hello-world.meta.toml
│   └── projects/
│       ├── my-project.md
│       └── my-project.meta.toml
│
├── templates/                # Jinja templates
│   ├── base.html            # Base layout
│   ├── article.html         # Article template
│   └── index.html           # Homepage template
│
└── static/                   # Static assets
    ├── css/style.css
    ├── js/app.js
    └── images/
```

## Content File Pairing

Each content item consists of two files:

```
┌─────────────────────────┐     ┌─────────────────────────┐
│   hello-world.md        │     │  hello-world.meta.toml  │
├─────────────────────────┤     ├─────────────────────────┤
│ # Hello World           │     │ title = "Hello World"   │
│                         │     │ date = "2025-01-15..."  │
│ ## Context              │     │ author = "Jane Doe"     │
│ Introduction text...    │ ←── │ tags = ["intro"]        │
│                         │     │                         │
│ ## Main Content         │     │ [extra]                 │
│ Full article body...    │     │ reading_time = "5 min"  │
└─────────────────────────┘     └─────────────────────────┘
         │                                  │
         │        ┌───────────────┐         │
         └───────→│ LoadedContent │←────────┘
                  ├───────────────┤
                  │ html: String  │ ← rendered markdown
                  │ meta: Meta    │ ← parsed TOML
                  │ excerpt: Str  │ ← from ## Context
                  │ content_type  │ ← from directory
                  └───────────────┘
```

## Template Rendering

Templates receive context variables from multiple sources:

```
┌─────────────────────────────────────────────────────────────────┐
│                     Template Context                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  From site.toml:           From content:                        │
│  ┌─────────────────┐       ┌─────────────────┐                 │
│  │ config.site.*   │       │ meta.title      │                 │
│  │ config.dynamic.*│       │ meta.date       │                 │
│  └─────────────────┘       │ meta.author     │                 │
│                            │ meta.tags       │                 │
│                            │ meta.cover      │                 │
│                            │ meta.extra.*    │                 │
│                            │ meta.extra_js   │                 │
│                            │ content (HTML)  │                 │
│                            │ excerpt (HTML)  │                 │
│                            │ filename        │                 │
│                            │ formatted_date  │                 │
│                            └─────────────────┘                 │
│                                                                 │
│  Index templates also get:                                      │
│  ┌─────────────────┐                                           │
│  │ contents[]      │ ← all items of this content type          │
│  │ all_content[]   │ ← all items across all types              │
│  └─────────────────┘                                           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## URL Pattern System

The output path is computed from the `url_pattern` configuration:

```
Input:
  File: content/articles/hello-world.md
  meta.date: 2025-01-15T10:00:00Z
  url_pattern: "{{date}}-{{stem}}"
  clean_urls: true

Processing:
  ┌────────────────┐
  │ {{stem}}       │ → hello-world     (from filename)
  │ {{date}}       │ → 2025-01-15      (from meta.date)
  │ {{year}}       │ → 2025            (from meta.date)
  │ {{month}}      │ → 01              (from meta.date)
  │ {{day}}        │ → 15              (from meta.date)
  └────────────────┘

Output:
  output/articles/2025-01-15-hello-world/index.html
                  └──────────┬─────────┘
                       url_pattern resolved
```

## Output Structure

```
output/
├── index.html                              # Site homepage
├── sitemap.xml                             # Generated sitemap
├── feed.xml                                # RSS feed
│
├── articles/
│   ├── index.html                          # Articles index
│   ├── 2025-01-15-hello-world/
│   │   └── index.html                      # Article page
│   └── 2025-01-10-another-post/
│       └── index.html
│
├── projects/
│   ├── index.html                          # Projects index
│   └── my-project/
│       └── index.html                      # Project page
│
└── static/                                 # Copied assets
    ├── css/
    │   └── style.a1b2c3d4.css             # Hashed filename
    ├── js/
    │   └── app.b5c6d7e8.js                # Hashed filename
    └── images/
        └── logo.png                        # Unchanged
```

## Build Pipeline

```
1. Parse Config
   site.toml → Config struct

2. Copy Static Assets
   static/ → output/static/
   (with optional content-based hashing for CSS/JS)

3. Load Content (parallel)
   For each .md file:
   ├── Read .meta.toml → ContentMeta
   ├── Read .md → raw markdown
   ├── Convert markdown → HTML
   ├── Extract excerpt from ## Context
   └── Create LoadedContent

4. Render Templates
   For each content type:
   ├── Render index template → {type}/index.html
   └── For each content item:
       └── Render content template → {type}/{slug}/index.html

5. Render Site Index
   index template + all_content → index.html

6. Generate Sitemap (if enabled)
   All URLs → sitemap.xml

7. Generate RSS Feed (if enabled)
   Content with rss_include=true → feed.xml

8. Generate Redirects (if configured)
   redirect mappings → HTML redirect files
```

## Asset Hashing Flow

When `asset_hashing_enabled = true`:

```
static/css/style.css
        │
        ▼
┌───────────────────┐
│ Compute BLAKE3    │
│ hash of content   │
│ → "a1b2c3d4"      │
└───────────────────┘
        │
        ▼
output/static/css/style.a1b2c3d4.css
        │
        ▼
┌───────────────────┐
│ Asset Manifest    │
│ "css/style.css"   │
│  → "/static/css/  │
│     style.a1b...  │
└───────────────────┘
        │
        ▼
Template: {{ "css/style.css" | asset_hash }}
Output:   /static/css/style.a1b2c3d4.css
```

## Color Legend for Diagrams

When creating visual diagrams:

- **Blue**: Data from `.meta.toml` (metadata)
- **Orange**: Data from `.md` files (content)
- **Green**: Data derived from filesystem (directory, filename)
- **Purple**: Data from `site.toml` (configuration)
- **Gray**: Computed/generated data
