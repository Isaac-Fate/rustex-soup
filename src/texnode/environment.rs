use std::collections::VecDeque;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::{Regex, Match};

use crate::ParseError;
use super::{
    node::{TexNode, extract_nodes},
    span::Span, 
    arg::Arg
};

lazy_static! {
    static ref BEGIN_ENVIRONMENT_RE: Regex = Regex::new(r"^\\begin\{[a-zA-Z]+\*?\}")
        .unwrap();
    static ref ENVIRONMENT_SYMBOL_RE: Regex = Regex::new(r"\\(begin|end)\{[a-zA-Z]+\*?\}")
        .unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub span: Span,
    pub name: String,
    pub args: Vec<Arg>,
    pub children: Vec<TexNode>
}

pub fn extract_environment(tex: &str, start: usize) -> Option<Environment> {
    if let Some(mat) = BEGIN_ENVIRONMENT_RE.find(&tex[start..]) {
        let (span, inner_nodes_span) = find_environment_and_inner_nodes_spans(tex, start)
            .unwrap();
        assert_eq!(span.start, start);

        // Get environment name
        let name = get_environment_name(&mat);

        println!("{}", name);

        // Nodes inside the environment
        let nodes = extract_nodes(
                tex, 
                inner_nodes_span.start, 
                inner_nodes_span.end
            )
            .unwrap();

        // Find args
        let mut args: Vec<Arg> = vec![];
        let mut nodes = VecDeque::from(nodes);
        while !nodes.is_empty() {
            if let Some(node) = nodes.pop_front() {
                match node {
                    TexNode::Group(group) => {
                        args.push(Arg::Mandatory { children: group.children });
                    },
                    TexNode::SquareGroup(square_group) => {
                        args.push(Arg::Optional { children: square_group.children });
                    },
                    _ => {
                        nodes.push_front(node);
                        break;
                    }
                }
            }
        }
        
        Some(Environment { 
            span, 
            name,
            args,
            children: nodes.into()
        })

    } else {
        None
    }
}

fn find_environment_and_inner_nodes_spans(
    tex: &str, 
    mut start: usize
) -> Result<(Span, Span)> {
    let mut environment_name_stack: Vec<String> = vec![];
    let mut environment_span = Span::default();
    let mut inner_nodes_span = Span::default();

    while start < tex.len() {
        if let Some(mat) = ENVIRONMENT_SYMBOL_RE.find(&tex[start..]) {
            // Get environment name
            let name = get_environment_name(&mat);
    
            // A begin environment symbol is found
            if mat.as_str().starts_with(r"\begin") {
                // Set the `start` fields of both spans
                if environment_name_stack.is_empty() {
                    environment_span.start = start + mat.start();
                    inner_nodes_span.start = start + mat.end();
                }
    
                // Push the name to the stack
                environment_name_stack.push(name);

            } else if let Some(begin_environment_name) = environment_name_stack.pop() {
                // An end environment symbol is found
                // Pop a begin environment name from the stack

                if begin_environment_name == name {
                    // The stack is empty,
                    // which means the environment is matched
                    if environment_name_stack.is_empty() {
                        environment_span.end = start + mat.end();
                        inner_nodes_span.end = start + mat.start();

                        return Ok((
                            environment_span,
                            inner_nodes_span
                        ));
                    }
                } else {
                    return Err(ParseError::EnvironmentMismatch.into());
                }
            } else {
                return Err(ParseError::EnvironmentMismatch.into());
            }
    
            // Update the starting index
            start += mat.end();

        } else {
            return Err(ParseError::EnvironmentMismatch.into());
        }
    }

    Err(ParseError::EnvironmentMismatch.into())
}

fn get_environment_name(mat: &Match) -> String {
    let mat_str = mat.as_str();
    mat_str[
        mat_str.find('{').unwrap() + 1
        ..
        mat_str.find('}').unwrap()
    ]
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        ENVIRONMENT_SYMBOL_RE,
        extract_environment
    };

    #[test]
    fn test_environment_symbol_re() {
        let begin_environment = r#"\begin{theorem}"#;
        let end_environment = r#"\end{theorem}"#;

        let mat = ENVIRONMENT_SYMBOL_RE
            .find(begin_environment)
            .unwrap();
        println!("{:?}", mat.as_str());

        let mat = ENVIRONMENT_SYMBOL_RE
            .find(end_environment)
            .unwrap();
        println!("{:?}", mat.as_str());
    }

    #[test]
    fn succeed_in_extracting_environment() {
        let tex = r#"\begin{theorem}[Taylor's Theorem]
This is an important theorem.
\end{theorem}
"#;
        let environment = extract_environment(tex, 0)
            .unwrap();
        println!("{:#?}", environment);
    }
}
