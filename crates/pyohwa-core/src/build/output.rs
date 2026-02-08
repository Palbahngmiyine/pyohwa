use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::content::page::Page;
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
pub fn write_output(pages: &[(Route, String)], output_dir: &Path) -> Result<(), BuildError> {
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

/// Generate a sitemap.xml string from rendered pages.
pub fn generate_sitemap(pages: &[(Route, String)], config: &Config) -> String {
    let base = normalize_sitemap_base(&config.site.base_url);
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for (route, _) in pages {
        let loc = format!("{}{}", base, escape_xml(route.path()));
        xml.push_str(&format!("  <url>\n    <loc>{loc}</loc>\n  </url>\n"));
    }
    xml.push_str("</urlset>\n");
    xml
}

/// Write sitemap.xml to the output directory if enabled.
pub fn write_sitemap(
    pages: &[(Route, String)],
    config: &Config,
    output_dir: &Path,
) -> Result<(), BuildError> {
    if !config.seo.sitemap {
        return Ok(());
    }
    let sitemap = generate_sitemap(pages, config);
    fs::write(output_dir.join("sitemap.xml"), sitemap)?;
    Ok(())
}

/// Generate an Atom 1.0 feed from pages that have dates.
pub fn generate_atom_feed(pages: &[Page], config: &Config) -> String {
    let base = normalize_sitemap_base(&config.site.base_url);
    let escaped_title = escape_xml(&config.site.title);
    let escaped_desc = escape_xml(&config.site.description);

    let mut dated_pages: Vec<&Page> = pages
        .iter()
        .filter(|p| !p.frontmatter.draft && p.frontmatter.date.is_some())
        .collect();

    // Sort by date descending
    dated_pages.sort_by(|a, b| {
        let da = a.frontmatter.date.as_deref().unwrap_or("");
        let db = b.frontmatter.date.as_deref().unwrap_or("");
        db.cmp(da)
    });

    // Limit to 20 entries
    dated_pages.truncate(20);

    let updated = dated_pages
        .first()
        .and_then(|p| p.frontmatter.date.as_deref())
        .unwrap_or("1970-01-01");

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{escaped_title}</title>
  <subtitle>{escaped_desc}</subtitle>
  <link href="{base}" rel="alternate"/>
  <link href="{base}feed.xml" rel="self"/>
  <id>{base}</id>
  <updated>{updated}T00:00:00Z</updated>
"#
    );

    for page in &dated_pages {
        let title = escape_xml(&page.frontmatter.title);
        let url = format!("{}{}", base, page.route.path());
        let date = page.frontmatter.date.as_deref().unwrap_or("1970-01-01");
        let desc = page.frontmatter.description.as_deref().unwrap_or("");
        let escaped_desc_entry = escape_xml(desc);
        xml.push_str(&format!(
            r#"  <entry>
    <title>{title}</title>
    <link href="{url}" rel="alternate"/>
    <id>{url}</id>
    <updated>{date}T00:00:00Z</updated>
    <summary>{escaped_desc_entry}</summary>
  </entry>
"#
        ));
    }

    xml.push_str("</feed>\n");
    xml
}

/// Write atom feed to the output directory if enabled.
pub fn write_atom_feed(
    pages: &[Page],
    config: &Config,
    output_dir: &Path,
) -> Result<(), BuildError> {
    if !config.seo.rss {
        return Ok(());
    }
    let feed = generate_atom_feed(pages, config);
    fs::write(output_dir.join("feed.xml"), feed)?;
    Ok(())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn normalize_sitemap_base(base: &str) -> String {
    if base.ends_with('/') {
        base.trim_end_matches('/').to_string()
    } else {
        base.to_string()
    }
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

    #[test]
    fn test_generate_sitemap() {
        let pages = vec![
            (
                Route {
                    path: "/".to_string(),
                    source: PathBuf::from("index.md"),
                    output: PathBuf::from("index.html"),
                },
                String::new(),
            ),
            (
                Route {
                    path: "/guide/intro".to_string(),
                    source: PathBuf::from("guide/intro.md"),
                    output: PathBuf::from("guide/intro/index.html"),
                },
                String::new(),
            ),
        ];
        let config = Config::default();
        let xml = generate_sitemap(&pages, &config);
        assert!(xml.contains("<urlset"));
        assert!(xml.contains("<loc>/</loc>"));
        assert!(xml.contains("<loc>/guide/intro</loc>"));
    }

    #[test]
    fn test_write_sitemap_disabled() {
        let tmp = std::env::temp_dir().join("pyohwa_test_sitemap_disabled");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let mut config = Config::default();
        config.seo.sitemap = false;

        write_sitemap(&[], &config, &tmp).unwrap();
        assert!(!tmp.join("sitemap.xml").exists());

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_generate_atom_feed() {
        use crate::content::frontmatter::Frontmatter;
        let pages = vec![Page {
            route: Route {
                path: "/blog/post".to_string(),
                source: PathBuf::from("blog/post.md"),
                output: PathBuf::from("blog/post/index.html"),
            },
            frontmatter: Frontmatter {
                title: "My Post".to_string(),
                description: Some("A blog post".to_string()),
                date: Some("2024-01-15".to_string()),
                ..Default::default()
            },
            html: "<p>Content</p>".to_string(),
            toc: vec![],
            prev: None,
            next: None,
        }];
        let config = Config::default();
        let xml = generate_atom_feed(&pages, &config);
        assert!(xml.contains("<feed"));
        assert!(xml.contains("<title>My Post</title>"));
        assert!(xml.contains("2024-01-15"));
    }

    #[test]
    fn test_atom_feed_filters_drafts_and_dateless() {
        use crate::content::frontmatter::Frontmatter;
        let pages = vec![
            Page {
                route: Route {
                    path: "/a".to_string(),
                    source: PathBuf::from("a.md"),
                    output: PathBuf::from("a/index.html"),
                },
                frontmatter: Frontmatter {
                    title: "Published".to_string(),
                    date: Some("2024-01-01".to_string()),
                    ..Default::default()
                },
                html: String::new(),
                toc: vec![],
                prev: None,
                next: None,
            },
            Page {
                route: Route {
                    path: "/b".to_string(),
                    source: PathBuf::from("b.md"),
                    output: PathBuf::from("b/index.html"),
                },
                frontmatter: Frontmatter {
                    title: "Draft".to_string(),
                    date: Some("2024-02-01".to_string()),
                    draft: true,
                    ..Default::default()
                },
                html: String::new(),
                toc: vec![],
                prev: None,
                next: None,
            },
            Page {
                route: Route {
                    path: "/c".to_string(),
                    source: PathBuf::from("c.md"),
                    output: PathBuf::from("c/index.html"),
                },
                frontmatter: Frontmatter {
                    title: "No Date".to_string(),
                    ..Default::default()
                },
                html: String::new(),
                toc: vec![],
                prev: None,
                next: None,
            },
        ];
        let config = Config::default();
        let xml = generate_atom_feed(&pages, &config);
        assert!(xml.contains("Published"));
        assert!(!xml.contains("Draft"));
        assert!(!xml.contains("No Date"));
    }

    #[test]
    fn test_atom_feed_sorted_by_date_desc() {
        use crate::content::frontmatter::Frontmatter;
        let pages = vec![
            Page {
                route: Route {
                    path: "/old".to_string(),
                    source: PathBuf::from("old.md"),
                    output: PathBuf::from("old/index.html"),
                },
                frontmatter: Frontmatter {
                    title: "Old".to_string(),
                    date: Some("2023-01-01".to_string()),
                    ..Default::default()
                },
                html: String::new(),
                toc: vec![],
                prev: None,
                next: None,
            },
            Page {
                route: Route {
                    path: "/new".to_string(),
                    source: PathBuf::from("new.md"),
                    output: PathBuf::from("new/index.html"),
                },
                frontmatter: Frontmatter {
                    title: "New".to_string(),
                    date: Some("2024-06-01".to_string()),
                    ..Default::default()
                },
                html: String::new(),
                toc: vec![],
                prev: None,
                next: None,
            },
        ];
        let config = Config::default();
        let xml = generate_atom_feed(&pages, &config);
        let new_pos = xml.find("New").unwrap();
        let old_pos = xml.find("Old").unwrap();
        assert!(new_pos < old_pos, "Newer entry should appear first");
    }

    #[test]
    fn test_write_atom_feed_disabled() {
        let tmp = std::env::temp_dir().join("pyohwa_test_feed_disabled");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let mut config = Config::default();
        config.seo.rss = false;

        write_atom_feed(&[], &config, &tmp).unwrap();
        assert!(!tmp.join("feed.xml").exists());

        let _ = fs::remove_dir_all(&tmp);
    }
}
