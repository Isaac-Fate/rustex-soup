use lazy_static::lazy_static;
use regex::Regex;

use super::{
    node::{TexNode, extract_nodes},
    span::Span
};

lazy_static! {
    static ref SQUARE_GROUP_RE: Regex = Regex::new(r"^\[[^\n\]]*\]")
        .unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct SquareGroup {
    pub span: Span,
    pub children: Vec<TexNode>
}

pub fn extract_square_group(tex: &str, start: usize) -> Option<SquareGroup> {
    if let Some(mat) = SQUARE_GROUP_RE.find(&tex[start..]) {
        let span = Span {
            start: start + mat.start(),
            end: start + mat.end()
        };

        Some(SquareGroup { 
            span, 
            children: extract_nodes(
                    tex, 
                    span.start + 1, 
                    span.end - 1
                )
                .unwrap()
        })
    } else {
        None
    }
}