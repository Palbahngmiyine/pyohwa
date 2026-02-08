use serde::Deserialize;

use crate::content::page::{ParsedContent, RawContent};
use crate::error::ContentError;

/// Parse frontmatter from raw content, separating YAML header from body
pub fn parse_frontmatter(raw: &RawContent) -> Result<ParsedContent, ContentError> {
    if raw.raw.trim().is_empty() {
        return Err(ContentError::EmptyContent {
            path: raw.path.clone(),
        });
    }

    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let parsed = matter.parse(&raw.raw);

    let frontmatter = if let Some(data) = parsed.data {
        let raw_fm: RawFrontmatter =
            data.deserialize()
                .map_err(|e| ContentError::InvalidFrontmatter {
                    path: raw.path.clone(),
                    reason: e.to_string(),
                })?;
        let fm = raw_fm.into_frontmatter("");
        if fm.title.is_empty() {
            return Err(ContentError::MissingTitle {
                path: raw.path.clone(),
            });
        }
        fm
    } else {
        return Err(ContentError::MissingFrontmatter {
            path: raw.path.clone(),
        });
    };

    Ok(ParsedContent {
        path: raw.path.clone(),
        frontmatter,
        body: parsed.content,
    })
}

#[derive(Debug, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub description: Option<String>,
    pub layout: Layout,
    pub order: Option<i32>,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub draft: bool,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Layout {
    #[default]
    Doc,
    Home,
    Page,
    Custom(String),
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            layout: Layout::default(),
            order: None,
            tags: Vec::new(),
            date: None,
            draft: false,
            prev: None,
            next: None,
        }
    }
}

/// Raw frontmatter as deserialized from YAML before validation
#[derive(Debug, Deserialize)]
pub(crate) struct RawFrontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub layout: Option<String>,
    pub order: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub date: Option<String>,
    pub draft: Option<bool>,
    pub prev: Option<String>,
    pub next: Option<String>,
}

impl RawFrontmatter {
    pub fn into_frontmatter(self, title_fallback: &str) -> Frontmatter {
        Frontmatter {
            title: self.title.unwrap_or_else(|| title_fallback.to_string()),
            description: self.description,
            layout: match self.layout.as_deref() {
                Some("home") => Layout::Home,
                Some("page") => Layout::Page,
                Some("doc") | None => Layout::Doc,
                Some(custom) => Layout::Custom(custom.to_string()),
            },
            order: self.order,
            tags: self.tags.unwrap_or_default(),
            date: self.date,
            draft: self.draft.unwrap_or(false),
            prev: self.prev,
            next: self.next,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn raw(content: &str) -> RawContent {
        RawContent {
            path: PathBuf::from("test.md"),
            raw: content.to_string(),
        }
    }

    #[test]
    fn parses_valid_frontmatter() {
        let input = raw(r#"---
title: Hello World
description: A test page
tags:
  - rust
  - docs
---
Body content here."#);
        let result = parse_frontmatter(&input).unwrap();
        assert_eq!(result.frontmatter.title, "Hello World");
        assert_eq!(
            result.frontmatter.description,
            Some("A test page".to_string())
        );
        assert_eq!(result.frontmatter.tags, vec!["rust", "docs"]);
        assert_eq!(result.frontmatter.layout, Layout::Doc);
        assert!(!result.frontmatter.draft);
        assert_eq!(result.body, "Body content here.");
    }

    #[test]
    fn missing_title_returns_error() {
        let input = raw(r#"---
description: No title here
---
Body"#);
        let err = parse_frontmatter(&input).unwrap_err();
        assert!(matches!(err, ContentError::MissingTitle { .. }));
    }

    #[test]
    fn empty_content_returns_error() {
        let input = raw("");
        let err = parse_frontmatter(&input).unwrap_err();
        assert!(matches!(err, ContentError::EmptyContent { .. }));
    }

    #[test]
    fn whitespace_only_returns_error() {
        let input = raw("   \n  \n  ");
        let err = parse_frontmatter(&input).unwrap_err();
        assert!(matches!(err, ContentError::EmptyContent { .. }));
    }

    #[test]
    fn no_frontmatter_returns_error() {
        let input = raw("Just some text without frontmatter.");
        let err = parse_frontmatter(&input).unwrap_err();
        assert!(matches!(err, ContentError::MissingFrontmatter { .. }));
    }

    #[test]
    fn layout_variants_parsed_correctly() {
        for (layout_str, expected) in [
            ("home", Layout::Home),
            ("page", Layout::Page),
            ("doc", Layout::Doc),
            ("custom-layout", Layout::Custom("custom-layout".to_string())),
        ] {
            let input = raw(&format!(
                "---\ntitle: Test\nlayout: {layout_str}\n---\nBody"
            ));
            let result = parse_frontmatter(&input).unwrap();
            assert_eq!(result.frontmatter.layout, expected);
        }
    }

    #[test]
    fn draft_flag_parsed() {
        let input = raw("---\ntitle: Draft\ndraft: true\n---\nBody");
        let result = parse_frontmatter(&input).unwrap();
        assert!(result.frontmatter.draft);
    }

    #[test]
    fn optional_fields_default_correctly() {
        let input = raw("---\ntitle: Minimal\n---\nBody");
        let result = parse_frontmatter(&input).unwrap();
        assert!(result.frontmatter.description.is_none());
        assert!(result.frontmatter.order.is_none());
        assert!(result.frontmatter.tags.is_empty());
        assert!(result.frontmatter.date.is_none());
        assert!(!result.frontmatter.draft);
        assert!(result.frontmatter.prev.is_none());
        assert!(result.frontmatter.next.is_none());
    }
}
