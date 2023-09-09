use lazy_static::lazy_static;
use regex::Regex;

use super::span::Span;

lazy_static! {
    static ref COMMENT_RE: Regex = Regex::new(r"^%[^\n]*\n")
        .unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub span: Span,
    pub content: String
}

pub fn extract_comment(tex: &str, start: usize) -> Option<Comment> {
    if let Some(mat) = COMMENT_RE.find(&tex[start..]) {
        Some(Comment { 
            span: Span { start, end: start + mat.end() }, 
            content: mat.as_str().to_string()
        })
    } else {
        None
    }
}