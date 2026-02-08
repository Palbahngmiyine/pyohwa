use std::path::PathBuf;

use super::frontmatter::Frontmatter;
use crate::site::route::Route;

/// Stage 2 output: raw file content loaded from disk
#[derive(Debug, Clone)]
pub struct RawContent {
    pub path: PathBuf,
    pub raw: String,
}

/// Stage 3 output: frontmatter parsed, body separated
#[derive(Debug, Clone)]
pub struct ParsedContent {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub body: String,
}

/// TOC item extracted from headings
#[derive(Debug, Clone, serde::Serialize)]
pub struct TocItem {
    pub id: String,
    pub text: String,
    pub level: u8,
}

/// Stage 4-5 output: Markdown rendered to HTML with TOC
#[derive(Debug, Clone)]
pub struct RenderedContent {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub html: String,
    pub toc: Vec<TocItem>,
}

/// Final page representation used in site graph
#[derive(Debug, Clone)]
pub struct Page {
    pub route: Route,
    pub frontmatter: Frontmatter,
    pub html: String,
    pub toc: Vec<TocItem>,
    pub prev: Option<Route>,
    pub next: Option<Route>,
}
