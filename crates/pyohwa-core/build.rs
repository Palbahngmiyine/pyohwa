use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let project_root = Path::new(&manifest_dir).join("../.."); // workspace root

    // Rerun if source assets change
    println!("cargo:rerun-if-changed=../../themes/default");
    println!("cargo:rerun-if-changed=../../elm/src");
    println!("cargo:rerun-if-changed=../../elm/elm.json");

    // --- Elm JS ---
    let elm_out = Path::new(&out_dir).join("elm.min.js");
    let elm_src = project_root.join("elm/src/Main.elm");
    let elm_dist = project_root.join("elm/dist/elm.min.js");

    if elm_src.exists() {
        // Try to compile Elm from source
        if let Some(elm_bin) = find_executable("elm") {
            let elm_dir = project_root.join("elm");
            let status = Command::new(&elm_bin)
                .current_dir(&elm_dir)
                .args(["make", "src/Main.elm", "--optimize"])
                .arg(format!("--output={}", elm_out.display()))
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("cargo:warning=Elm: compiled from source (--optimize)");
                    // Also update elm/dist/ for git tracking
                    let _ = fs::copy(&elm_out, &elm_dist);
                }
                _ => {
                    println!("cargo:warning=Elm: compilation failed, using pre-built fallback");
                    copy_elm_fallback(&elm_dist, &elm_out);
                }
            }
        } else {
            println!("cargo:warning=Elm: compiler not found, using pre-built fallback");
            copy_elm_fallback(&elm_dist, &elm_out);
        }
    } else {
        copy_elm_fallback(&elm_dist, &elm_out);
    }

    // --- Theme CSS ---
    let theme_out = Path::new(&out_dir).join("theme.css");
    let theme_source = project_root.join("themes/default/theme.css");
    let theme_dist = project_root.join("themes/default/dist/theme.css");

    // Try Tailwind CLI if available
    if theme_source.exists() {
        if let Some(tw_bin) = find_tailwind() {
            let status = Command::new(&tw_bin)
                .args([
                    "--input",
                    &theme_source.display().to_string(),
                    "--output",
                    &theme_out.display().to_string(),
                    "--minify",
                ])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("cargo:warning=Tailwind: compiled from source (--minify)");
                    // Update dist/ for git tracking
                    let _ = fs::copy(&theme_out, &theme_dist);
                    return;
                }
                _ => {
                    println!(
                        "cargo:warning=Tailwind: compilation failed, using pre-built fallback"
                    );
                }
            }
        }
    }

    // Fallback: copy pre-built CSS
    if theme_dist.exists() {
        fs::copy(&theme_dist, &theme_out).expect("Failed to copy theme.css");
    } else if theme_source.exists() {
        fs::copy(&theme_source, &theme_out).expect("Failed to copy theme source");
    } else {
        fs::write(&theme_out, "/* No theme CSS found */").expect("Failed to write placeholder");
    }
}

fn copy_elm_fallback(elm_dist: &Path, elm_out: &Path) {
    if elm_dist.exists() {
        fs::copy(elm_dist, elm_out).expect("Failed to copy elm.min.js");
    } else {
        fs::write(
            elm_out,
            "// Elm app not available - install elm compiler or provide elm/dist/elm.min.js",
        )
        .expect("Failed to write elm placeholder");
    }
}

fn find_executable(name: &str) -> Option<String> {
    Command::new("which")
        .arg(name)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn find_tailwind() -> Option<String> {
    // Try common Tailwind CLI names
    for name in &["tailwindcss", "npx tailwindcss"] {
        if let Some(path) = find_executable(name) {
            return Some(path);
        }
    }
    None
}
