use std::collections::HashMap;
use std::fs;

use std::cell::RefCell;
use std::rc::Rc;

mod graph;

type NodeRef = Rc<RefCell<Node>>;

#[derive(PartialEq, Debug)]
struct Node {
    name: String,
    neighbours: Vec<(NodeRef, u32)>,
}

fn load_input() -> Vec<Vec<u32>> {
    let nodes = fs::read_to_string("data/input.txt")
        .expect("Oops, could not open file.")
        .lines()
        .fold(
            HashMap::new(),
            |mut acc, curr| -> HashMap<String, NodeRef> {
                let parsed = curr.split(",").collect::<Vec<&str>>();

                let begin = parsed[0].to_owned() + parsed[1];
                let end = parsed[2].to_owned() + parsed[3];
                let cost = parsed[4].parse::<u32>().unwrap();

                let end_node = acc
                    .entry(end.clone())
                    .or_insert(Rc::new(RefCell::new(Node {
                        name: end,
                        neighbours: vec![],
                    })))
                    .clone();

                acc.entry(begin.clone())
                    .and_modify(|node| node.borrow_mut().neighbours.push((end_node, cost)))
                    .or_insert(Rc::new(RefCell::new(Node {
                        name: begin,
                        neighbours: vec![],
                    })));

                acc
            },
        );

    let len = nodes.len();
    let mut matrix: Vec<Vec<u32>> = (0..len).map(|_| vec![u32::MAX; len]).collect();

    let values = nodes.values().collect::<Vec<&NodeRef>>();

    for (i, &curr) in values.iter().enumerate() {
        for (j, &target) in values.iter().enumerate() {
            if j == i {
                continue;
            }

            if let Some(found) = curr
                .borrow()
                .neighbours
                .iter()
                .find_map(|n| if n.0 == (*target) { Some(n.1) } else { None })
            {
                matrix[i][j] = found;
            }
        }
    }

    matrix
}

fn main() {
    let graph = graph::Graph::new(load_input());

    println!("{}", graph.sequential_held_karp());
}
