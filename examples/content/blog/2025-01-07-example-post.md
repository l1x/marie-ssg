+++
# Required metadata
title = "Example Blog Post"
date = "2025-01-07T10:00:00-05:00"
author = "Jane Doe"
tags = ["tutorial", "marie-ssg", "static-site"]

# Optional: specify a different template for this content
# template = "custom_post.html"

# Optional: cover image for social sharing and article headers
cover = "/static/images/example-cover.jpg"

# Optional: extra custom fields accessible in templates via meta.extra
[extra]
reading_time = "5 min"
series = "Getting Started"
difficulty = "beginner"
+++

## Context

This is an example blog post demonstrating all the metadata features available in Marie SSG.

## Introduction

Marie SSG supports rich metadata in your markdown files using TOML frontmatter.

### Standard Fields

- **title**: The post title (required)
- **date**: Publication date in RFC3339 format (required)
- **author**: Author name (required)
- **tags**: List of tags for categorization

### Optional Fields

- **template**: Override the default template for this content
- **cover**: Cover image URL for social sharing
- **extra**: Custom key-value pairs for your specific needs

## Code Example

Here's some syntax-highlighted code:

```rust
fn main() {
    println!("Hello from Marie SSG!");
}
```

```python
def greet(name: str) -> str:
    return f"Hello, {name}!"
```

## Accessing Extra Fields in Templates

In your Jinja templates, access extra fields like this:

```html
{% if meta.extra.reading_time %}
<span class="reading-time">{{ meta.extra.reading_time }}</span>
{% endif %}

{% if meta.cover %}
<img src="{{ meta.cover }}" alt="{{ meta.title }}">
{% endif %}
```

## Conclusion

That's all the metadata features available in Marie SSG!
