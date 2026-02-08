use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let project_root = Path::new(&manifest_dir).join("../.."); // workspace root

    // Rerun if source assets change
    println!("cargo:rerun-if-changed=../../themes/default");
    println!("cargo:rerun-if-changed=../../elm/src");

    // --- Theme CSS ---
    // Phase 2+: Tailwind CLI builds from source → OUT_DIR
    // Phase 1: copy pre-built CSS from themes/default/dist/
    let theme_out = Path::new(&out_dir).join("theme.css");
    let theme_source = project_root.join("themes/default/theme.css");
    let theme_dist = project_root.join("themes/default/dist/theme.css");

    if theme_dist.exists() {
        fs::copy(&theme_dist, &theme_out).expect("Failed to copy theme.css");
    } else if theme_source.exists() {
        // Fallback: use the Tailwind source CSS as-is (won't have utility classes resolved)
        fs::copy(&theme_source, &theme_out).expect("Failed to copy theme source");
    } else {
        fs::write(&theme_out, "/* No theme CSS found */").expect("Failed to write placeholder");
    }

    // --- Elm JS ---
    // Phase 2+: Elm compiler builds from source → OUT_DIR
    // Phase 1: copy pre-built JS or use placeholder
    let elm_out = Path::new(&out_dir).join("elm.min.js");
    let elm_dist = project_root.join("elm/dist/elm.min.js");

    if elm_dist.exists() {
        fs::copy(&elm_dist, &elm_out).expect("Failed to copy elm.min.js");
    } else {
        fs::write(
            &elm_out,
            "// Elm app placeholder - will be compiled from elm/src/ in Phase 2",
        )
        .expect("Failed to write elm placeholder");
    }
}
