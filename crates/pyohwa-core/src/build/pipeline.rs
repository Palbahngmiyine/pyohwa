use std::path::Path;

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

/// Execute the full 8-stage build pipeline.
///
/// Each stage transforms data from the previous stage:
/// 1. Config load (IO)
/// 2. Content discovery (IO)
/// 3. Frontmatter parsing (pure)
/// 4. Markdown -> HTML (pure)
/// 5. Syntax highlighting (pure)
/// 6. Site graph construction (pure)
/// 7. HTML template rendering (pure)
/// 8. Output writing (IO)
pub fn build(project_root: &Path) -> Result<(), BuildError> {
    // Stage 1: Load config from pyohwa.toml (or use defaults)
    let config = config::load(project_root)?;

    let content_dir = project_root.join(&config.build.content_dir);
    let output_dir = project_root.join(&config.build.output_dir);
    let static_dir = project_root.join(&config.build.static_dir);

    // Guard: content directory must exist
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
    let site_graph = graph::build_graph_with_content_dir(&highlighted_contents, &config, &content_dir);

    // Stage 7: Render HTML templates (pure)
    let output_pages: Vec<_> = site_graph
        .pages
        .iter()
        .map(|page| {
            let html = template::render_page(page, &site_graph, &config)?;
            Ok((page.route.clone(), html))
        })
        .collect::<Result<Vec<_>, BuildError>>()?;

    // Stage 8: Write output (IO)
    crate::build::output::write_output(&output_pages, &output_dir)?;

    // Copy static assets
    if static_dir.exists() {
        assets::copy_static_assets(&static_dir, &output_dir)?;
    }

    Ok(())
}
