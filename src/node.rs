#[derive(PartialEq, Debug)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub neighbours: Vec<(usize, u32)>,
}
