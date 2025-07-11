use std::cmp::min;

use crate::node::Node;

pub struct Graph {
    matrix: Vec<Vec<u32>>,
    values: Vec<Node>,
}

impl Graph {
    pub fn new(matrix: Vec<Vec<u32>>, values: Vec<Node>) -> Graph {
        Graph { matrix, values }
    }

    pub fn sequential_held_karp(&self) -> u32 {
        let n = self.matrix.len();
        let size = 1 << n;

        let mut dp = vec![vec![usize::MAX; n]; size];
        dp[1][0] = 0;

        for mask in 0..size {
            println!("{}", mask);
            println!("{:?}", dp);

            for u in 0..n {
                println!("{}", mask & (1 << u));
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
                        dp[mask][u].saturating_add(self.matrix[u][v] as usize),
                    );
                }
            }
        }

        let full_mask = (1 << n) - 1;
        let mut result = usize::MAX;

        println!("{:?}", dp[full_mask]);

        for u in 1..n {
            result = min(
                result,
                dp[full_mask][u].saturating_add(self.matrix[u][0] as usize),
            );
        }

        result as u32
    }
}
