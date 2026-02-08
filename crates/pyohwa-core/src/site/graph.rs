use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

use crate::config::Config;
use crate::content::frontmatter::Layout;
use crate::content::page::{Page, RenderedContent};
use crate::site::route::{resolve_route, Route};

/// The complete site graph containing all pages and navigation structure
#[derive(Debug, Clone)]
pub struct SiteGraph {
    pub pages: Vec<Page>,
    pub sidebar: Vec<SidebarGroup>,
    pub nav: Vec<NavItem>,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct SidebarGroup {
    pub text: String,
    pub items: Vec<SidebarItem>,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct SidebarItem {
    pub text: String,
    pub link: String,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct NavItem {
    pub text: String,
    pub link: String,
}

/// Build the complete site graph from rendered content and config.
///
/// This is a pure function that:
/// 1. Resolves routes for all pages
/// 2. Builds the sidebar (auto or manual)
/// 3. Copies nav items from config
/// 4. Computes prev/next links based on sidebar order
pub fn build_graph(rendered: &[RenderedContent], config: &Config) -> SiteGraph {
    build_graph_with_content_dir(rendered, config, &config.build.content_dir)
}

/// Build site graph with an explicit content_dir path.
/// Used by the pipeline to pass the resolved absolute content directory.
pub fn build_graph_with_content_dir(
    rendered: &[RenderedContent],
    config: &Config,
    content_dir: &Path,
) -> SiteGraph {
    let mut pages: Vec<Page> = rendered
        .iter()
        .map(|rc| {
            let route = resolve_route(content_dir, &rc.path);
            Page {
                route,
                frontmatter: rc.frontmatter.clone(),
                html: rc.html.clone(),
                toc: rc.toc.clone(),
                prev: None,
                next: None,
            }
        })
        .collect();

    let sidebar = build_sidebar(&pages, config);
    let nav = build_nav(config);

    let ordered_paths = collect_sidebar_links(&sidebar);
    assign_prev_next(&mut pages, &ordered_paths);

    SiteGraph {
        pages,
        sidebar,
        nav,
    }
}

/// Build sidebar groups from pages.
/// If `config.sidebar.auto` is true, groups pages by directory.
/// Otherwise, uses manual sidebar config.
fn build_sidebar(pages: &[Page], config: &Config) -> Vec<SidebarGroup> {
    if !config.sidebar.auto {
        return config.sidebar.groups.clone();
    }

    auto_generate_sidebar(pages)
}

fn auto_generate_sidebar(pages: &[Page]) -> Vec<SidebarGroup> {
    let mut groups: BTreeMap<String, Vec<&Page>> = BTreeMap::new();

    for page in pages.iter().filter(|p| p.frontmatter.layout == Layout::Doc) {
        let dir = page.route.parent_dir();
        groups.entry(dir).or_default().push(page);
    }

    groups
        .into_iter()
        .map(|(dir, mut dir_pages)| {
            dir_pages.sort_by(|a, b| {
                let order_a = a.frontmatter.order.unwrap_or(i32::MAX);
                let order_b = b.frontmatter.order.unwrap_or(i32::MAX);
                order_a
                    .cmp(&order_b)
                    .then_with(|| a.frontmatter.title.cmp(&b.frontmatter.title))
            });

            let display = dir_display_name(&dir);
            let items = dir_pages
                .iter()
                .map(|p| SidebarItem {
                    text: p.frontmatter.title.clone(),
                    link: p.route.path().to_string(),
                })
                .collect();

            SidebarGroup {
                text: display,
                items,
            }
        })
        .collect()
}

fn dir_display_name(dir: &str) -> String {
    if dir.is_empty() {
        return "Root".to_string();
    }

    let last = Path::new(dir)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(dir);

    last.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{upper}{}", chars.as_str())
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn build_nav(config: &Config) -> Vec<NavItem> {
    config.nav.clone()
}

/// Collect all sidebar links in order for prev/next computation
fn collect_sidebar_links(sidebar: &[SidebarGroup]) -> Vec<String> {
    sidebar
        .iter()
        .flat_map(|group| group.items.iter().map(|item| item.link.clone()))
        .collect()
}

/// Assign prev/next routes to pages based on sidebar link order.
///
/// Uses index-based iteration to avoid simultaneous mutable and immutable borrows.
fn assign_prev_next(pages: &mut [Page], ordered_paths: &[String]) {
    // Pre-compute: for each page index, determine its prev/next Route
    let assignments: Vec<(Option<Route>, Option<Route>)> = pages
        .iter()
        .map(|page| {
            if page.frontmatter.prev.is_some() || page.frontmatter.next.is_some() {
                let prev = page.frontmatter.prev.as_ref().map(|p| Route {
                    path: p.clone(),
                    source: Default::default(),
                    output: Default::default(),
                });
                let next = page.frontmatter.next.as_ref().map(|p| Route {
                    path: p.clone(),
                    source: Default::default(),
                    output: Default::default(),
                });
                return (prev, next);
            }

            let current_path = page.route.path();
            let pos = ordered_paths.iter().position(|p| p == current_path);

            let prev = pos
                .filter(|&idx| idx > 0)
                .and_then(|idx| find_route_by_path(pages, &ordered_paths[idx - 1]));

            let next = pos
                .filter(|&idx| idx + 1 < ordered_paths.len())
                .and_then(|idx| find_route_by_path(pages, &ordered_paths[idx + 1]));

            (prev, next)
        })
        .collect();

    for (page, (prev, next)) in pages.iter_mut().zip(assignments) {
        page.prev = prev;
        page.next = next;
    }
}

fn find_route_by_path(pages: &[Page], path: &str) -> Option<Route> {
    pages
        .iter()
        .find(|p| p.route.path() == path)
        .map(|p| p.route.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::frontmatter::Frontmatter;
    use crate::content::page::RenderedContent;
    use std::path::PathBuf;

    fn make_rendered(path: &str, title: &str, order: Option<i32>) -> RenderedContent {
        RenderedContent {
            path: PathBuf::from(path),
            frontmatter: Frontmatter {
                title: title.to_string(),
                order,
                ..Default::default()
            },
            html: format!("<p>{title}</p>"),
            toc: vec![],
        }
    }

    #[test]
    fn test_build_graph_auto_sidebar() {
        let rendered = vec![
            make_rendered("content/guide/intro.md", "Introduction", Some(1)),
            make_rendered("content/guide/setup.md", "Setup", Some(2)),
            make_rendered("content/api/overview.md", "API Overview", None),
        ];

        let config = Config::default();
        let graph = build_graph(&rendered, &config);

        assert_eq!(graph.sidebar.len(), 2);
        assert_eq!(graph.sidebar[0].text, "Api");
        assert_eq!(graph.sidebar[0].items.len(), 1);
        assert_eq!(graph.sidebar[1].text, "Guide");
        assert_eq!(graph.sidebar[1].items.len(), 2);
        assert_eq!(graph.sidebar[1].items[0].text, "Introduction");
        assert_eq!(graph.sidebar[1].items[1].text, "Setup");
    }

    #[test]
    fn test_build_graph_prev_next() {
        let rendered = vec![
            make_rendered("content/guide/intro.md", "Introduction", Some(1)),
            make_rendered("content/guide/setup.md", "Setup", Some(2)),
            make_rendered("content/guide/config.md", "Configuration", Some(3)),
        ];

        let config = Config::default();
        let graph = build_graph(&rendered, &config);

        let intro = graph
            .pages
            .iter()
            .find(|p| p.frontmatter.title == "Introduction");
        let setup = graph.pages.iter().find(|p| p.frontmatter.title == "Setup");
        let cfg = graph
            .pages
            .iter()
            .find(|p| p.frontmatter.title == "Configuration");

        assert!(intro.is_some());
        assert!(intro.as_ref().map_or(true, |p| p.prev.is_none()));
        assert_eq!(
            intro
                .as_ref()
                .and_then(|p| p.next.as_ref().map(|r| r.path.as_str())),
            Some("/guide/setup")
        );

        assert_eq!(
            setup
                .as_ref()
                .and_then(|p| p.prev.as_ref().map(|r| r.path.as_str())),
            Some("/guide/intro")
        );
        assert_eq!(
            setup
                .as_ref()
                .and_then(|p| p.next.as_ref().map(|r| r.path.as_str())),
            Some("/guide/config")
        );

        assert_eq!(
            cfg.as_ref()
                .and_then(|p| p.prev.as_ref().map(|r| r.path.as_str())),
            Some("/guide/setup")
        );
        assert!(cfg.as_ref().map_or(true, |p| p.next.is_none()));
    }

    #[test]
    fn test_manual_sidebar() {
        let rendered = vec![make_rendered(
            "content/guide/intro.md",
            "Introduction",
            None,
        )];

        let mut config = Config::default();
        config.sidebar.auto = false;
        config.sidebar.groups = vec![SidebarGroup {
            text: "My Group".to_string(),
            items: vec![SidebarItem {
                text: "Custom Item".to_string(),
                link: "/custom".to_string(),
            }],
        }];

        let graph = build_graph(&rendered, &config);
        assert_eq!(graph.sidebar.len(), 1);
        assert_eq!(graph.sidebar[0].text, "My Group");
    }

    #[test]
    fn test_nav_from_config() {
        let rendered = vec![];
        let mut config = Config::default();
        config.nav = vec![NavItem {
            text: "Guide".to_string(),
            link: "/guide/".to_string(),
        }];

        let graph = build_graph(&rendered, &config);
        assert_eq!(graph.nav.len(), 1);
        assert_eq!(graph.nav[0].text, "Guide");
    }

    #[test]
    fn test_sidebar_order_by_frontmatter() {
        let rendered = vec![
            make_rendered("content/guide/z-last.md", "Z Last", Some(3)),
            make_rendered("content/guide/a-first.md", "A First", Some(1)),
            make_rendered("content/guide/m-middle.md", "M Middle", Some(2)),
        ];

        let config = Config::default();
        let graph = build_graph(&rendered, &config);

        assert_eq!(graph.sidebar[0].items[0].text, "A First");
        assert_eq!(graph.sidebar[0].items[1].text, "M Middle");
        assert_eq!(graph.sidebar[0].items[2].text, "Z Last");
    }
}
