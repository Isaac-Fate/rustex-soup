use lazy_static::lazy_static;
use regex::Regex;

use super::{
    node::{TexNode, extract_node},
    span::Span,
    arg::Arg
};

lazy_static! {
    static ref COMMAND_NAME_RE: Regex = Regex::new(r"^\\[a-zA-Z]+\*?")
        .unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub span: Span,
    pub name: String,
    pub args: Vec<Arg>
}

pub fn extract_command(tex: &str, mut start: usize) -> Option<Command> {
    // A tex command name with "\" prefix is found
    if let Some(mat) = COMMAND_NAME_RE.find(&tex[start..]) {
        // Set the `start` field of the span
        // the `end` is still unknown
        let mut span = Span {
            start,
            end: start + mat.end()
        };

        // Command name without "\" prefix
        let name = mat
            .as_str()
            .strip_prefix(r"\")
            .unwrap()
            .to_string();

        // Command args
        let mut args: Vec<Arg> = vec![];
        while start < tex.len() {
            // Extract the first node
            let node = extract_node(tex, span.end).unwrap();
            match node {
                // It is a group, 
                // which should be regarded as a mandatory arg
                TexNode::Group(group) => {
                    span.end = group.span.end;
                    start = group.span.end;
                    args.push(Arg::Mandatory { children: group.children });
                },

                // It is a square group, 
                // which should be regarded as an optional arg
                TexNode::SquareGroup(square_group) => {
                    span.end = square_group.span.end;
                    start = square_group.span.end;
                    args.push(Arg::Optional { children: square_group.children });
                },

                _ => {
                    break;
                }
            }
        }

        Some(Command { span, name, args })
    } else {
        None
    }
}