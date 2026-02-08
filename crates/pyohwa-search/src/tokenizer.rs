use regex::Regex;

/// Remove HTML tags from a string, decode common entities, and normalize whitespace.
pub fn strip_html(html: &str) -> String {
    let tag_re = Regex::new(r"<[^>]*>").unwrap();
    let stripped = tag_re.replace_all(html, " ");
    let decoded = decode_html_entities(&stripped);
    collapse_whitespace(&decoded)
}

/// Truncate text at a word boundary, appending "..." if truncated.
pub fn truncate_content(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }

    let truncated = &text[..max_chars];
    // Find the last space to avoid cutting words
    match truncated.rfind(' ') {
        Some(pos) => format!("{}...", &truncated[..pos]),
        None => format!("{truncated}..."),
    }
}

fn decode_html_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&#x27;", "'")
        .replace("&nbsp;", " ")
}

fn collapse_whitespace(s: &str) -> String {
    let ws_re = Regex::new(r"\s+").unwrap();
    ws_re.replace_all(s.trim(), " ").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_simple_tags() {
        assert_eq!(strip_html("<p>Hello</p>"), "Hello");
    }

    #[test]
    fn test_strip_tags_with_attributes() {
        assert_eq!(
            strip_html(r#"<a href="/link" class="active">Click</a>"#),
            "Click"
        );
    }

    #[test]
    fn test_decode_html_entities() {
        assert_eq!(
            strip_html("<p>A &amp; B &lt; C &gt; D</p>"),
            "A & B < C > D"
        );
    }

    #[test]
    fn test_whitespace_normalization() {
        assert_eq!(strip_html("<p>Hello</p>   <p>World</p>"), "Hello World");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(strip_html(""), "");
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            strip_html("<pre><code>fn main() {}</code></pre>"),
            "fn main() {}"
        );
    }

    #[test]
    fn test_nested_tags() {
        assert_eq!(
            strip_html("<div><p><strong>Bold</strong> text</p></div>"),
            "Bold text"
        );
    }

    #[test]
    fn test_truncate_content() {
        let text = "Hello world this is a long text";
        assert_eq!(truncate_content(text, 11), "Hello...");
        assert_eq!(truncate_content(text, 100), text);
    }
}
