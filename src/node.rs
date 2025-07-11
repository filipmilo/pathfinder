use std::{cell::RefCell, rc::Rc};

pub type NodeRef = Rc<RefCell<Node>>;

#[derive(PartialEq, Debug)]
pub struct Node {
    pub name: String,
    pub neighbours: Vec<(NodeRef, u32)>,
}
