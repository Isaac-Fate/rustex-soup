use super::node::TexNode;

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Mandatory {children: Vec<TexNode>},
    Optional {children: Vec<TexNode>}
}
