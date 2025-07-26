use std::{cmp::min, collections::VecDeque};

pub struct DPSolver {
    matrix: Vec<Vec<u32>>,
}

impl DPSolver {
    pub fn new(matrix: Vec<Vec<u32>>) -> Self {
        Self { matrix }
    }
}

impl DPSolver {
    pub fn solve(&self) -> (u32, Vec<usize>) {
        let n = self.matrix.len();
        let size = 1 << n;

        let mut dp = vec![vec![usize::MAX; n]; size];
        dp[1][0] = 0;

        for mask in 1..size {
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
                        dp[mask][u].saturating_add(self.matrix[u][v] as usize),
                    );
                }
            }
        }

        let full_mask = (1 << n) - 1;
        let mut result = usize::MAX;
        let mut last_city = 0;

        for u in 1..n {
            let cost = dp[full_mask][u].saturating_add(self.matrix[u][0] as usize);

            if cost < result {
                result = cost;
                last_city = u;
            }
        }

        let mut tour_rev = VecDeque::with_capacity(n + 1);
        let mut mask = full_mask;
        let mut city = last_city;

        tour_rev.push_front(0);
        tour_rev.push_front(city);

        while mask != 1 {
            let prev_mask = mask ^ (1 << city);

            let mut prev_city = 0;
            for u in 0..n {
                if prev_mask & (1 << u) == 0 {
                    continue;
                }
                let cost_through_u = dp[prev_mask][u].saturating_add(self.matrix[u][city] as usize);
                if cost_through_u == dp[mask][city] {
                    prev_city = u;
                    break;
                }
            }

            tour_rev.push_front(prev_city);
            city = prev_city;
            mask = prev_mask;
        }

        (result as u32, tour_rev.into_iter().collect())
    }
}
