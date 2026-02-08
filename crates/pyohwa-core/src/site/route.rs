use std::path::{Path, PathBuf};

/// A route maps a content file path to a URL path
#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    /// URL path (e.g., "/guide/getting-started")
    pub path: String,
    /// Original file path relative to content dir
    pub source: PathBuf,
    /// Output file path relative to dist dir
    pub output: PathBuf,
}

/// Resolve a file path to a Route.
///
/// Converts a content file path into a URL path:
/// - `content/guide/getting-started.md` -> `/guide/getting-started`
/// - `content/index.md` -> `/`
/// - `content/guide/index.md` -> `/guide/`
pub fn resolve_route(content_dir: &Path, file_path: &Path) -> Route {
    let relative = file_path
        .strip_prefix(content_dir)
        .unwrap_or(file_path);

    let url_path = build_url_path(relative);
    let output = build_output_path(relative);

    Route {
        path: url_path,
        source: relative.to_path_buf(),
        output,
    }
}

fn build_url_path(relative: &Path) -> String {
    let stem = relative
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let parent = relative.parent().unwrap_or_else(|| Path::new(""));

    if stem == "index" {
        if parent.as_os_str().is_empty() {
            return "/".to_string();
        }
        return format!("/{}/", normalize_path_separators(parent));
    }

    if parent.as_os_str().is_empty() {
        return format!("/{stem}");
    }

    format!("/{}/{}", normalize_path_separators(parent), stem)
}

fn build_output_path(relative: &Path) -> PathBuf {
    let stem = relative
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let parent = relative.parent().unwrap_or_else(|| Path::new(""));

    if stem == "index" {
        parent.join("index.html")
    } else {
        parent.join(stem).join("index.html")
    }
}

fn normalize_path_separators(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_str().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("/")
}

impl Route {
    /// Returns the URL path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the parent directory name from the source path
    pub fn parent_dir(&self) -> String {
        self.source
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string()
    }

    /// Returns a human-readable display name derived from the parent directory.
    /// Converts directory names like "getting-started" to "Getting Started".
    pub fn display_name(&self) -> String {
        let dir = self.parent_dir();
        if dir.is_empty() {
            return "Root".to_string();
        }

        let last_segment = Path::new(&dir)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&dir);

        last_segment
            .split('-')
            .map(capitalize_first)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => {
            let upper: String = c.to_uppercase().collect();
            format!("{upper}{}", chars.as_str())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_resolve_route_index() {
        let route = resolve_route(Path::new("content"), Path::new("content/index.md"));
        assert_eq!(route.path, "/");
        assert_eq!(route.output, PathBuf::from("index.html"));
    }

    #[test]
    fn test_resolve_route_nested_index() {
        let route = resolve_route(Path::new("content"), Path::new("content/guide/index.md"));
        assert_eq!(route.path, "/guide/");
        assert_eq!(route.output, PathBuf::from("guide/index.html"));
    }

    #[test]
    fn test_resolve_route_regular_page() {
        let route = resolve_route(
            Path::new("content"),
            Path::new("content/guide/getting-started.md"),
        );
        assert_eq!(route.path, "/guide/getting-started");
        assert_eq!(
            route.output,
            PathBuf::from("guide/getting-started/index.html")
        );
    }

    #[test]
    fn test_resolve_route_top_level_page() {
        let route = resolve_route(Path::new("content"), Path::new("content/about.md"));
        assert_eq!(route.path, "/about");
        assert_eq!(route.output, PathBuf::from("about/index.html"));
    }

    #[test]
    fn test_route_parent_dir() {
        let route = resolve_route(
            Path::new("content"),
            Path::new("content/guide/getting-started.md"),
        );
        assert_eq!(route.parent_dir(), "guide");
    }

    #[test]
    fn test_route_parent_dir_root() {
        let route = resolve_route(Path::new("content"), Path::new("content/index.md"));
        assert_eq!(route.parent_dir(), "");
    }

    #[test]
    fn test_route_display_name() {
        let route = resolve_route(
            Path::new("content"),
            Path::new("content/getting-started/intro.md"),
        );
        assert_eq!(route.display_name(), "Getting Started");
    }

    #[test]
    fn test_route_display_name_root() {
        let route = resolve_route(Path::new("content"), Path::new("content/index.md"));
        assert_eq!(route.display_name(), "Root");
    }
}
