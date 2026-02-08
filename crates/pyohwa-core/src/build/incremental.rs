use std::collections::HashMap;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::content::page::RawContent;
use crate::error::BuildError;

/// Maps file paths to their SHA-256 content hashes.
pub type BuildManifest = HashMap<PathBuf, String>;

const MANIFEST_DIR: &str = ".pyohwa";
const MANIFEST_FILE: &str = "manifest.json";

/// Compute SHA-256 hash of content.
pub fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Load the build manifest from `.pyohwa/manifest.json`.
/// Returns an empty manifest if the file doesn't exist.
pub fn load_manifest(project_root: &Path) -> BuildManifest {
    let path = project_root.join(MANIFEST_DIR).join(MANIFEST_FILE);
    match std::fs::read_to_string(&path) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

/// Save the build manifest to `.pyohwa/manifest.json`.
pub fn save_manifest(project_root: &Path, manifest: &BuildManifest) -> Result<(), BuildError> {
    let dir = project_root.join(MANIFEST_DIR);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(MANIFEST_FILE);
    let data = serde_json::to_string_pretty(manifest)
        .map_err(|e| BuildError::Render(crate::error::RenderError::Serialization(e)))?;
    std::fs::write(&path, data)?;
    Ok(())
}

/// Compare raw contents against a previous manifest.
/// Returns `(changed_paths, new_manifest)`.
/// A file is considered changed if it's new or its hash differs.
pub fn detect_changes(
    raw_contents: &[RawContent],
    old_manifest: &BuildManifest,
) -> (Vec<PathBuf>, BuildManifest) {
    let mut new_manifest = BuildManifest::new();
    let mut changed = Vec::new();

    for raw in raw_contents {
        let hash = hash_content(&raw.raw);
        let old_hash = old_manifest.get(&raw.path);

        if old_hash != Some(&hash) {
            changed.push(raw.path.clone());
        }

        new_manifest.insert(raw.path.clone(), hash);
    }

    // Detect deleted files (in old but not in new)
    for old_path in old_manifest.keys() {
        if !new_manifest.contains_key(old_path) {
            changed.push(old_path.clone());
        }
    }

    (changed, new_manifest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic() {
        let h1 = hash_content("hello world");
        let h2 = hash_content("hello world");
        assert_eq!(h1, h2);
        assert!(!h1.is_empty());
    }

    #[test]
    fn hash_differs_for_different_content() {
        let h1 = hash_content("hello");
        let h2 = hash_content("world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn first_run_detects_all_as_changed() {
        let raw = vec![
            RawContent {
                path: PathBuf::from("a.md"),
                raw: "# A".to_string(),
            },
            RawContent {
                path: PathBuf::from("b.md"),
                raw: "# B".to_string(),
            },
        ];

        let (changed, manifest) = detect_changes(&raw, &HashMap::new());
        assert_eq!(changed.len(), 2);
        assert_eq!(manifest.len(), 2);
    }

    #[test]
    fn no_changes_detected_when_content_same() {
        let raw = vec![RawContent {
            path: PathBuf::from("a.md"),
            raw: "# A".to_string(),
        }];

        let mut old = HashMap::new();
        old.insert(PathBuf::from("a.md"), hash_content("# A"));

        let (changed, _) = detect_changes(&raw, &old);
        assert!(changed.is_empty());
    }

    #[test]
    fn detects_modified_file() {
        let raw = vec![RawContent {
            path: PathBuf::from("a.md"),
            raw: "# A modified".to_string(),
        }];

        let mut old = HashMap::new();
        old.insert(PathBuf::from("a.md"), hash_content("# A"));

        let (changed, _) = detect_changes(&raw, &old);
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0], PathBuf::from("a.md"));
    }

    #[test]
    fn detects_new_file() {
        let raw = vec![
            RawContent {
                path: PathBuf::from("a.md"),
                raw: "# A".to_string(),
            },
            RawContent {
                path: PathBuf::from("b.md"),
                raw: "# B".to_string(),
            },
        ];

        let mut old = HashMap::new();
        old.insert(PathBuf::from("a.md"), hash_content("# A"));

        let (changed, _) = detect_changes(&raw, &old);
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0], PathBuf::from("b.md"));
    }

    #[test]
    fn manifest_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        let mut manifest = BuildManifest::new();
        manifest.insert(PathBuf::from("a.md"), "abc123".to_string());
        manifest.insert(PathBuf::from("b.md"), "def456".to_string());

        save_manifest(root, &manifest).unwrap();
        let loaded = load_manifest(root);

        assert_eq!(loaded, manifest);
    }
}
