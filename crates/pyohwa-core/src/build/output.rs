use std::fs;
use std::path::Path;

use crate::error::BuildError;
use crate::render::embedded;
use crate::site::route::Route;

/// Write rendered HTML files and embedded assets to the output directory
/// without cleaning the directory first. Used for incremental dev builds.
pub fn write_output_incremental(
    pages: &[(Route, String)],
    output_dir: &Path,
) -> Result<(), BuildError> {
    fs::create_dir_all(output_dir)?;

    for (route, html) in pages {
        let output_path = output_dir.join(&route.output);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&output_path, html)?;
    }

    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;
    fs::write(assets_dir.join("elm.min.js"), embedded::ELM_JS)?;
    fs::write(assets_dir.join("theme.css"), embedded::THEME_CSS)?;

    Ok(())
}

/// Write rendered HTML files and embedded assets to the output directory.
///
/// 1. Clean and recreate the output directory
/// 2. Write each HTML page at its route output path
/// 3. Extract embedded assets (elm.min.js, theme.css) to dist/assets/
pub fn write_output(
    pages: &[(Route, String)],
    output_dir: &Path,
) -> Result<(), BuildError> {
    // Clean output directory
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;

    // Write HTML pages
    for (route, html) in pages {
        let output_path = output_dir.join(&route.output);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&output_path, html)?;
    }

    // Write embedded assets
    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;
    fs::write(assets_dir.join("elm.min.js"), embedded::ELM_JS)?;
    fs::write(assets_dir.join("theme.css"), embedded::THEME_CSS)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_write_output_creates_files() {
        let tmp = std::env::temp_dir().join("pyohwa_test_output");
        let _ = fs::remove_dir_all(&tmp);

        let pages = vec![
            (
                Route {
                    path: "/".to_string(),
                    source: PathBuf::from("index.md"),
                    output: PathBuf::from("index.html"),
                },
                "<html>Home</html>".to_string(),
            ),
            (
                Route {
                    path: "/guide/intro".to_string(),
                    source: PathBuf::from("guide/intro.md"),
                    output: PathBuf::from("guide/intro/index.html"),
                },
                "<html>Guide</html>".to_string(),
            ),
        ];

        write_output(&pages, &tmp).unwrap();

        assert!(tmp.join("index.html").exists());
        assert!(tmp.join("guide/intro/index.html").exists());
        assert!(tmp.join("assets/elm.min.js").exists());
        assert!(tmp.join("assets/theme.css").exists());

        assert_eq!(
            fs::read_to_string(tmp.join("index.html")).unwrap(),
            "<html>Home</html>"
        );
        assert_eq!(
            fs::read_to_string(tmp.join("guide/intro/index.html")).unwrap(),
            "<html>Guide</html>"
        );

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_write_output_cleans_previous_build() {
        let tmp = std::env::temp_dir().join("pyohwa_test_output_clean");
        let _ = fs::remove_dir_all(&tmp);

        // Create a leftover file from a "previous build"
        fs::create_dir_all(&tmp).unwrap();
        fs::write(tmp.join("stale.html"), "old").unwrap();

        let pages = vec![(
            Route {
                path: "/".to_string(),
                source: PathBuf::from("index.md"),
                output: PathBuf::from("index.html"),
            },
            "<html>New</html>".to_string(),
        )];

        write_output(&pages, &tmp).unwrap();

        assert!(tmp.join("index.html").exists());
        assert!(!tmp.join("stale.html").exists());

        let _ = fs::remove_dir_all(&tmp);
    }
}
