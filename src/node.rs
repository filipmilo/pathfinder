use petgraph::graph::NodeIndex;

#[derive(PartialEq, Debug)]
pub struct Node {
    pub id: NodeIndex,
    pub name: String,
    pub neighbours: Vec<(NodeIndex, u32)>,
}
