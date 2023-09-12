use anyhow::Result;

use crate::error::ParseError;
use super::{
    span::Span,
    text::{Text, extract_text},
    comment::{Comment, extract_comment},
    inline_math::{InlineMath, extract_inline_math},
    group::{Group, extract_group},
    square_group::{SquareGroup, extract_square_group},
    command::{Command, extract_command},
    environment::{Environment, extract_environment}
};

#[derive(Debug, Clone, PartialEq)]
pub enum TexNode {
    Text(Text),
    Comment(Comment),
    InlineMath(InlineMath),
    Group(Group),
    SquareGroup(SquareGroup),
    Command(Command),
    Environment(Environment)
}

impl TexNode {
    pub fn span(&self) -> Span {
        match self {
            Self::Text(text) => text.span,
            Self::Comment(comment) => comment.span,
            Self::InlineMath(inline_math) => inline_math.span,
            Self::Group(group) => group.span,
            Self::SquareGroup(square_group) => square_group.span,
            Self::Command(command) => command.span,
            Self::Environment(environment) => environment.span
        }
    }
}

pub fn extract_node(tex: &str, start: usize) -> Result<TexNode> {
    if let Some(square_group) = extract_square_group(tex, start) {
        Ok(TexNode::SquareGroup(square_group))
    } else if let Some(text) = extract_text(tex, start) {
        Ok(TexNode::Text(text))
    } else if let Some(comment) = extract_comment(tex, start) {
        Ok(TexNode::Comment(comment))
    } else if let Some(inline_math) = extract_inline_math(tex, start) {
        Ok(TexNode::InlineMath(inline_math))
    } else if let Some(group) = extract_group(tex, start) {
        Ok(TexNode::Group(group))
    } else if let Some(environment) = extract_environment(tex, start) {
        Ok(TexNode::Environment(environment))
    } else if let Some(command) = extract_command(tex, start) {
        Ok(TexNode::Command(command))
    } else {
        Err(ParseError::UnknownTexNode(tex[start..].to_string()).into())
    }
}

pub fn extract_nodes(tex: &str, mut start: usize, end: usize) -> Result<Vec<TexNode>> {
    let mut nodes: Vec<TexNode> = vec![];
    while start < end {
        let node = extract_node(&tex[..end], start)?;
        start = node.span().end;
        nodes.push(node);
    }
    
    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::{
        TexNode,
        Span,
        Text,
        extract_node,
        extract_nodes
    };

    const TEX: &'static str = r#"
\section{Introduction}
Hello, world!
\section{Theory}
An operator $T$ is \textbf{self-adjoint} if $T^\ast = T$, i.e.,
it is own adjoint.
"#;

    #[test]
    fn succeed_in_extracting_nodes() {
        let nodes = extract_nodes(TEX, 0, TEX.len()).unwrap();
        println!("{:#?}", nodes);
    }

    #[test]
    fn succeed_in_extracting_text_nodes() {
        let text = extract_node(TEX, 0).unwrap();
        assert_eq!(
            text, 
            TexNode::Text(
                Text {
                    span: Span { start: 0, end: 1 },
                    content: "\n".to_string()
                }
            )
        );
    }

    #[test]
    fn fail_to_extract_text_nodes() {
        assert!(extract_node(TEX, 1).is_err());
    }

}
