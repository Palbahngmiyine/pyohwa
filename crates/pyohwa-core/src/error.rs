use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContentError {
    #[error("empty content at {path}")]
    EmptyContent { path: PathBuf },

    #[error("missing frontmatter in {path}")]
    MissingFrontmatter { path: PathBuf },

    #[error("invalid frontmatter in {path}: {reason}")]
    InvalidFrontmatter { path: PathBuf, reason: String },

    #[error("missing required field 'title' in {path}")]
    MissingTitle { path: PathBuf },
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to read config file {path}: {source}")]
    ReadError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse config file {path}: {reason}")]
    ParseError { path: PathBuf, reason: String },
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("template error: {0}")]
    Template(String),

    #[error("layout not found: {0}")]
    LayoutNotFound(String),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("content directory not found: {0}")]
    ContentDirNotFound(PathBuf),

    #[error("config error: {0}")]
    Config(#[from] ConfigError),

    #[error("content error: {0}")]
    Content(#[from] ContentError),

    #[error("render error: {0}")]
    Render(#[from] RenderError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("search error: {0}")]
    Search(String),
}
