# Marie SSG Data Flow

This document describes how Marie SSG transforms input files into a static website.

## Overview

<svg viewBox="0 0 800 400" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#666"/>
    </marker>
  </defs>

  <!-- Input Section -->
  <rect x="10" y="10" width="780" height="100" rx="8" fill="#e8f4fc" stroke="#4a90d9" stroke-width="2"/>
  <text x="400" y="35" text-anchor="middle" font-family="system-ui" font-size="14" font-weight="bold" fill="#2c5282">INPUT</text>

  <!-- Input boxes -->
  <rect x="30" y="50" width="160" height="50" rx="4" fill="#9f7aea" stroke="#6b46c1" stroke-width="1.5"/>
  <text x="110" y="80" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">site.toml</text>

  <rect x="210" y="50" width="160" height="50" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1.5"/>
  <text x="290" y="72" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">content/</text>
  <text x="290" y="88" text-anchor="middle" font-family="system-ui" font-size="10" fill="white">(md + meta.toml)</text>

  <rect x="390" y="50" width="160" height="50" rx="4" fill="#48bb78" stroke="#276749" stroke-width="1.5"/>
  <text x="470" y="80" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">templates/</text>

  <rect x="570" y="50" width="160" height="50" rx="4" fill="#ed8936" stroke="#c05621" stroke-width="1.5"/>
  <text x="650" y="80" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">static/</text>

  <!-- Arrows down -->
  <line x1="110" y1="100" x2="110" y2="140" stroke="#666" stroke-width="2" marker-end="url(#arrowhead)"/>
  <line x1="290" y1="100" x2="290" y2="140" stroke="#666" stroke-width="2" marker-end="url(#arrowhead)"/>
  <line x1="470" y1="100" x2="470" y2="140" stroke="#666" stroke-width="2" marker-end="url(#arrowhead)"/>
  <line x1="650" y1="100" x2="650" y2="140" stroke="#666" stroke-width="2" marker-end="url(#arrowhead)"/>

  <!-- Build Section -->
  <rect x="10" y="150" width="780" height="100" rx="8" fill="#faf5ff" stroke="#9f7aea" stroke-width="2"/>
  <text x="400" y="175" text-anchor="middle" font-family="system-ui" font-size="14" font-weight="bold" fill="#553c9a">MARIE SSG BUILD</text>

  <!-- Build stages -->
  <rect x="40" y="190" width="100" height="40" rx="4" fill="#fff" stroke="#9f7aea" stroke-width="1"/>
  <text x="90" y="215" text-anchor="middle" font-family="system-ui" font-size="10" fill="#553c9a">Parse Config</text>

  <rect x="160" y="190" width="100" height="40" rx="4" fill="#fff" stroke="#9f7aea" stroke-width="1"/>
  <text x="210" y="215" text-anchor="middle" font-family="system-ui" font-size="10" fill="#553c9a">Load Content</text>

  <rect x="280" y="190" width="100" height="40" rx="4" fill="#fff" stroke="#9f7aea" stroke-width="1"/>
  <text x="330" y="215" text-anchor="middle" font-family="system-ui" font-size="10" fill="#553c9a">Render HTML</text>

  <rect x="400" y="190" width="100" height="40" rx="4" fill="#fff" stroke="#9f7aea" stroke-width="1"/>
  <text x="450" y="208" text-anchor="middle" font-family="system-ui" font-size="10" fill="#553c9a">Generate</text>
  <text x="450" y="222" text-anchor="middle" font-family="system-ui" font-size="10" fill="#553c9a">Sitemap/RSS</text>

  <rect x="520" y="190" width="100" height="40" rx="4" fill="#fff" stroke="#9f7aea" stroke-width="1"/>
  <text x="570" y="215" text-anchor="middle" font-family="system-ui" font-size="10" fill="#553c9a">Copy Assets</text>

  <rect x="640" y="190" width="100" height="40" rx="4" fill="#fff" stroke="#9f7aea" stroke-width="1"/>
  <text x="690" y="215" text-anchor="middle" font-family="system-ui" font-size="10" fill="#553c9a">Write Output</text>

  <!-- Stage arrows -->
  <line x1="140" y1="210" x2="155" y2="210" stroke="#9f7aea" stroke-width="1.5" marker-end="url(#arrowhead)"/>
  <line x1="260" y1="210" x2="275" y2="210" stroke="#9f7aea" stroke-width="1.5" marker-end="url(#arrowhead)"/>
  <line x1="380" y1="210" x2="395" y2="210" stroke="#9f7aea" stroke-width="1.5" marker-end="url(#arrowhead)"/>
  <line x1="500" y1="210" x2="515" y2="210" stroke="#9f7aea" stroke-width="1.5" marker-end="url(#arrowhead)"/>
  <line x1="620" y1="210" x2="635" y2="210" stroke="#9f7aea" stroke-width="1.5" marker-end="url(#arrowhead)"/>

  <!-- Arrows down -->
  <line x1="110" y1="250" x2="110" y2="290" stroke="#666" stroke-width="2" marker-end="url(#arrowhead)"/>
  <line x1="290" y1="250" x2="290" y2="290" stroke="#666" stroke-width="2" marker-end="url(#arrowhead)"/>
  <line x1="470" y1="250" x2="470" y2="290" stroke="#666" stroke-width="2" marker-end="url(#arrowhead)"/>
  <line x1="650" y1="250" x2="650" y2="290" stroke="#666" stroke-width="2" marker-end="url(#arrowhead)"/>

  <!-- Output Section -->
  <rect x="10" y="300" width="780" height="90" rx="8" fill="#f0fff4" stroke="#48bb78" stroke-width="2"/>
  <text x="400" y="325" text-anchor="middle" font-family="system-ui" font-size="14" font-weight="bold" fill="#276749">OUTPUT</text>

  <!-- Output boxes -->
  <rect x="30" y="340" width="160" height="40" rx="4" fill="#48bb78" stroke="#276749" stroke-width="1.5"/>
  <text x="110" y="365" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">index.html</text>

  <rect x="210" y="340" width="160" height="40" rx="4" fill="#48bb78" stroke="#276749" stroke-width="1.5"/>
  <text x="290" y="365" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">{type}/*.html</text>

  <rect x="390" y="340" width="160" height="40" rx="4" fill="#48bb78" stroke="#276749" stroke-width="1.5"/>
  <text x="470" y="358" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">sitemap.xml</text>
  <text x="470" y="374" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">feed.xml</text>

  <rect x="570" y="340" width="160" height="40" rx="4" fill="#48bb78" stroke="#276749" stroke-width="1.5"/>
  <text x="650" y="365" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">static/*</text>
</svg>

## Data Sources

Each piece of data in the final HTML comes from a specific source:

<svg viewBox="0 0 700 320" xmlns="http://www.w3.org/2000/svg">
  <!-- Legend -->
  <rect x="10" y="10" width="680" height="40" rx="4" fill="#f7fafc" stroke="#e2e8f0" stroke-width="1"/>
  <rect x="20" y="20" width="20" height="20" rx="2" fill="#4299e1"/>
  <text x="50" y="35" font-family="system-ui" font-size="11" fill="#2d3748">meta.toml</text>
  <rect x="140" y="20" width="20" height="20" rx="2" fill="#ed8936"/>
  <text x="170" y="35" font-family="system-ui" font-size="11" fill="#2d3748">.md file</text>
  <rect x="250" y="20" width="20" height="20" rx="2" fill="#48bb78"/>
  <text x="280" y="35" font-family="system-ui" font-size="11" fill="#2d3748">filesystem</text>
  <rect x="370" y="20" width="20" height="20" rx="2" fill="#9f7aea"/>
  <text x="400" y="35" font-family="system-ui" font-size="11" fill="#2d3748">site.toml</text>
  <rect x="490" y="20" width="20" height="20" rx="2" fill="#a0aec0"/>
  <text x="520" y="35" font-family="system-ui" font-size="11" fill="#2d3748">computed</text>

  <!-- Data items -->
  <rect x="10" y="60" width="150" height="35" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1"/>
  <text x="85" y="82" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">title</text>

  <rect x="170" y="60" width="150" height="35" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1"/>
  <text x="245" y="82" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">date</text>

  <rect x="330" y="60" width="150" height="35" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1"/>
  <text x="405" y="82" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">author</text>

  <rect x="490" y="60" width="150" height="35" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1"/>
  <text x="565" y="82" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">tags</text>

  <rect x="10" y="105" width="150" height="35" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1"/>
  <text x="85" y="127" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">cover</text>

  <rect x="170" y="105" width="150" height="35" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1"/>
  <text x="245" y="127" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">extra.*</text>

  <rect x="330" y="105" width="150" height="35" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1"/>
  <text x="405" y="127" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">extra_js</text>

  <rect x="490" y="105" width="150" height="35" rx="4" fill="#4299e1" stroke="#2b6cb0" stroke-width="1"/>
  <text x="565" y="127" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">template</text>

  <rect x="10" y="150" width="150" height="35" rx="4" fill="#ed8936" stroke="#c05621" stroke-width="1"/>
  <text x="85" y="172" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">content (HTML)</text>

  <rect x="170" y="150" width="150" height="35" rx="4" fill="#ed8936" stroke="#c05621" stroke-width="1"/>
  <text x="245" y="172" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">excerpt</text>

  <rect x="330" y="150" width="150" height="35" rx="4" fill="#48bb78" stroke="#276749" stroke-width="1"/>
  <text x="405" y="172" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">content_type</text>

  <rect x="490" y="150" width="150" height="35" rx="4" fill="#48bb78" stroke="#276749" stroke-width="1"/>
  <text x="565" y="172" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">stem (slug)</text>

  <rect x="10" y="195" width="150" height="35" rx="4" fill="#9f7aea" stroke="#6b46c1" stroke-width="1"/>
  <text x="85" y="217" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">config.site.*</text>

  <rect x="170" y="195" width="150" height="35" rx="4" fill="#9f7aea" stroke="#6b46c1" stroke-width="1"/>
  <text x="245" y="217" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">config.dynamic.*</text>

  <rect x="330" y="195" width="150" height="35" rx="4" fill="#a0aec0" stroke="#718096" stroke-width="1"/>
  <text x="405" y="217" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">filename</text>

  <rect x="490" y="195" width="150" height="35" rx="4" fill="#a0aec0" stroke="#718096" stroke-width="1"/>
  <text x="565" y="217" text-anchor="middle" font-family="system-ui" font-size="12" fill="white">formatted_date</text>

  <!-- Descriptions -->
  <text x="10" y="260" font-family="system-ui" font-size="11" fill="#4a5568">
    <tspan x="10" dy="0">• content_type: derived from parent directory name (e.g., "articles", "projects")</tspan>
    <tspan x="10" dy="18">• stem: filename without extension or date prefix (e.g., "hello-world" from "2025-01-15-hello-world.md")</tspan>
    <tspan x="10" dy="18">• filename: computed from url_pattern (e.g., "2025-01-15-hello-world/index.html")</tspan>
  </text>
</svg>

## Content File Pairing

Each content item consists of two paired files:

<svg viewBox="0 0 700 280" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <marker id="arrow2" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#666"/>
    </marker>
  </defs>

  <!-- Markdown file -->
  <rect x="10" y="10" width="200" height="180" rx="6" fill="#fff7ed" stroke="#ed8936" stroke-width="2"/>
  <rect x="10" y="10" width="200" height="30" rx="6" fill="#ed8936"/>
  <text x="110" y="30" text-anchor="middle" font-family="monospace" font-size="11" fill="white">hello-world.md</text>

  <text x="20" y="60" font-family="monospace" font-size="10" fill="#c05621"># Hello World</text>
  <text x="20" y="80" font-family="monospace" font-size="10" fill="#c05621">## Context</text>
  <text x="20" y="100" font-family="monospace" font-size="10" fill="#744210">Introduction text that</text>
  <text x="20" y="115" font-family="monospace" font-size="10" fill="#744210">becomes the excerpt...</text>
  <text x="20" y="140" font-family="monospace" font-size="10" fill="#c05621">## Main Content</text>
  <text x="20" y="160" font-family="monospace" font-size="10" fill="#744210">Full article body</text>
  <text x="20" y="175" font-family="monospace" font-size="10" fill="#744210">goes here...</text>

  <!-- Meta file -->
  <rect x="230" y="10" width="200" height="180" rx="6" fill="#ebf8ff" stroke="#4299e1" stroke-width="2"/>
  <rect x="230" y="10" width="200" height="30" rx="6" fill="#4299e1"/>
  <text x="330" y="30" text-anchor="middle" font-family="monospace" font-size="11" fill="white">hello-world.meta.toml</text>

  <text x="240" y="60" font-family="monospace" font-size="10" fill="#2b6cb0">title = "Hello World"</text>
  <text x="240" y="80" font-family="monospace" font-size="10" fill="#2b6cb0">date = "2025-01-15..."</text>
  <text x="240" y="100" font-family="monospace" font-size="10" fill="#2b6cb0">author = "Jane Doe"</text>
  <text x="240" y="120" font-family="monospace" font-size="10" fill="#2b6cb0">tags = ["intro"]</text>
  <text x="240" y="145" font-family="monospace" font-size="10" fill="#2b6cb0">[extra]</text>
  <text x="240" y="165" font-family="monospace" font-size="10" fill="#2b6cb0">reading_time = "5 min"</text>

  <!-- Arrows to LoadedContent -->
  <line x1="210" y1="100" x2="460" y2="80" stroke="#666" stroke-width="2" marker-end="url(#arrow2)"/>
  <line x1="430" y1="100" x2="460" y2="100" stroke="#666" stroke-width="2" marker-end="url(#arrow2)"/>

  <!-- LoadedContent struct -->
  <rect x="470" y="10" width="220" height="180" rx="6" fill="#faf5ff" stroke="#9f7aea" stroke-width="2"/>
  <rect x="470" y="10" width="220" height="30" rx="6" fill="#9f7aea"/>
  <text x="580" y="30" text-anchor="middle" font-family="system-ui" font-size="12" font-weight="bold" fill="white">LoadedContent</text>

  <text x="485" y="60" font-family="monospace" font-size="10" fill="#553c9a">html: String</text>
  <text x="570" y="60" font-family="system-ui" font-size="9" fill="#718096">← rendered markdown</text>

  <text x="485" y="82" font-family="monospace" font-size="10" fill="#553c9a">meta: ContentMeta</text>
  <text x="605" y="82" font-family="system-ui" font-size="9" fill="#718096">← parsed TOML</text>

  <text x="485" y="104" font-family="monospace" font-size="10" fill="#553c9a">excerpt: String</text>
  <text x="580" y="104" font-family="system-ui" font-size="9" fill="#718096">← ## Context</text>

  <text x="485" y="126" font-family="monospace" font-size="10" fill="#553c9a">content_type: String</text>
  <text x="615" y="126" font-family="system-ui" font-size="9" fill="#718096">← directory</text>

  <text x="485" y="148" font-family="monospace" font-size="10" fill="#553c9a">output_path: PathBuf</text>
  <text x="615" y="148" font-family="system-ui" font-size="9" fill="#718096">← computed</text>

  <!-- Directory context -->
  <rect x="10" y="210" width="680" height="60" rx="4" fill="#f0fff4" stroke="#48bb78" stroke-width="1"/>
  <text x="20" y="235" font-family="system-ui" font-size="11" fill="#276749" font-weight="bold">Directory structure determines content_type:</text>
  <text x="20" y="255" font-family="monospace" font-size="10" fill="#276749">content/articles/hello-world.md → content_type = "articles"</text>
</svg>

## URL Pattern System

The output path is computed from the `url_pattern` configuration:

<svg viewBox="0 0 700 300" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <marker id="arrow3" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#666"/>
    </marker>
  </defs>

  <!-- Input -->
  <rect x="10" y="10" width="300" height="100" rx="6" fill="#ebf8ff" stroke="#4299e1" stroke-width="2"/>
  <text x="160" y="35" text-anchor="middle" font-family="system-ui" font-size="12" font-weight="bold" fill="#2b6cb0">Input</text>
  <text x="20" y="58" font-family="monospace" font-size="10" fill="#2d3748">file: content/articles/hello-world.md</text>
  <text x="20" y="75" font-family="monospace" font-size="10" fill="#2d3748">meta.date: 2025-01-15T10:00:00Z</text>
  <text x="20" y="92" font-family="monospace" font-size="10" fill="#2d3748">url_pattern: "{{date}}-{{stem}}"</text>

  <!-- Arrow -->
  <line x1="310" y1="60" x2="380" y2="60" stroke="#666" stroke-width="2" marker-end="url(#arrow3)"/>

  <!-- Placeholders -->
  <rect x="390" y="10" width="300" height="100" rx="6" fill="#faf5ff" stroke="#9f7aea" stroke-width="2"/>
  <text x="540" y="35" text-anchor="middle" font-family="system-ui" font-size="12" font-weight="bold" fill="#553c9a">Placeholder Resolution</text>

  <text x="400" y="55" font-family="monospace" font-size="10" fill="#553c9a">{{stem}}</text>
  <text x="480" y="55" font-family="system-ui" font-size="10" fill="#2d3748">→ hello-world</text>
  <text x="400" y="72" font-family="monospace" font-size="10" fill="#553c9a">{{date}}</text>
  <text x="480" y="72" font-family="system-ui" font-size="10" fill="#2d3748">→ 2025-01-15</text>
  <text x="400" y="89" font-family="monospace" font-size="10" fill="#553c9a">{{year}}/{{month}}/{{day}}</text>
  <text x="560" y="89" font-family="system-ui" font-size="10" fill="#2d3748">→ 2025/01/15</text>

  <!-- Output examples -->
  <rect x="10" y="130" width="680" height="160" rx="6" fill="#f0fff4" stroke="#48bb78" stroke-width="2"/>
  <text x="350" y="155" text-anchor="middle" font-family="system-ui" font-size="12" font-weight="bold" fill="#276749">Output Examples</text>

  <!-- Table header -->
  <text x="30" y="180" font-family="system-ui" font-size="10" font-weight="bold" fill="#276749">url_pattern</text>
  <text x="220" y="180" font-family="system-ui" font-size="10" font-weight="bold" fill="#276749">clean_urls</text>
  <text x="320" y="180" font-family="system-ui" font-size="10" font-weight="bold" fill="#276749">Output Path</text>

  <line x1="20" y1="188" x2="680" y2="188" stroke="#48bb78" stroke-width="1"/>

  <!-- Table rows -->
  <text x="30" y="205" font-family="monospace" font-size="9" fill="#2d3748">{{stem}}</text>
  <text x="220" y="205" font-family="system-ui" font-size="9" fill="#2d3748">false</text>
  <text x="320" y="205" font-family="monospace" font-size="9" fill="#2d3748">/articles/hello-world.html</text>

  <text x="30" y="222" font-family="monospace" font-size="9" fill="#2d3748">{{stem}}</text>
  <text x="220" y="222" font-family="system-ui" font-size="9" fill="#2d3748">true</text>
  <text x="320" y="222" font-family="monospace" font-size="9" fill="#2d3748">/articles/hello-world/index.html</text>

  <text x="30" y="239" font-family="monospace" font-size="9" fill="#2d3748">{{date}}-{{stem}}</text>
  <text x="220" y="239" font-family="system-ui" font-size="9" fill="#2d3748">true</text>
  <text x="320" y="239" font-family="monospace" font-size="9" fill="#2d3748">/articles/2025-01-15-hello-world/index.html</text>

  <text x="30" y="256" font-family="monospace" font-size="9" fill="#2d3748">{{date}}/{{stem}}</text>
  <text x="220" y="256" font-family="system-ui" font-size="9" fill="#2d3748">true</text>
  <text x="320" y="256" font-family="monospace" font-size="9" fill="#2d3748">/articles/2025-01-15/hello-world/index.html</text>

  <text x="30" y="273" font-family="monospace" font-size="9" fill="#2d3748">{{year}}/{{month}}/{{stem}}</text>
  <text x="220" y="273" font-family="system-ui" font-size="9" fill="#2d3748">true</text>
  <text x="320" y="273" font-family="monospace" font-size="9" fill="#2d3748">/articles/2025/01/hello-world/index.html</text>
</svg>

## Template Context

Templates receive variables from multiple sources:

<svg viewBox="0 0 700 320" xmlns="http://www.w3.org/2000/svg">
  <!-- Config source -->
  <rect x="10" y="10" width="200" height="120" rx="6" fill="#faf5ff" stroke="#9f7aea" stroke-width="2"/>
  <rect x="10" y="10" width="200" height="28" rx="6" fill="#9f7aea"/>
  <text x="110" y="29" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">site.toml</text>

  <text x="20" y="55" font-family="monospace" font-size="9" fill="#553c9a">config.site.title</text>
  <text x="20" y="70" font-family="monospace" font-size="9" fill="#553c9a">config.site.domain</text>
  <text x="20" y="85" font-family="monospace" font-size="9" fill="#553c9a">config.site.author</text>
  <text x="20" y="100" font-family="monospace" font-size="9" fill="#553c9a">config.dynamic.*</text>
  <text x="20" y="115" font-family="monospace" font-size="9" fill="#553c9a">config.content.*</text>

  <!-- Content source -->
  <rect x="10" y="145" width="200" height="165" rx="6" fill="#ebf8ff" stroke="#4299e1" stroke-width="2"/>
  <rect x="10" y="145" width="200" height="28" rx="6" fill="#4299e1"/>
  <text x="110" y="164" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">Content Item</text>

  <text x="20" y="190" font-family="monospace" font-size="9" fill="#2b6cb0">meta.title</text>
  <text x="20" y="205" font-family="monospace" font-size="9" fill="#2b6cb0">meta.date</text>
  <text x="20" y="220" font-family="monospace" font-size="9" fill="#2b6cb0">meta.author</text>
  <text x="20" y="235" font-family="monospace" font-size="9" fill="#2b6cb0">meta.tags</text>
  <text x="20" y="250" font-family="monospace" font-size="9" fill="#2b6cb0">meta.cover</text>
  <text x="20" y="265" font-family="monospace" font-size="9" fill="#2b6cb0">meta.extra.*</text>
  <text x="20" y="280" font-family="monospace" font-size="9" fill="#2b6cb0">meta.extra_js</text>
  <text x="20" y="295" font-family="monospace" font-size="9" fill="#2b6cb0">content | excerpt</text>

  <!-- Arrows -->
  <line x1="210" y1="70" x2="270" y2="120" stroke="#9f7aea" stroke-width="2"/>
  <line x1="210" y1="220" x2="270" y2="170" stroke="#4299e1" stroke-width="2"/>

  <!-- Template -->
  <rect x="280" y="60" width="200" height="200" rx="6" fill="#fff" stroke="#718096" stroke-width="2"/>
  <rect x="280" y="60" width="200" height="28" rx="6" fill="#718096"/>
  <text x="380" y="79" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">Template Context</text>

  <text x="290" y="105" font-family="system-ui" font-size="10" fill="#4a5568" font-weight="bold">Content templates:</text>
  <text x="300" y="122" font-family="monospace" font-size="9" fill="#4a5568">{{ meta.title }}</text>
  <text x="300" y="137" font-family="monospace" font-size="9" fill="#4a5568">{{ content | safe }}</text>
  <text x="300" y="152" font-family="monospace" font-size="9" fill="#4a5568">{{ config.site.* }}</text>

  <text x="290" y="175" font-family="system-ui" font-size="10" fill="#4a5568" font-weight="bold">Index templates also get:</text>
  <text x="300" y="192" font-family="monospace" font-size="9" fill="#4a5568">{{ contents }}</text>
  <text x="395" y="192" font-family="system-ui" font-size="8" fill="#718096">← this type</text>
  <text x="300" y="207" font-family="monospace" font-size="9" fill="#4a5568">{{ all_content }}</text>
  <text x="395" y="207" font-family="system-ui" font-size="8" fill="#718096">← all types</text>

  <text x="290" y="230" font-family="system-ui" font-size="10" fill="#4a5568" font-weight="bold">Filters:</text>
  <text x="300" y="247" font-family="monospace" font-size="9" fill="#4a5568">| safe | url | asset_hash</text>

  <!-- Arrow to output -->
  <line x1="480" y1="160" x2="530" y2="160" stroke="#666" stroke-width="2" marker-end="url(#arrow3)"/>

  <!-- Output -->
  <rect x="540" y="100" width="150" height="120" rx="6" fill="#f0fff4" stroke="#48bb78" stroke-width="2"/>
  <rect x="540" y="100" width="150" height="28" rx="6" fill="#48bb78"/>
  <text x="615" y="119" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">Output HTML</text>

  <text x="555" y="150" font-family="system-ui" font-size="9" fill="#276749">Fully rendered</text>
  <text x="555" y="165" font-family="system-ui" font-size="9" fill="#276749">static HTML page</text>
  <text x="555" y="185" font-family="monospace" font-size="8" fill="#276749">index.html</text>
  <text x="555" y="200" font-family="monospace" font-size="8" fill="#276749">{type}/slug/index.html</text>
</svg>

## Asset Hashing Flow

When `asset_hashing_enabled = true`, CSS and JS files get content-based hashes:

<svg viewBox="0 0 700 200" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <marker id="arrow4" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#666"/>
    </marker>
  </defs>

  <!-- Input file -->
  <rect x="10" y="30" width="140" height="60" rx="6" fill="#ed8936" stroke="#c05621" stroke-width="2"/>
  <text x="80" y="55" text-anchor="middle" font-family="monospace" font-size="10" fill="white">static/css/</text>
  <text x="80" y="72" text-anchor="middle" font-family="monospace" font-size="10" fill="white">style.css</text>

  <!-- Arrow -->
  <line x1="150" y1="60" x2="180" y2="60" stroke="#666" stroke-width="2" marker-end="url(#arrow4)"/>

  <!-- Hash computation -->
  <rect x="190" y="20" width="120" height="80" rx="6" fill="#faf5ff" stroke="#9f7aea" stroke-width="2"/>
  <text x="250" y="45" text-anchor="middle" font-family="system-ui" font-size="10" fill="#553c9a">BLAKE3 Hash</text>
  <text x="250" y="65" text-anchor="middle" font-family="monospace" font-size="10" fill="#553c9a">a1b2c3d4</text>
  <text x="250" y="85" text-anchor="middle" font-family="system-ui" font-size="8" fill="#718096">(8 chars)</text>

  <!-- Arrow -->
  <line x1="310" y1="60" x2="340" y2="60" stroke="#666" stroke-width="2" marker-end="url(#arrow4)"/>

  <!-- Output file -->
  <rect x="350" y="30" width="160" height="60" rx="6" fill="#48bb78" stroke="#276749" stroke-width="2"/>
  <text x="430" y="55" text-anchor="middle" font-family="monospace" font-size="10" fill="white">static/css/</text>
  <text x="430" y="72" text-anchor="middle" font-family="monospace" font-size="10" fill="white">style.a1b2c3d4.css</text>

  <!-- Arrow down -->
  <line x1="430" y1="90" x2="430" y2="115" stroke="#666" stroke-width="2" marker-end="url(#arrow4)"/>

  <!-- Manifest -->
  <rect x="350" y="125" width="160" height="60" rx="6" fill="#ebf8ff" stroke="#4299e1" stroke-width="2"/>
  <text x="430" y="148" text-anchor="middle" font-family="system-ui" font-size="10" fill="#2b6cb0">Asset Manifest</text>
  <text x="430" y="168" text-anchor="middle" font-family="monospace" font-size="8" fill="#2b6cb0">"css/style.css" →</text>
  <text x="430" y="180" text-anchor="middle" font-family="monospace" font-size="8" fill="#2b6cb0">"/static/css/style.a1b..."</text>

  <!-- Arrow to template -->
  <line x1="510" y1="155" x2="540" y2="155" stroke="#666" stroke-width="2" marker-end="url(#arrow4)"/>

  <!-- Template usage -->
  <rect x="550" y="100" width="140" height="110" rx="6" fill="#fff" stroke="#718096" stroke-width="2"/>
  <text x="620" y="125" text-anchor="middle" font-family="system-ui" font-size="10" fill="#4a5568" font-weight="bold">In Template</text>
  <text x="560" y="150" font-family="monospace" font-size="8" fill="#4a5568">{{ "css/style.css"</text>
  <text x="560" y="165" font-family="monospace" font-size="8" fill="#4a5568">   | asset_hash }}</text>
  <text x="560" y="190" font-family="system-ui" font-size="9" fill="#718096">↓ outputs</text>
  <text x="560" y="205" font-family="monospace" font-size="7" fill="#276749">/static/css/style.a1b2c3d4.css</text>
</svg>

## Build Pipeline Stages

<svg viewBox="0 0 700 380" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <marker id="arrow5" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#4299e1"/>
    </marker>
  </defs>

  <!-- Stage 1 -->
  <rect x="10" y="10" width="150" height="50" rx="6" fill="#4299e1" stroke="#2b6cb0" stroke-width="2"/>
  <text x="85" y="30" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">1. Parse Config</text>
  <text x="85" y="48" text-anchor="middle" font-family="system-ui" font-size="9" fill="white">site.toml → Config</text>

  <line x1="160" y1="35" x2="180" y2="35" stroke="#4299e1" stroke-width="2" marker-end="url(#arrow5)"/>

  <!-- Stage 2 -->
  <rect x="190" y="10" width="150" height="50" rx="6" fill="#4299e1" stroke="#2b6cb0" stroke-width="2"/>
  <text x="265" y="30" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">2. Copy Assets</text>
  <text x="265" y="48" text-anchor="middle" font-family="system-ui" font-size="9" fill="white">static/ → output/</text>

  <line x1="340" y1="35" x2="360" y2="35" stroke="#4299e1" stroke-width="2" marker-end="url(#arrow5)"/>

  <!-- Stage 3 -->
  <rect x="370" y="10" width="150" height="50" rx="6" fill="#4299e1" stroke="#2b6cb0" stroke-width="2"/>
  <text x="445" y="30" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">3. Load Content</text>
  <text x="445" y="48" text-anchor="middle" font-family="system-ui" font-size="9" fill="white">(parallel with Rayon)</text>

  <line x1="520" y1="35" x2="540" y2="35" stroke="#4299e1" stroke-width="2" marker-end="url(#arrow5)"/>

  <!-- Stage 4 -->
  <rect x="550" y="10" width="140" height="50" rx="6" fill="#4299e1" stroke="#2b6cb0" stroke-width="2"/>
  <text x="620" y="30" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">4. Render</text>
  <text x="620" y="48" text-anchor="middle" font-family="system-ui" font-size="9" fill="white">templates → HTML</text>

  <!-- Arrow down -->
  <line x1="620" y1="60" x2="620" y2="90" stroke="#4299e1" stroke-width="2" marker-end="url(#arrow5)"/>

  <!-- Stage 5 -->
  <rect x="550" y="100" width="140" height="50" rx="6" fill="#48bb78" stroke="#276749" stroke-width="2"/>
  <text x="620" y="120" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">5. Generate</text>
  <text x="620" y="138" text-anchor="middle" font-family="system-ui" font-size="9" fill="white">sitemap + RSS</text>

  <line x1="550" y1="125" x2="520" y2="125" stroke="#48bb78" stroke-width="2" marker-end="url(#arrow5)"/>

  <!-- Stage 6 -->
  <rect x="370" y="100" width="140" height="50" rx="6" fill="#48bb78" stroke="#276749" stroke-width="2"/>
  <text x="440" y="120" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">6. Redirects</text>
  <text x="440" y="138" text-anchor="middle" font-family="system-ui" font-size="9" fill="white">(if configured)</text>

  <line x1="370" y1="125" x2="340" y2="125" stroke="#48bb78" stroke-width="2" marker-end="url(#arrow5)"/>

  <!-- Stage 7 -->
  <rect x="190" y="100" width="140" height="50" rx="6" fill="#48bb78" stroke="#276749" stroke-width="2"/>
  <text x="260" y="120" text-anchor="middle" font-family="system-ui" font-size="11" font-weight="bold" fill="white">7. Write Output</text>
  <text x="260" y="138" text-anchor="middle" font-family="system-ui" font-size="9" fill="white">all files to disk</text>

  <!-- Details boxes -->
  <rect x="10" y="180" width="220" height="90" rx="4" fill="#ebf8ff" stroke="#4299e1" stroke-width="1"/>
  <text x="120" y="200" text-anchor="middle" font-family="system-ui" font-size="10" font-weight="bold" fill="#2b6cb0">Stage 3: Load Content</text>
  <text x="20" y="220" font-family="system-ui" font-size="9" fill="#2d3748">For each .md file (in parallel):</text>
  <text x="30" y="235" font-family="system-ui" font-size="8" fill="#4a5568">• Read .meta.toml → ContentMeta</text>
  <text x="30" y="248" font-family="system-ui" font-size="8" fill="#4a5568">• Read .md → convert to HTML</text>
  <text x="30" y="261" font-family="system-ui" font-size="8" fill="#4a5568">• Extract excerpt from ## Context</text>

  <rect x="240" y="180" width="220" height="90" rx="4" fill="#ebf8ff" stroke="#4299e1" stroke-width="1"/>
  <text x="350" y="200" text-anchor="middle" font-family="system-ui" font-size="10" font-weight="bold" fill="#2b6cb0">Stage 4: Render Templates</text>
  <text x="250" y="220" font-family="system-ui" font-size="9" fill="#2d3748">For each content type:</text>
  <text x="260" y="235" font-family="system-ui" font-size="8" fill="#4a5568">• Render index → {type}/index.html</text>
  <text x="260" y="248" font-family="system-ui" font-size="8" fill="#4a5568">• For each item:</text>
  <text x="270" y="261" font-family="system-ui" font-size="8" fill="#4a5568">→ Render content template</text>

  <rect x="470" y="180" width="220" height="90" rx="4" fill="#f0fff4" stroke="#48bb78" stroke-width="1"/>
  <text x="580" y="200" text-anchor="middle" font-family="system-ui" font-size="10" font-weight="bold" fill="#276749">Stage 5: Generate</text>
  <text x="480" y="220" font-family="system-ui" font-size="9" fill="#2d3748">If enabled:</text>
  <text x="490" y="235" font-family="system-ui" font-size="8" fill="#4a5568">• sitemap.xml (all URLs)</text>
  <text x="490" y="248" font-family="system-ui" font-size="8" fill="#4a5568">• feed.xml (RSS 2.0)</text>
  <text x="490" y="261" font-family="system-ui" font-size="8" fill="#4a5568">• asset-manifest.json (if hashing)</text>

  <!-- Output summary -->
  <rect x="10" y="285" width="680" height="85" rx="6" fill="#f7fafc" stroke="#e2e8f0" stroke-width="1"/>
  <text x="350" y="310" text-anchor="middle" font-family="system-ui" font-size="12" font-weight="bold" fill="#2d3748">Final Output Structure</text>
  <text x="30" y="335" font-family="monospace" font-size="9" fill="#4a5568">output/</text>
  <text x="30" y="350" font-family="monospace" font-size="9" fill="#4a5568">├── index.html          ├── {type}/index.html       ├── sitemap.xml      ├── static/</text>
  <text x="30" y="365" font-family="monospace" font-size="9" fill="#4a5568">                        └── {type}/{slug}/index.html └── feed.xml         └── (hashed assets)</text>
</svg>
