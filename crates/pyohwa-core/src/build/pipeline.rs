use std::path::{Path, PathBuf};

use crate::build::incremental;
use crate::config::{self, Config};
use crate::content::frontmatter;
use crate::content::loader;
use crate::content::page::{Page, RenderedContent};
use crate::error::BuildError;
use crate::markdown::highlight;
use crate::markdown::parser;
use crate::render::assets;
use crate::render::template;
use crate::site::graph::{self, SiteGraph};
use crate::site::route::Route;

/// Intermediate result from build_internal, holding all data needed for output.
struct BuildResult {
    output_pages: Vec<(Route, String)>,
    site_graph: SiteGraph,
    config: Config,
    output_dir: PathBuf,
    static_dir: PathBuf,
}

/// Execute the full build pipeline (production).
pub fn build(project_root: &Path) -> Result<(), BuildError> {
    let result = build_internal(project_root, None)?;

    crate::build::output::write_output(&result.output_pages, &result.output_dir)?;

    if result.static_dir.exists() {
        assets::copy_static_assets(&result.static_dir, &result.output_dir)?;
    }

    write_search_and_seo(&result)?;

    Ok(())
}

/// Execute the build pipeline with live reload JS injected.
/// Used by the dev server for initial build.
pub fn build_dev(project_root: &Path, ws_port: u16) -> Result<(), BuildError> {
    let result = build_internal(project_root, Some(ws_port))?;

    crate::build::output::write_output(&result.output_pages, &result.output_dir)?;

    if result.static_dir.exists() {
        assets::copy_static_assets(&result.static_dir, &result.output_dir)?;
    }

    write_search_and_seo(&result)?;

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
    let result = build_internal(project_root, Some(ws_port))?;

    crate::build::output::write_output_incremental(&result.output_pages, &result.output_dir)?;

    if result.static_dir.exists() {
        assets::copy_static_assets(&result.static_dir, &result.output_dir)?;
    }

    write_search_and_seo(&result)?;

    incremental::save_manifest(project_root, &new_manifest)?;

    Ok(true)
}

/// Convert Page types to pyohwa_search::PageData for search indexing.
fn pages_to_search_data(pages: &[Page]) -> Vec<pyohwa_search::PageData> {
    pages
        .iter()
        .map(|page| pyohwa_search::PageData {
            url: page.route.path().to_string(),
            title: page.frontmatter.title.clone(),
            description: page
                .frontmatter
                .description
                .clone()
                .unwrap_or_default(),
            html: page.html.clone(),
            tags: page.frontmatter.tags.clone(),
            date: page.frontmatter.date.clone(),
            draft: page.frontmatter.draft,
        })
        .collect()
}

/// Write search index, sitemap, and atom feed after the main build.
fn write_search_and_seo(result: &BuildResult) -> Result<(), BuildError> {
    // Search index
    if result.config.search.enabled {
        let search_data = pages_to_search_data(&result.site_graph.pages);
        let index = pyohwa_search::build_search_index(&search_data);
        let json = pyohwa_search::serialize_search_index(&index)
            .map_err(|e| BuildError::Search(e.to_string()))?;
        std::fs::write(result.output_dir.join("search-index.json"), json)?;
    }

    // Sitemap
    crate::build::output::write_sitemap(
        &result.output_pages,
        &result.config,
        &result.output_dir,
    )?;

    // Atom feed
    crate::build::output::write_atom_feed(
        &result.site_graph.pages,
        &result.config,
        &result.output_dir,
    )?;

    Ok(())
}

/// Internal: run stages 1–7, returning rendered pages and paths.
fn build_internal(
    project_root: &Path,
    ws_port: Option<u16>,
) -> Result<BuildResult, BuildError> {
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

    Ok(BuildResult {
        output_pages,
        site_graph,
        config,
        output_dir,
        static_dir,
    })
}
