use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

use crate::ParseError;
use super::{
    node::{TexNode, extract_nodes},
    span::Span
};

lazy_static! {
    static ref GROUP_SYMBOL_RE: Regex = Regex::new(r"[\{\}]")
        .unwrap();
    static ref BEGIN_GROUP_RE: Regex = Regex::new(r"^\{")
        .unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub span: Span,
    pub children: Vec<TexNode>
}

pub fn extract_group(tex: &str, start: usize) -> Option<Group> {
    if let Some(_) = BEGIN_GROUP_RE.find(&tex[start..]) {
        let span = find_group_span(tex, start).unwrap();
        assert_eq!(span.start, start);

        Some(Group { 
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

fn find_group_span(tex: &str, mut start: usize) -> Result<Span> {
    let mut stack: Vec<u8> = vec![];
    let end = tex.len();
    let mut span = Span::default();

    while start < end {
        if let Some(mat) = GROUP_SYMBOL_RE.find(&tex[start..]) {
            // Get the matched symbol string
            let symbol_str = mat.as_str();

            // A begin group symbol is found
            if symbol_str.starts_with("{") {
                // It is the first begin group symbol
                if stack.is_empty() {
                    span.start = start + mat.start();
                }

                // Push it to the stack
                stack.push(0);
            } else {
                // A end group symbol is found

                // Pop a symbol from the stack
                if let Some(_) = stack.pop() {
                    // The stack is empty,
                    // which means the matched end group symbol is found
                    if stack.is_empty() {
                        span.end = start + mat.end();
                        return Ok(span);
                    }
                } else {
                    // The stack is already empty
                    return Err(ParseError::GroupMismatch.into());
                }  
            }
            
            // Change the starting index
            start += mat.end();
        } else {
            return Err(ParseError::GroupMismatch.into());
        }
    }

    Err(ParseError::GroupMismatch.into())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{
        extract_group,
        find_group_span
    };

    #[test]
    fn succeed_in_finding_a_group_span() -> Result<()> {
        let tex = "{aaa{bbb}ccc{ddd}eee}fff";
        let span = find_group_span(tex, 0)?;
        println!("{}", &tex[span.start..span.end]);
        println!("{:#?}", extract_group(tex, 0));
        Ok(())
    }
}