use std::{cmp::min, collections::VecDeque};

use rand::{Rng, seq::SliceRandom};

pub trait GeneticAlgorithm {
    fn solve(&self) -> (u32, Vec<usize>);
}

#[derive(Debug)]
struct Chromosome {
    gnome: Vec<usize>,
    fitness: u32,
}

impl Chromosome {
    fn new(matrix: &[Vec<u32>]) -> Self {
        let gnome = Self::random_gnome(matrix.len());

        let fitness = Self::fitness(&gnome, matrix);

        Self { gnome, fitness }
    }

    fn fitness(gnome: &[usize], matrix: &[Vec<u32>]) -> u32 {
        gnome
            .windows(2)
            .map(|current| matrix[current[0]][current[1]])
            .fold(0, |acc, curr| acc.saturating_add(curr))
    }

    fn random_gnome(len: usize) -> Vec<usize> {
        let mut cities: Vec<usize> = (0..len).collect();
        cities.shuffle(&mut rand::rng());
        cities.push(cities[0]);
        cities
    }

    fn mutate_offspring(&self, matrix: &[Vec<u32>]) -> Self {
        let len = matrix.len();

        let (r, r1): (usize, usize) = (|&len| loop {
            let rr = rand::rng().random_range(1..len - 1);
            let rr1 = rand::rng().random_range(1..len - 1);

            if rr != rr1 {
                return (rr, rr1);
            }
        })(&len);

        let mut gnome = self.gnome.clone();

        gnome.swap(r, r1);

        let fitness = Self::fitness(&gnome, matrix);

        Self { gnome, fitness }
    }
}

pub struct Solver {
    matrix: Vec<Vec<u32>>,
}

impl Solver {
    pub fn new(matrix: Vec<Vec<u32>>) -> Self {
        Self { matrix }
    }

    pub fn sequential_held_karp(&self) -> (u32, Vec<usize>) {
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

impl GeneticAlgorithm for Solver {
    fn solve(&self) -> (u32, Vec<usize>) {
        let mut gen_num = 1;
        let gen_threshold = 1000000;

        let mut population: Vec<Chromosome> = (1..self.matrix.len())
            .map(|_| Chromosome::new(&self.matrix))
            .collect();

        let mut temperature = 10000;

        while gen_num < gen_threshold {
            let mut new_population: Vec<Chromosome> = vec![];

            for i in population.iter() {
                loop {
                    let new_chromosome = i.mutate_offspring(&self.matrix);

                    if new_chromosome.fitness <= i.fitness
                        || (2.7f64).powf(
                            -((new_chromosome.fitness - i.fitness) as f64 / temperature as f64),
                        ) > 0.5
                    {
                        new_population.push(new_chromosome);
                        break;
                    }
                }
            }

            temperature = (90 * temperature) / 100;

            population = new_population;
            gen_num += 1;
        }

        let minimum = population.iter().min_by(|x, y| x.fitness.cmp(&y.fitness));

        println!("{:?}", population);

        match minimum {
            Some(val) => (val.fitness, val.gnome.clone()),
            None => (0, vec![]),
        }
    }
}
