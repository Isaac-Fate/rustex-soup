use lazy_static::lazy_static;
use regex::Regex;

use super::{
    node::{TexNode, extract_nodes},
    span::Span
};

lazy_static! {
    static ref INLINE_MATH_RE: Regex = Regex::new(r#"^\$[^\$]+\$"#)
        .unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineMath {
    pub span: Span,
    pub children: Vec<TexNode>
}

pub fn extract_inline_math(tex: &str, start: usize) -> Option<InlineMath> {
    if let Some(mat) = INLINE_MATH_RE.find(&tex[start..]) {
        Some(InlineMath { 
            span: Span { start, end: start + mat.end() }, 
            children: extract_nodes(
                    tex, 
                    start + 1, 
                    start + mat.end() - 1
                )
                .unwrap()
        })
    } else {
        None
    }
}

