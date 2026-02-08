use crate::tokenizer::{strip_html, truncate_content};
use crate::{PageData, SearchEntry};

/// Options for controlling search index generation.
pub struct IndexOptions {
    pub max_content_length: usize,
}

impl Default for IndexOptions {
    fn default() -> Self {
        Self {
            max_content_length: 5000,
        }
    }
}

/// Extract indexable content from a page, stripping HTML and truncating.
pub fn extract_indexable_content(page: &PageData, options: &IndexOptions) -> SearchEntry {
    let plain_text = strip_html(&page.html);
    let content = truncate_content(&plain_text, options.max_content_length);

    SearchEntry {
        id: page.url.clone(),
        url: page.url.clone(),
        title: page.title.clone(),
        description: page.description.clone(),
        content,
        tags: page.tags.clone(),
        date: page.date.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_page(title: &str, html: &str) -> PageData {
        PageData {
            url: format!("/{}", title.to_lowercase()),
            title: title.to_string(),
            description: format!("About {title}"),
            html: html.to_string(),
            tags: vec!["test".to_string()],
            date: Some("2024-01-01".to_string()),
            draft: false,
        }
    }

    #[test]
    fn test_basic_extraction() {
        let page = make_page("Hello", "<p>Hello <strong>World</strong></p>");
        let entry = extract_indexable_content(&page, &IndexOptions::default());
        assert_eq!(entry.title, "Hello");
        assert_eq!(entry.content, "Hello World");
        assert_eq!(entry.url, "/hello");
    }

    #[test]
    fn test_content_length_limit() {
        let long_html = format!("<p>{}</p>", "word ".repeat(2000));
        let page = make_page("Long", &long_html);
        let options = IndexOptions {
            max_content_length: 50,
        };
        let entry = extract_indexable_content(&page, &options);
        assert!(entry.content.len() <= 55); // 50 + "..."
    }

    #[test]
    fn test_no_description() {
        let page = PageData {
            url: "/test".to_string(),
            title: "Test".to_string(),
            description: String::new(),
            html: "<p>Content</p>".to_string(),
            tags: vec![],
            date: None,
            draft: false,
        };
        let entry = extract_indexable_content(&page, &IndexOptions::default());
        assert!(entry.description.is_empty());
    }
}
