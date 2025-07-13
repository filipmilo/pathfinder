use petgraph::graph::{EdgeIndex, NodeIndex};

#[derive(PartialEq, Debug)]
pub struct Node {
    pub id: NodeIndex,
    pub name: String,
    pub neighbours: Vec<(NodeIndex, u32, Option<EdgeIndex>)>,
}

impl Node {
    pub fn get_edge_idxs(&self, filter_out: &[usize]) -> Vec<EdgeIndex> {
        self.neighbours
            .iter()
            .filter_map(|edge| {
                if !filter_out.contains(&edge.0.index()) {
                    Some(edge.2.unwrap())
                } else {
                    None
                }
            })
            .collect()
    }
}
