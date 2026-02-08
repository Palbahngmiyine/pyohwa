use syntect::highlighting::ThemeSet;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::content::page::RenderedContent;
use crate::error::BuildError;

/// Apply syntax highlighting to fenced code blocks in rendered HTML.
///
/// Finds `<pre><code class="language-XXX">...</code></pre>` blocks and replaces
/// the code content with syntect-highlighted HTML using CSS classes.
pub fn apply_syntax_highlighting(content: &RenderedContent) -> Result<RenderedContent, BuildError> {
    let highlighted = highlight_code_blocks(&content.html);
    Ok(RenderedContent {
        path: content.path.clone(),
        frontmatter: content.frontmatter.clone(),
        html: highlighted,
        toc: content.toc.clone(),
    })
}

/// Generate a CSS stylesheet for syntax highlighting.
pub fn generate_css() -> String {
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["InspiredGitHub"];
    syntect::html::css_for_theme_with_class_style(theme, ClassStyle::Spaced).unwrap_or_default()
}

fn highlight_code_blocks(html: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let mut result = String::with_capacity(html.len());
    let mut remaining = html;

    // Pattern: <pre><code class="language-XXX">...CODE...</code></pre>
    while let Some(pre_start) = remaining.find("<pre><code class=\"language-") {
        // Copy everything before this match
        result.push_str(&remaining[..pre_start]);

        let after_prefix = &remaining[pre_start + 26..]; // skip `<pre><code class="language-`
        let lang_end = match after_prefix.find('"') {
            Some(pos) => pos,
            None => {
                result.push_str(&remaining[pre_start..pre_start + 26]);
                remaining = after_prefix;
                continue;
            }
        };
        let lang = &after_prefix[..lang_end];

        // Skip past `">`
        let code_start_offset = lang_end + 2; // skip `">`
        let code_html = &after_prefix[code_start_offset..];

        let code_end = match code_html.find("</code></pre>") {
            Some(pos) => pos,
            None => {
                result.push_str(&remaining[pre_start..pre_start + 26 + code_start_offset]);
                remaining = code_html;
                continue;
            }
        };

        let code_text = &code_html[..code_end];
        let decoded = decode_html_entities(code_text);

        let syntax = ss
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| ss.find_syntax_plain_text());

        let mut generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &ss, ClassStyle::Spaced);

        for line in LinesWithEndings::from(&decoded) {
            let _ = generator.parse_html_for_line_which_includes_newline(line);
        }

        let highlighted = generator.finalize();
        result.push_str(&format!(
            "<pre class=\"highlight\"><code class=\"language-{lang}\">{highlighted}</code></pre>"
        ));

        remaining = &code_html[code_end + 13..]; // skip `</code></pre>`
    }

    result.push_str(remaining);
    result
}

/// Decode basic HTML entities that comrak encodes in code blocks.
fn decode_html_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::frontmatter::Frontmatter;
    use std::path::PathBuf;

    fn make_rendered(html: &str) -> RenderedContent {
        RenderedContent {
            path: PathBuf::from("test.md"),
            frontmatter: Frontmatter {
                title: "Test".to_string(),
                ..Default::default()
            },
            html: html.to_string(),
            toc: vec![],
        }
    }

    #[test]
    fn highlights_rust_code_block() {
        let html = r#"<pre><code class="language-rust">fn main() {
    println!("Hello");
}
</code></pre>"#;
        let content = make_rendered(html);
        let result = apply_syntax_highlighting(&content).unwrap();
        assert!(result.html.contains("class=\"highlight\""));
        assert!(result.html.contains("<span"));
    }

    #[test]
    fn preserves_non_code_html() {
        let html = "<p>Hello <strong>world</strong></p>";
        let content = make_rendered(html);
        let result = apply_syntax_highlighting(&content).unwrap();
        assert_eq!(result.html, html);
    }

    #[test]
    fn handles_unknown_language() {
        let html = r#"<pre><code class="language-unknownlang">some code</code></pre>"#;
        let content = make_rendered(html);
        let result = apply_syntax_highlighting(&content).unwrap();
        // Should not crash, falls back to plain text
        assert!(result.html.contains("some code"));
    }

    #[test]
    fn handles_html_entities_in_code() {
        let html =
            r#"<pre><code class="language-rust">let x = 1 &amp;&amp; 2 &lt; 3;</code></pre>"#;
        let content = make_rendered(html);
        let result = apply_syntax_highlighting(&content).unwrap();
        assert!(!result.html.contains("&amp;amp;"));
    }

    #[test]
    fn multiple_code_blocks() {
        let html = r#"<p>Text</p>
<pre><code class="language-rust">let x = 1;</code></pre>
<p>More text</p>
<pre><code class="language-python">x = 1</code></pre>"#;
        let content = make_rendered(html);
        let result = apply_syntax_highlighting(&content).unwrap();
        assert_eq!(result.html.matches("class=\"highlight\"").count(), 2);
    }

    #[test]
    fn generate_css_returns_non_empty() {
        let css = generate_css();
        assert!(!css.is_empty());
        assert!(css.contains("color"));
    }
}
