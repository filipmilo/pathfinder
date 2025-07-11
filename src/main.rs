use std::collections::HashMap;
use std::fs;

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

#[derive(PartialEq, Debug)]
struct Node {
    name: String,
    neighbours: Vec<(NodeRef, u32)>,
}

use std::cmp::min;

fn held_karp(dist: &[Vec<u32>]) -> u32 {
    let n = dist.len();
    let size = 1 << n;

    // dp[mask][u] = minimum cost to reach city u with visited set = mask
    let mut dp = vec![vec![usize::MAX; n]; size];
    dp[1][0] = 0; // start at city 0

    for mask in 0..size {
        for u in 0..n {
            if (mask & (1 << u)) == 0 {
                continue;
            }
            for v in 0..n {
                if (mask & (1 << v)) != 0 || u == v {
                    continue;
                }
                let next_mask = mask | (1 << v);
                dp[next_mask][v] = min(
                    dp[next_mask][v],
                    dp[mask][u].saturating_add(dist[u][v] as usize),
                );
            }
        }
    }

    // Return to start city (0)
    let full_mask = (1 << n) - 1;
    let mut result = usize::MAX;

    println!("{:?}", dp[full_mask]);

    for u in 1..n {
        result = min(result, dp[full_mask][u].saturating_add(dist[u][0] as usize));
    }

    result as u32
}

fn main() {
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

    println!("{}", held_karp(&matrix));
}
