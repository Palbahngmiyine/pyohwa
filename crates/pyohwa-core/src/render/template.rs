use serde_json::json;

use crate::config::Config;
use crate::content::page::Page;
use crate::error::RenderError;
use crate::render::embedded;
use crate::render::layout::wrap_layout;
use crate::site::graph::SiteGraph;

/// Render a page to a complete HTML5 document.
///
/// This is a pure function that combines:
/// - Page content (HTML)
/// - Site graph data (nav, sidebar)
/// - Config (title, language, theme)
/// - Embedded assets (Elm JS, CSS)
///
/// The generated HTML includes `window.__PYOHWA_DATA__` for Elm initialization.
pub fn render_page(
    page: &Page,
    site_graph: &SiteGraph,
    config: &Config,
) -> Result<String, RenderError> {
    let page_title = build_page_title(&page.frontmatter.title, &config.site.title);
    let description = page
        .frontmatter
        .description
        .as_deref()
        .unwrap_or(&config.site.description);

    let body_content = wrap_layout(&page.frontmatter.layout, &page.html);

    let pyohwa_data = build_pyohwa_data(page, site_graph, config)?;

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="{lang}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <meta name="description" content="{description}">
    <link rel="stylesheet" href="{base}assets/theme.css">
</head>
<body class="bg-white text-gray-900 dark:bg-gray-950 dark:text-gray-100">
    <div id="app">
        {body_content}
    </div>

    <script>
    window.__PYOHWA_DATA__ = {pyohwa_data};
    </script>
    <script src="{base}assets/elm.min.js"></script>
    <script>
    if (typeof Elm !== 'undefined') {{
        var app = Elm.Main.init({{
            node: document.getElementById('app'),
            flags: window.__PYOHWA_DATA__
        }});
        if (app.ports) {{
            if (app.ports.scrollToElement) {{
                app.ports.scrollToElement.subscribe(function(id) {{
                    var el = document.getElementById(id);
                    if (el) {{ el.scrollIntoView({{ behavior: 'smooth', block: 'start' }}); }}
                }});
            }}
            if (app.ports.onScroll) {{
                var ticking = false;
                window.addEventListener('scroll', function() {{
                    if (!ticking) {{
                        window.requestAnimationFrame(function() {{
                            app.ports.onScroll.send(window.scrollY);
                            ticking = false;
                        }});
                        ticking = true;
                    }}
                }});
            }}
        }}
    }}
    </script>
</body>
</html>"#,
        lang = config.site.language,
        title = escape_html(&page_title),
        description = escape_html(description),
        base = normalize_base_url(&config.site.base_url),
        body_content = body_content,
        pyohwa_data = pyohwa_data,
    );

    Ok(html)
}

fn build_page_title(page_title: &str, site_title: &str) -> String {
    if page_title.is_empty() {
        return site_title.to_string();
    }
    if site_title.is_empty() {
        return page_title.to_string();
    }
    format!("{page_title} | {site_title}")
}

fn normalize_base_url(base: &str) -> String {
    if base.ends_with('/') {
        base.to_string()
    } else {
        format!("{base}/")
    }
}

fn build_pyohwa_data(
    page: &Page,
    site_graph: &SiteGraph,
    config: &Config,
) -> Result<String, RenderError> {
    let toc_items: Vec<_> = page
        .toc
        .iter()
        .map(|item| {
            json!({
                "id": item.id,
                "text": item.text,
                "level": item.level,
            })
        })
        .collect();

    let nav_items: Vec<_> = site_graph
        .nav
        .iter()
        .map(|item| {
            json!({
                "text": item.text,
                "link": item.link,
                "active": false,
            })
        })
        .collect();

    let sidebar_groups: Vec<_> = site_graph
        .sidebar
        .iter()
        .map(|group| {
            let items: Vec<_> = group
                .items
                .iter()
                .map(|item| {
                    json!({
                        "text": item.text,
                        "link": item.link,
                        "active": item.link == page.route.path,
                    })
                })
                .collect();
            json!({
                "text": group.text,
                "items": items,
            })
        })
        .collect();

    let layout_str = match &page.frontmatter.layout {
        crate::content::frontmatter::Layout::Doc => "doc",
        crate::content::frontmatter::Layout::Home => "home",
        crate::content::frontmatter::Layout::Page => "page",
        crate::content::frontmatter::Layout::Custom(s) => s.as_str(),
    };

    let prev_link = page.prev.as_ref().and_then(|route| {
        find_page_title(site_graph, route.path()).map(|title| {
            json!({ "title": title, "link": route.path() })
        })
    });

    let next_link = page.next.as_ref().and_then(|route| {
        find_page_title(site_graph, route.path()).map(|title| {
            json!({ "title": title, "link": route.path() })
        })
    });

    let mut data = json!({
        "page": {
            "title": page.frontmatter.title,
            "description": page.frontmatter.description.as_deref().unwrap_or(""),
            "content": page.html,
            "toc": toc_items,
            "layout": layout_str,
            "frontmatter": {}
        },
        "site": {
            "title": config.site.title,
            "description": config.site.description,
            "base": config.site.base_url,
            "nav": nav_items,
            "sidebar": sidebar_groups,
        },
        "theme": {
            "highlightTheme": config.theme.highlight_theme,
        }
    });

    if let Some(prev) = prev_link {
        data["prev"] = prev;
    }
    if let Some(next) = next_link {
        data["next"] = next;
    }

    serde_json::to_string(&data).map_err(RenderError::Serialization)
}

/// Find a page's title by its route path from the site graph
fn find_page_title(site_graph: &SiteGraph, path: &str) -> Option<String> {
    // First check sidebar items (they have display titles)
    for group in &site_graph.sidebar {
        for item in &group.items {
            if item.link == path {
                return Some(item.text.clone());
            }
        }
    }
    // Fallback: check pages directly
    site_graph
        .pages
        .iter()
        .find(|p| p.route.path() == path)
        .map(|p| p.frontmatter.title.clone())
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Render a page with live reload script injected before `</body>`.
/// Used by the dev server to enable automatic browser refresh.
pub fn render_page_with_live_reload(
    page: &Page,
    site_graph: &SiteGraph,
    config: &Config,
    ws_port: u16,
) -> Result<String, RenderError> {
    let html = render_page(page, site_graph, config)?;
    let script = live_reload_client_js(ws_port);
    Ok(html.replace("</body>", &format!("{script}\n</body>")))
}

/// Generate the live reload client JavaScript.
/// Connects to the dev server WebSocket and reloads on "reload" message.
pub fn live_reload_client_js(port: u16) -> String {
    format!(
        r#"<script>
(function() {{
    var ws;
    function connect() {{
        ws = new WebSocket('ws://localhost:{port}/__pyohwa_ws');
        ws.onmessage = function(e) {{
            if (e.data === 'reload') {{ location.reload(); }}
        }};
        ws.onclose = function() {{
            setTimeout(function() {{
                connect();
            }}, 1000);
        }};
    }}
    connect();
}})();
</script>"#,
        port = port
    )
}

/// Write embedded assets (Elm JS, CSS) to the output directory
pub fn write_embedded_assets(output_dir: &std::path::Path) -> Result<(), crate::error::BuildError> {
    let assets_dir = output_dir.join("assets");
    std::fs::create_dir_all(&assets_dir)?;

    std::fs::write(assets_dir.join("elm.min.js"), embedded::ELM_JS)?;
    std::fs::write(assets_dir.join("theme.css"), embedded::THEME_CSS)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::frontmatter::Frontmatter;
    use crate::content::page::TocItem;
    use crate::site::graph::{NavItem, SidebarGroup, SidebarItem, SiteGraph};
    use crate::site::route::Route;
    use std::path::PathBuf;

    fn make_test_page() -> Page {
        Page {
            route: Route {
                path: "/guide/intro".to_string(),
                source: PathBuf::from("guide/intro.md"),
                output: PathBuf::from("guide/intro/index.html"),
            },
            frontmatter: Frontmatter {
                title: "Introduction".to_string(),
                description: Some("Getting started guide".to_string()),
                ..Default::default()
            },
            html: "<h1>Introduction</h1><p>Hello world</p>".to_string(),
            toc: vec![TocItem {
                id: "introduction".to_string(),
                text: "Introduction".to_string(),
                level: 1,
            }],
            prev: None,
            next: None,
        }
    }

    fn make_test_graph() -> SiteGraph {
        SiteGraph {
            pages: vec![],
            sidebar: vec![SidebarGroup {
                text: "Guide".to_string(),
                items: vec![SidebarItem {
                    text: "Introduction".to_string(),
                    link: "/guide/intro".to_string(),
                }],
            }],
            nav: vec![NavItem {
                text: "Guide".to_string(),
                link: "/guide/".to_string(),
            }],
        }
    }

    #[test]
    fn test_render_page_html5() {
        let page = make_test_page();
        let graph = make_test_graph();
        let config = Config::default();

        let html = render_page(&page, &graph, &config).unwrap();

        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<html lang=\"en\">"));
        assert!(html.contains("<meta charset=\"UTF-8\">"));
        assert!(html.contains("<title>Introduction | Documentation</title>"));
        assert!(html.contains("window.__PYOHWA_DATA__"));
        assert!(html.contains("Elm.Main.init"));
    }

    #[test]
    fn test_render_page_contains_content() {
        let page = make_test_page();
        let graph = make_test_graph();
        let config = Config::default();

        let html = render_page(&page, &graph, &config).unwrap();

        assert!(html.contains("Hello world"));
    }

    #[test]
    fn test_build_page_title() {
        assert_eq!(
            build_page_title("Intro", "My Docs"),
            "Intro | My Docs"
        );
        assert_eq!(build_page_title("", "My Docs"), "My Docs");
        assert_eq!(build_page_title("Intro", ""), "Intro");
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(
            escape_html("<script>alert('xss')</script>"),
            "&lt;script&gt;alert('xss')&lt;/script&gt;"
        );
    }

    #[test]
    fn test_live_reload_script_injected() {
        let page = make_test_page();
        let graph = make_test_graph();
        let config = Config::default();

        let html = render_page_with_live_reload(&page, &graph, &config, 3000).unwrap();
        assert!(html.contains("__pyohwa_ws"));
        assert!(html.contains("WebSocket"));
    }

    #[test]
    fn test_live_reload_script_before_body_close() {
        let page = make_test_page();
        let graph = make_test_graph();
        let config = Config::default();

        let html = render_page_with_live_reload(&page, &graph, &config, 3000).unwrap();
        let ws_pos = html.find("__pyohwa_ws").unwrap();
        let body_pos = html.find("</body>").unwrap();
        assert!(ws_pos < body_pos);
    }

    #[test]
    fn test_live_reload_port_substitution() {
        let js = live_reload_client_js(4567);
        assert!(js.contains("localhost:4567"));
    }
}
