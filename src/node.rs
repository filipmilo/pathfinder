use petgraph::graph::{EdgeIndex, NodeIndex};

#[derive(PartialEq, Debug)]
pub struct Node {
    pub id: NodeIndex,
    pub name: String,
    pub neighbours: Vec<(NodeIndex, u32, Option<EdgeIndex>)>,
}
