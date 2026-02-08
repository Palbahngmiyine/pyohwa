use std::path::Path;

use crate::build::incremental;
use crate::config;
use crate::content::frontmatter;
use crate::content::loader;
use crate::content::page::RenderedContent;
use crate::error::BuildError;
use crate::markdown::highlight;
use crate::markdown::parser;
use crate::render::assets;
use crate::render::template;
use crate::site::graph;
use crate::site::route::Route;

/// Execute the full 8-stage build pipeline (production).
pub fn build(project_root: &Path) -> Result<(), BuildError> {
    let (output_pages, output_dir, static_dir) = build_internal(project_root, None)?;

    crate::build::output::write_output(&output_pages, &output_dir)?;

    if static_dir.exists() {
        assets::copy_static_assets(&static_dir, &output_dir)?;
    }

    Ok(())
}

/// Execute the build pipeline with live reload JS injected.
/// Used by the dev server for initial build.
pub fn build_dev(project_root: &Path, ws_port: u16) -> Result<(), BuildError> {
    let (output_pages, output_dir, static_dir) = build_internal(project_root, Some(ws_port))?;

    crate::build::output::write_output(&output_pages, &output_dir)?;

    if static_dir.exists() {
        assets::copy_static_assets(&static_dir, &output_dir)?;
    }

    Ok(())
}

/// Incremental dev build: detect changes via manifest, rebuild if needed.
/// Returns `true` if the site was rebuilt, `false` if no changes detected.
pub fn build_dev_incremental(project_root: &Path, ws_port: u16) -> Result<bool, BuildError> {
    let config = config::load(project_root)?;
    let content_dir = project_root.join(&config.build.content_dir);

    if !content_dir.exists() {
        return Err(BuildError::ContentDirNotFound(content_dir));
    }

    let raw_contents = loader::discover(&content_dir)?;
    let old_manifest = incremental::load_manifest(project_root);
    let (changed, new_manifest) = incremental::detect_changes(&raw_contents, &old_manifest);

    if changed.is_empty() {
        return Ok(false);
    }

    // Changes detected — full rebuild (site graph depends on all pages)
    let (output_pages, output_dir, static_dir) = build_internal(project_root, Some(ws_port))?;

    crate::build::output::write_output_incremental(&output_pages, &output_dir)?;

    if static_dir.exists() {
        assets::copy_static_assets(&static_dir, &output_dir)?;
    }

    incremental::save_manifest(project_root, &new_manifest)?;

    Ok(true)
}

/// Internal: run stages 1–7, returning rendered pages and paths.
fn build_internal(
    project_root: &Path,
    ws_port: Option<u16>,
) -> Result<(Vec<(Route, String)>, std::path::PathBuf, std::path::PathBuf), BuildError> {
    // Stage 1: Load config
    let config = config::load(project_root)?;

    let content_dir = project_root.join(&config.build.content_dir);
    let output_dir = project_root.join(&config.build.output_dir);
    let static_dir = project_root.join(&config.build.static_dir);

    if !content_dir.exists() {
        return Err(BuildError::ContentDirNotFound(content_dir));
    }

    // Stage 2: Discover content files (IO)
    let raw_contents = loader::discover(&content_dir)?;

    // Stage 3: Parse frontmatter (pure)
    let parsed_contents: Vec<_> = raw_contents
        .iter()
        .map(|raw| frontmatter::parse_frontmatter(raw))
        .collect::<Result<Vec<_>, _>>()?;

    // Stage 4: Markdown -> HTML (pure)
    let rendered_contents: Vec<_> = parsed_contents
        .iter()
        .map(|parsed| parser::parse_markdown(parsed))
        .collect::<Result<Vec<_>, _>>()?;

    // Stage 5: Syntax highlighting (pure)
    let highlighted_contents: Vec<RenderedContent> = rendered_contents
        .iter()
        .map(|rendered| highlight::apply_syntax_highlighting(rendered))
        .collect::<Result<Vec<_>, _>>()?;

    // Stage 6: Build site graph (pure)
    let site_graph =
        graph::build_graph_with_content_dir(&highlighted_contents, &config, &content_dir);

    // Stage 7: Render HTML templates (pure)
    let output_pages: Vec<_> = site_graph
        .pages
        .iter()
        .map(|page| {
            let html = match ws_port {
                Some(port) => {
                    template::render_page_with_live_reload(page, &site_graph, &config, port)?
                }
                None => template::render_page(page, &site_graph, &config)?,
            };
            Ok((page.route.clone(), html))
        })
        .collect::<Result<Vec<_>, BuildError>>()?;

    Ok((output_pages, output_dir, static_dir))
}
