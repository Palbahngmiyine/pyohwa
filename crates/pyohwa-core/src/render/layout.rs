use crate::content::frontmatter::Layout;

/// Wrap content HTML in a layout-specific div structure.
///
/// For Phase 1, only the `doc` layout is fully supported:
/// - `doc`: sidebar area + content + TOC area
/// - `home`: content only (full width)
/// - `page`: content only (centered)
/// - `custom`: same as doc
pub fn wrap_layout(layout: &Layout, content: &str) -> String {
    match layout {
        Layout::Doc | Layout::Custom(_) => wrap_doc_layout(content),
        Layout::Home => wrap_home_layout(content),
        Layout::Page => wrap_page_layout(content),
    }
}

fn wrap_doc_layout(content: &str) -> String {
    format!(
        r#"<div class="pyohwa-layout-doc flex">
    <aside class="pyohwa-sidebar" id="sidebar"></aside>
    <main class="pyohwa-content flex-1">
        <div class="pyohwa-prose" id="content">{content}</div>
    </main>
    <aside class="pyohwa-toc" id="toc"></aside>
</div>"#
    )
}

fn wrap_home_layout(content: &str) -> String {
    format!(
        r#"<div class="pyohwa-layout-home">
    <main class="pyohwa-content">
        <div class="pyohwa-prose" id="content">{content}</div>
    </main>
</div>"#
    )
}

fn wrap_page_layout(content: &str) -> String {
    format!(
        r#"<div class="pyohwa-layout-page">
    <main class="pyohwa-content mx-auto max-w-3xl">
        <div class="pyohwa-prose" id="content">{content}</div>
    </main>
</div>"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_layout() {
        let html = wrap_layout(&Layout::Doc, "<p>Hello</p>");
        assert!(html.contains("pyohwa-layout-doc"));
        assert!(html.contains("pyohwa-sidebar"));
        assert!(html.contains("pyohwa-toc"));
        assert!(html.contains("<p>Hello</p>"));
    }

    #[test]
    fn test_home_layout() {
        let html = wrap_layout(&Layout::Home, "<p>Welcome</p>");
        assert!(html.contains("pyohwa-layout-home"));
        assert!(!html.contains("pyohwa-sidebar"));
        assert!(html.contains("<p>Welcome</p>"));
    }

    #[test]
    fn test_page_layout() {
        let html = wrap_layout(&Layout::Page, "<p>About</p>");
        assert!(html.contains("pyohwa-layout-page"));
        assert!(html.contains("max-w-3xl"));
        assert!(html.contains("<p>About</p>"));
    }

    #[test]
    fn test_custom_layout_uses_doc() {
        let html = wrap_layout(&Layout::Custom("my-layout".to_string()), "<p>Custom</p>");
        assert!(html.contains("pyohwa-layout-doc"));
    }
}
