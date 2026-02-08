pub mod error;
pub mod indexer;
pub mod tokenizer;

use error::SearchError;
use indexer::{extract_indexable_content, IndexOptions};
use serde::Serialize;

/// Independent page data type to avoid circular dependency with pyohwa-core.
/// pyohwa-core converts its own Page type into this before calling search APIs.
pub struct PageData {
    pub url: String,
    pub title: String,
    pub description: String,
    pub html: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub draft: bool,
}

/// The complete search index containing all searchable pages.
#[derive(Debug, Serialize)]
pub struct SearchIndex {
    pub pages: Vec<SearchEntry>,
}

/// A single entry in the search index.
#[derive(Debug, Clone, Serialize)]
pub struct SearchEntry {
    pub id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
}

/// Build a search index from a collection of pages.
/// Filters out draft pages and processes HTML content.
pub fn build_search_index(pages: &[PageData]) -> SearchIndex {
    let options = IndexOptions::default();
    let entries = pages
        .iter()
        .filter(|p| !p.draft)
        .map(|p| extract_indexable_content(p, &options))
        .collect();
    SearchIndex { pages: entries }
}

/// Serialize the search index to a JSON string.
pub fn serialize_search_index(index: &SearchIndex) -> Result<String, SearchError> {
    serde_json::to_string(index).map_err(SearchError::Serialization)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_page(title: &str, draft: bool) -> PageData {
        PageData {
            url: format!("/{}", title.to_lowercase().replace(' ', "-")),
            title: title.to_string(),
            description: format!("About {title}"),
            html: format!("<p>{title} content</p>"),
            tags: vec!["docs".to_string()],
            date: Some("2024-01-01".to_string()),
            draft,
        }
    }

    #[test]
    fn test_build_search_index_basic() {
        let pages = vec![make_page("Hello", false), make_page("World", false)];
        let index = build_search_index(&pages);
        assert_eq!(index.pages.len(), 2);
        assert_eq!(index.pages[0].title, "Hello");
        assert_eq!(index.pages[1].title, "World");
    }

    #[test]
    fn test_build_search_index_filters_drafts() {
        let pages = vec![
            make_page("Published", false),
            make_page("Draft", true),
            make_page("Another", false),
        ];
        let index = build_search_index(&pages);
        assert_eq!(index.pages.len(), 2);
        assert!(index.pages.iter().all(|e| e.title != "Draft"));
    }

    #[test]
    fn test_build_search_index_empty() {
        let pages: Vec<PageData> = vec![];
        let index = build_search_index(&pages);
        assert!(index.pages.is_empty());
    }

    #[test]
    fn test_serialize_produces_valid_json() {
        let pages = vec![make_page("Test", false)];
        let index = build_search_index(&pages);
        let json = serialize_search_index(&index).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("pages").unwrap().is_array());
    }

    #[test]
    fn test_search_entry_schema() {
        let pages = vec![make_page("Schema Test", false)];
        let index = build_search_index(&pages);
        let json = serialize_search_index(&index).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let entry = &parsed["pages"][0];
        assert!(entry.get("id").is_some());
        assert!(entry.get("url").is_some());
        assert!(entry.get("title").is_some());
        assert!(entry.get("description").is_some());
        assert!(entry.get("content").is_some());
        assert!(entry.get("tags").is_some());
        assert!(entry.get("date").is_some());
    }
}
