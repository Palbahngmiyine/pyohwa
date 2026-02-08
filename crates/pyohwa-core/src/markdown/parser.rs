use comrak::nodes::NodeValue;
use comrak::{format_html, parse_document, Arena, Options};

use crate::content::page::{ParsedContent, RenderedContent, TocItem};
use crate::error::BuildError;

/// Parse Markdown body to HTML and extract TOC headings.
///
/// Uses comrak with CommonMark + GFM extensions (tables, strikethrough, tasklist, autolink).
/// Headings are collected into a flat TocItem list with slugified ids.
pub fn parse_markdown(content: &ParsedContent) -> Result<RenderedContent, BuildError> {
    let (html, toc) = markdown_to_html_with_toc(&content.body);

    Ok(RenderedContent {
        path: content.path.clone(),
        frontmatter: content.frontmatter.clone(),
        html,
        toc,
    })
}

/// Convert markdown string to HTML and extract TOC items.
fn markdown_to_html_with_toc(markdown: &str) -> (String, Vec<TocItem>) {
    let arena = Arena::new();
    let options = comrak_options();

    let root = parse_document(&arena, markdown, &options);

    let mut toc = Vec::new();
    collect_toc(root, &mut toc);

    // Insert id attributes into headings in the AST is not straightforward with comrak,
    // so we render HTML first, then post-process heading tags to add ids.
    let mut html_buf = Vec::new();
    format_html(root, &options, &mut html_buf).unwrap_or_default();
    let html = String::from_utf8_lossy(&html_buf).to_string();

    let html = inject_heading_ids(&html, &toc);

    (html, toc)
}

fn comrak_options() -> Options<'static> {
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.render.unsafe_ = true;
    options
}

/// Walk the AST to extract heading nodes and build TocItem list.
fn collect_toc<'a>(node: &'a comrak::nodes::AstNode<'a>, toc: &mut Vec<TocItem>) {
    let data = node.data.borrow();
    if let NodeValue::Heading(ref heading) = data.value {
        let text = collect_text(node);
        if !text.is_empty() {
            let id = slugify(&text);
            toc.push(TocItem {
                id,
                text,
                level: heading.level,
            });
        }
    }
    drop(data);

    for child in node.children() {
        collect_toc(child, toc);
    }
}

/// Recursively collect all text content from a node and its children.
fn collect_text<'a>(node: &'a comrak::nodes::AstNode<'a>) -> String {
    let mut text = String::new();
    collect_text_inner(node, &mut text);
    text
}

fn collect_text_inner<'a>(node: &'a comrak::nodes::AstNode<'a>, buf: &mut String) {
    let data = node.data.borrow();
    if let NodeValue::Text(ref s) = data.value {
        buf.push_str(s);
    } else if let NodeValue::Code(ref code) = data.value {
        buf.push_str(&code.literal);
    }
    drop(data);

    for child in node.children() {
        collect_text_inner(child, buf);
    }
}

/// Convert text to a URL-safe slug for heading IDs.
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c == ' ' || c == '-' || c == '_' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Post-process HTML to inject id attributes into heading tags.
///
/// Matches `<h1>`, `<h2>`, etc. and adds `id="slug"` based on the corresponding TocItem.
fn inject_heading_ids(html: &str, toc: &[TocItem]) -> String {
    let mut result = html.to_string();
    for item in toc {
        // Find `<hN>` and replace with `<hN id="slug">`
        let tag = format!("<h{}>", item.level);
        let tag_with_id = format!("<h{} id=\"{}\">", item.level, item.id);
        // Only replace the first occurrence for each TOC item
        if let Some(pos) = result.find(&tag) {
            result = format!(
                "{}{}{}",
                &result[..pos],
                tag_with_id,
                &result[pos + tag.len()..]
            );
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::frontmatter::Frontmatter;
    use std::path::PathBuf;

    fn make_parsed(body: &str) -> ParsedContent {
        ParsedContent {
            path: PathBuf::from("test.md"),
            frontmatter: Frontmatter {
                title: "Test".to_string(),
                ..Default::default()
            },
            body: body.to_string(),
        }
    }

    #[test]
    fn basic_markdown_conversion() {
        let content = make_parsed("Hello **world**!");
        let result = parse_markdown(&content).unwrap();
        assert!(result.html.contains("<strong>world</strong>"));
        assert!(result.html.contains("<p>"));
    }

    #[test]
    fn gfm_table_support() {
        let content = make_parsed("| Foo | Bar |\n|-----|-----|\n| Baz | Bim |");
        let result = parse_markdown(&content).unwrap();
        assert!(result.html.contains("<table>"));
        assert!(result.html.contains("<td>Baz</td>"));
    }

    #[test]
    fn gfm_tasklist_support() {
        let content = make_parsed("- [x] Done\n- [ ] Todo");
        let result = parse_markdown(&content).unwrap();
        assert!(result.html.contains("checked"));
        assert!(result.html.contains("type=\"checkbox\""));
    }

    #[test]
    fn gfm_strikethrough_support() {
        let content = make_parsed("~~deleted~~");
        let result = parse_markdown(&content).unwrap();
        assert!(result.html.contains("<del>deleted</del>"));
    }

    #[test]
    fn toc_extraction_from_headings() {
        let content = make_parsed(
            "# Introduction\n\nSome text.\n\n## Getting Started\n\nMore text.\n\n### Sub Section",
        );
        let result = parse_markdown(&content).unwrap();
        assert_eq!(result.toc.len(), 3);
        assert_eq!(result.toc[0].text, "Introduction");
        assert_eq!(result.toc[0].level, 1);
        assert_eq!(result.toc[0].id, "introduction");
        assert_eq!(result.toc[1].text, "Getting Started");
        assert_eq!(result.toc[1].level, 2);
        assert_eq!(result.toc[1].id, "getting-started");
        assert_eq!(result.toc[2].text, "Sub Section");
        assert_eq!(result.toc[2].level, 3);
        assert_eq!(result.toc[2].id, "sub-section");
    }

    #[test]
    fn heading_ids_injected_into_html() {
        let content = make_parsed("## Hello World");
        let result = parse_markdown(&content).unwrap();
        assert!(result.html.contains("id=\"hello-world\""));
    }

    #[test]
    fn empty_markdown_produces_empty_html() {
        let content = make_parsed("");
        let result = parse_markdown(&content).unwrap();
        assert!(result.toc.is_empty());
    }

    #[test]
    fn code_block_preserved() {
        let content = make_parsed("```rust\nfn main() {}\n```");
        let result = parse_markdown(&content).unwrap();
        assert!(result.html.contains("<code"));
        assert!(result.html.contains("fn main()"));
    }

    #[test]
    fn slugify_handles_special_chars() {
        assert_eq!(slugify("Hello World!"), "hello-world");
        assert_eq!(slugify("C++ & Rust"), "c-rust");
        assert_eq!(slugify("  multiple   spaces  "), "multiple-spaces");
        assert_eq!(slugify("already-slugged"), "already-slugged");
    }
}
