# Pyohwa(표화/標火)

Zero-config static site generator powered by Rust and Elm.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- **Zero Configuration** — Start writing docs immediately, no setup required
- **Fast Builds** — Rust-powered build engine for instant site generation
- **Live Reload** — WebSocket-based dev server reflects changes in real time
- **Built-in Search** — Client-side full-text search with `Ctrl+K` shortcut
- **SEO Ready** — Auto-generated sitemap.xml, Atom feed, and Open Graph tags
- **Syntax Highlighting** — 50+ languages supported via syntect
- **Responsive UI** — Mobile, tablet, and desktop layouts with dark mode
- **Single Binary** — No need to install Elm or Tailwind; everything is embedded

## Quick Start

```bash
# Install
cargo install pyohwa-cli

# Create a new project and start the dev server
pyohwa init my-docs
cd my-docs
pyohwa dev
```

Open [http://localhost:3000](http://localhost:3000) to see your site.

## Installation

### From source

```bash
git clone https://github.com/Palbahngmiyine/pyohwa.git
cd pyohwa
cargo build --release

# The binary is at target/release/pyohwa
```

### Using cargo install

```bash
cargo install pyohwa-cli
```

## Usage

### `pyohwa init <name>`

Create a new project with default configuration and sample content.

```bash
pyohwa init my-docs
```

### `pyohwa build`

Build the static site into the `dist/` directory.

```bash
pyohwa build
pyohwa build --root ./my-project
```

| Option | Default | Description |
|--------|---------|-------------|
| `-r, --root` | `.` | Project root directory |

### `pyohwa dev`

Start a development server with live reload.

```bash
pyohwa dev
pyohwa dev --port 8080 --open
```

| Option | Default | Description |
|--------|---------|-------------|
| `-r, --root` | `.` | Project root directory |
| `-p, --port` | `3000` | Port to serve on |
| `--open` | `false` | Open browser automatically |

## Configuration

Pyohwa uses a `pyohwa.toml` file at the project root. All fields are optional — sensible defaults are applied automatically.

### Minimal

```toml
[site]
title = "My Documentation"
```

### Full example

```toml
[site]
title = "My Documentation"
description = "Project documentation powered by Pyohwa"
base_url = "/"
language = "en"

[build]
content_dir = "content"
output_dir = "dist"
static_dir = "static"

[theme]
name = "default"
highlight_theme = "one-dark"
# custom_css = "custom.css"

[[nav]]
text = "Guide"
link = "/guide/getting-started"

[sidebar]
auto = true

[search]
enabled = true

[seo]
sitemap = true
rss = false
# og_image = "og.png"
```

### Configuration reference

| Section | Key | Default | Description |
|---------|-----|---------|-------------|
| `site` | `title` | `"Documentation"` | Site title |
| `site` | `description` | `""` | Site description |
| `site` | `base_url` | `"/"` | Base URL path |
| `site` | `language` | `"en"` | Language code |
| `build` | `content_dir` | `"content"` | Markdown source directory |
| `build` | `output_dir` | `"dist"` | Build output directory |
| `build` | `static_dir` | `"static"` | Static assets directory |
| `theme` | `name` | `"default"` | Theme name |
| `theme` | `highlight_theme` | `"one-dark"` | Syntax highlight theme |
| `theme` | `custom_css` | — | Path to custom CSS file |
| `sidebar` | `auto` | `true` | Auto-generate sidebar from file tree |
| `search` | `enabled` | `true` | Enable client-side search |
| `seo` | `sitemap` | `true` | Generate sitemap.xml |
| `seo` | `rss` | `false` | Generate Atom feed (feed.xml) |
| `seo` | `og_image` | — | Default Open Graph image path |

## Writing Content

### Directory structure

```
my-docs/
├── pyohwa.toml
├── content/
│   ├── index.md          # Home page
│   ├── guide/
│   │   ├── getting-started.md
│   │   └── configuration.md
│   └── api/
│       └── reference.md
└── static/               # Copied as-is to dist/
    └── logo.png
```

### Frontmatter

Every Markdown file requires a YAML frontmatter block with at least a `title` field:

```markdown
---
title: "Getting Started"
description: "Learn how to set up Pyohwa"
layout: doc
order: 1
tags:
  - guide
  - setup
date: "2025-01-01"
draft: false
---

Your content here...
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | string | **(required)** | Page title |
| `description` | string | — | Page description (used in SEO meta tags) |
| `layout` | string | `"doc"` | Layout type: `doc`, `home`, `page`, or custom |
| `order` | integer | — | Sort order in sidebar |
| `tags` | list | `[]` | Tags for categorization |
| `date` | string | — | Publication date |
| `draft` | boolean | `false` | Exclude from build when `true` |
| `prev` | string | — | Custom previous page link |
| `next` | string | — | Custom next page link |

### File-based routing

Files in the `content/` directory map directly to URL paths:

| File | URL |
|------|-----|
| `content/index.md` | `/` |
| `content/guide/getting-started.md` | `/guide/getting-started/` |
| `content/api/reference.md` | `/api/reference/` |

## Project Structure (build output)

After running `pyohwa build`, the `dist/` directory contains the complete static site:

```
dist/
├── index.html
├── guide/
│   └── getting-started/
│       └── index.html
├── assets/
│   ├── app.js
│   └── style.css
├── search-index.json
├── sitemap.xml
└── feed.xml
```

## License

[MIT](LICENSE)
