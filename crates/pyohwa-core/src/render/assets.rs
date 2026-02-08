use std::fs;
use std::path::Path;

use crate::error::BuildError;

/// Recursively copy static assets from static_dir to output_dir.
///
/// If static_dir does not exist, this is a no-op (not an error).
pub fn copy_static_assets(static_dir: &Path, output_dir: &Path) -> Result<(), BuildError> {
    if !static_dir.exists() {
        return Ok(());
    }

    copy_dir_recursive(static_dir, output_dir)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), BuildError> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    let entries = fs::read_dir(src)?;

    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_copy_static_assets() {
        let tmp = std::env::temp_dir().join("pyohwa_test_assets");
        let src = tmp.join("static");
        let dst = tmp.join("dist");

        // Clean up from previous runs
        let _ = fs::remove_dir_all(&tmp);

        // Create test files
        fs::create_dir_all(src.join("images")).unwrap();
        fs::write(src.join("favicon.ico"), "icon data").unwrap();
        fs::write(src.join("images/logo.png"), "png data").unwrap();

        copy_static_assets(&src, &dst).unwrap();

        assert!(dst.join("favicon.ico").exists());
        assert!(dst.join("images/logo.png").exists());
        assert_eq!(
            fs::read_to_string(dst.join("favicon.ico")).unwrap(),
            "icon data"
        );

        // Clean up
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_copy_nonexistent_static_dir() {
        let result = copy_static_assets(
            Path::new("/nonexistent/static"),
            Path::new("/tmp/pyohwa_test_nonexistent"),
        );
        assert!(result.is_ok());
    }
}
