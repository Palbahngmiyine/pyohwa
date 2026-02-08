/// Elm app (minified) — built by build.rs from elm/src/ (Phase 2+)
pub const ELM_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/elm.min.js"));

/// Theme CSS — built by build.rs from themes/default/ (Tailwind v4 in Phase 2+)
pub const THEME_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/theme.css"));
