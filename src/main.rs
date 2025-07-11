use std::collections::{HashMap, HashSet};
use std::fs;

use node::Node;

mod graph;
mod node;

fn load_input() -> (Vec<Vec<u32>>, Vec<Node>) {
    let lines = fs::read_to_string("data/basic.txt")
        .expect("Oops, could not open file.")
        .lines()
        .map(|line| {
            let parsed = line.split(",").collect::<Vec<&str>>();

            (
                parsed[0].to_owned() + parsed[1],
                parsed[2].to_owned() + parsed[3],
                parsed[4].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(String, String, u32)>>();

    let ids = lines
        .iter()
        .flat_map(|line| [line.0.clone(), line.1.clone()])
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    let nodes = lines
        .iter()
        .fold(HashMap::new(), |mut acc, curr| -> HashMap<usize, Node> {
            let curr_id = ids.iter().position(|name| *name == curr.0).unwrap();
            let end_id = ids.iter().position(|name| *name == curr.1).unwrap();

            acc.entry(curr_id)
                .and_modify(|node| node.neighbours.push((end_id, curr.2)))
                .or_insert(Node {
                    id: curr_id,
                    name: curr.0.clone(),
                    neighbours: vec![],
                });

            acc
        });

    let len = ids.len();
    let mut matrix: Vec<Vec<u32>> = (0..len).map(|_| vec![u32::MAX; len]).collect();

    let values = nodes.into_values().collect::<Vec<Node>>();

    for (i, column) in matrix.iter_mut().enumerate() {
        for (j, val) in column.iter_mut().enumerate() {
            if j == i {
                continue;
            }

            if let Some(begin) = values.iter().find(|node| node.id == j) {
                if let Some(found) = begin
                    .neighbours
                    .iter()
                    .find_map(|n| if n.0 == i { Some(n.1) } else { None })
                {
                    *val = found;
                }
            }
        }
    }

    (matrix, values)
}

fn main() {
    let (matrix, values) = load_input();

    let graph = graph::Graph::new(matrix, values);

    println!("{}", graph.sequential_held_karp());
}
