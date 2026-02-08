use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::ConfigError;
use crate::site::graph::{NavItem, SidebarGroup};

/// Load config from pyohwa.toml at project root.
/// Returns default Config if file does not exist.
pub fn load(project_root: &Path) -> Result<Config, ConfigError> {
    let config_path = project_root.join("pyohwa.toml");
    if !config_path.exists() {
        return Ok(Config::default());
    }
    let content = std::fs::read_to_string(&config_path).map_err(|e| ConfigError::ReadError {
        path: config_path.clone(),
        source: e,
    })?;
    let config: Config = toml::from_str(&content).map_err(|e| ConfigError::ParseError {
        path: config_path,
        reason: e.to_string(),
    })?;
    Ok(config)
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub site: SiteConfig,
    pub build: BuildConfig,
    pub theme: ThemeConfig,
    pub nav: Vec<NavItem>,
    pub sidebar: SidebarConfig,
    pub search: SearchConfig,
    pub seo: SeoConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            site: SiteConfig::default(),
            build: BuildConfig::default(),
            theme: ThemeConfig::default(),
            nav: Vec::new(),
            sidebar: SidebarConfig::default(),
            search: SearchConfig::default(),
            seo: SeoConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub base_url: String,
    pub language: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "Documentation".to_string(),
            description: String::new(),
            base_url: "/".to_string(),
            language: "en".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct BuildConfig {
    pub content_dir: PathBuf,
    pub output_dir: PathBuf,
    pub static_dir: PathBuf,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            content_dir: PathBuf::from("content"),
            output_dir: PathBuf::from("dist"),
            static_dir: PathBuf::from("static"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    pub name: String,
    pub highlight_theme: String,
    pub custom_css: Option<PathBuf>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            highlight_theme: "one-dark".to_string(),
            custom_css: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SidebarConfig {
    pub auto: bool,
    pub groups: Vec<SidebarGroup>,
}

impl Default for SidebarConfig {
    fn default() -> Self {
        Self {
            auto: true,
            groups: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SearchConfig {
    pub enabled: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SeoConfig {
    pub sitemap: bool,
    pub rss: bool,
    pub og_image: Option<String>,
}

impl Default for SeoConfig {
    fn default() -> Self {
        Self {
            sitemap: true,
            rss: false,
            og_image: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn missing_file_returns_default() {
        let tmp = tempfile::tempdir().unwrap();
        let config = load(tmp.path()).unwrap();
        assert_eq!(config.site.title, "Documentation");
        assert_eq!(config.build.content_dir, PathBuf::from("content"));
        assert!(config.sidebar.auto);
    }

    #[test]
    fn partial_config_fills_defaults() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(
            tmp.path().join("pyohwa.toml"),
            r#"
[site]
title = "My Docs"
"#,
        )
        .unwrap();
        let config = load(tmp.path()).unwrap();
        assert_eq!(config.site.title, "My Docs");
        assert_eq!(config.site.language, "en");
        assert_eq!(config.build.output_dir, PathBuf::from("dist"));
    }

    #[test]
    fn invalid_toml_returns_parse_error() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("pyohwa.toml"), "invalid = [[[").unwrap();
        let err = load(tmp.path()).unwrap_err();
        assert!(matches!(err, ConfigError::ParseError { .. }));
    }

    #[test]
    fn full_config_parses_all_fields() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(
            tmp.path().join("pyohwa.toml"),
            r#"
[site]
title = "Full Site"
description = "A full test"
base_url = "/docs/"
language = "ko"

[build]
content_dir = "src"
output_dir = "build"
static_dir = "public"

[theme]
name = "custom"
highlight_theme = "monokai"

[search]
enabled = false

[seo]
sitemap = false
rss = true
og_image = "og.png"
"#,
        )
        .unwrap();
        let config = load(tmp.path()).unwrap();
        assert_eq!(config.site.title, "Full Site");
        assert_eq!(config.site.language, "ko");
        assert_eq!(config.build.content_dir, PathBuf::from("src"));
        assert_eq!(config.theme.highlight_theme, "monokai");
        assert!(!config.search.enabled);
        assert!(config.seo.rss);
        assert_eq!(config.seo.og_image, Some("og.png".to_string()));
    }
}
