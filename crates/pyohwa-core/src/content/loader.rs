use std::path::Path;

use walkdir::WalkDir;

use crate::content::page::RawContent;
use crate::error::ContentError;

/// Discover all `.md` files recursively under `content_dir`.
///
/// Returns an empty vec if the directory does not exist.
/// Draft filtering happens later, after frontmatter parsing.
pub fn discover(content_dir: &Path) -> Result<Vec<RawContent>, ContentError> {
    if !content_dir.exists() {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();

    for entry in WalkDir::new(content_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("md") {
            continue;
        }

        let raw = std::fs::read_to_string(path).map_err(|_| ContentError::EmptyContent {
            path: path.to_path_buf(),
        })?;

        results.push(RawContent {
            path: path.to_path_buf(),
            raw,
        });
    }

    results.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn discover_finds_md_files() {
        let tmp = tempfile::tempdir().unwrap();
        let content = tmp.path().join("content");
        fs::create_dir_all(content.join("guide")).unwrap();
        fs::write(content.join("index.md"), "# Home").unwrap();
        fs::write(content.join("guide/intro.md"), "# Intro").unwrap();
        fs::write(content.join("guide/notes.txt"), "not markdown").unwrap();

        let results = discover(&content).unwrap();
        assert_eq!(results.len(), 2);
        let paths: Vec<PathBuf> = results.iter().map(|r| r.path.clone()).collect();
        assert!(paths.iter().any(|p| p.ends_with("index.md")));
        assert!(paths.iter().any(|p| p.ends_with("intro.md")));
    }

    #[test]
    fn discover_returns_empty_for_missing_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let missing = tmp.path().join("nonexistent");
        let results = discover(&missing).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn discover_returns_empty_for_no_md_files() {
        let tmp = tempfile::tempdir().unwrap();
        let content = tmp.path().join("content");
        fs::create_dir_all(&content).unwrap();
        fs::write(content.join("readme.txt"), "text").unwrap();
        let results = discover(&content).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn discover_results_are_sorted() {
        let tmp = tempfile::tempdir().unwrap();
        let content = tmp.path().join("content");
        fs::create_dir_all(&content).unwrap();
        fs::write(content.join("z.md"), "# Z").unwrap();
        fs::write(content.join("a.md"), "# A").unwrap();
        fs::write(content.join("m.md"), "# M").unwrap();

        let results = discover(&content).unwrap();
        assert_eq!(results.len(), 3);
        assert!(results[0].path < results[1].path);
        assert!(results[1].path < results[2].path);
    }
}
