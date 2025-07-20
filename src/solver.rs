use std::{
    cmp::{Ordering, min},
    collections::VecDeque,
};

use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
    seq::SliceRandom,
};

pub trait GeneticAlgorithm {
    fn solve(&self) -> (u32, Vec<usize>);
    fn random_gnome(&self) -> Vec<usize>;
    fn crossover(&self, parent_1: &Chromosome, parent_2: &Chromosome) -> (Chromosome, Chromosome);
    fn mutate(&self, individual: &mut Chromosome);
    fn select(&self, population: &[Chromosome]) -> (usize, usize);
}

#[derive(Debug, Clone)]
pub struct Chromosome {
    gnome: Vec<usize>,
    fitness: u32,
}

impl Ord for Chromosome {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for Chromosome {}

impl Chromosome {
    fn new(matrix: &[Vec<u32>], gnome: Vec<usize>) -> Self {
        let fitness = Self::fitness(&gnome, matrix);

        Self { gnome, fitness }
    }

    fn fitness(gnome: &[usize], matrix: &[Vec<u32>]) -> u32 {
        gnome
            .windows(2)
            .map(|current| matrix[current[0]][current[1]])
            .fold(0, |acc, curr| acc.saturating_add(curr))
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
        let gen_threshold = 1000000;

        let mut population: Vec<Chromosome> = (1..self.matrix.len())
            .map(|_| Chromosome::new(&self.matrix, self.random_gnome()))
            .collect();

        population.sort();

        let pop_len = population.len();

        for _ in 0..gen_threshold {
            let mut new_population = population.clone();
            let mut replaced = 1;

            while replaced < pop_len {
                let (p_1, p_2) = self.select(&population);

                let (mut child_1, mut child_2) = if rand::rng().random::<f32>() < 0.7 {
                    self.crossover(&population[p_1], &population[p_2])
                } else {
                    (population[p_1].clone(), population[p_2].clone())
                };

                if rand::rng().random::<f32>() < 0.3 {
                    self.mutate(&mut child_1);
                }

                if rand::rng().random::<f32>() < 0.3 {
                    self.mutate(&mut child_2);
                }

                new_population[replaced] = child_1;
                replaced += 1;

                if replaced < pop_len {
                    new_population[replaced] = child_2;
                    replaced += 1;
                }
            }

            population = new_population;

            population.sort();
        }

        let minimum = population.iter().min_by(|x, y| x.fitness.cmp(&y.fitness));

        println!("{:?}", population);

        match minimum {
            Some(val) => (val.fitness, val.gnome.clone()),
            None => (0, vec![]),
        }
    }

    fn random_gnome(&self) -> Vec<usize> {
        let len = self.matrix.len();
        let mut path: Vec<usize> = vec![0];

        let mut cities: Vec<usize> = (1..len).collect();
        cities.shuffle(&mut rand::rng());

        path.append(&mut cities);
        path.push(0);
        path
    }

    fn select(&self, population: &[Chromosome]) -> (usize, usize) {
        let weights: Vec<u32> = population.iter().map(|ind| ind.fitness).collect();
        let dist = WeightedIndex::new(&weights).expect("Invalid weights.");
        let mut rng = rand::rng();

        (dist.sample(&mut rng), dist.sample(&mut rng))
    }

    fn crossover(&self, parent_1: &Chromosome, parent_2: &Chromosome) -> (Chromosome, Chromosome) {
        let mut cities: Vec<usize> = (1..self.matrix.len()).collect();
        cities.shuffle(&mut rand::rng());

        let chosen = cities[..3].to_vec();

        let order_1 = parent_1
            .gnome
            .iter()
            .filter_map(|city| {
                if chosen.contains(city) {
                    Some(*city)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let order_2 = parent_2
            .gnome
            .iter()
            .filter_map(|city| {
                if chosen.contains(city) {
                    Some(*city)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let (child_1, child_2) = order_1.iter().zip(order_2).fold(
            (parent_1.gnome.clone(), parent_2.gnome.clone()),
            |mut acc, curr| {
                if let Some(found) = acc.0.iter_mut().find(|city| *city == curr.0) {
                    *found = curr.1
                }

                if let Some(found) = acc.1.iter_mut().find(|city| **city == curr.1) {
                    *found = *curr.0
                }

                acc
            },
        );

        (
            Chromosome::new(&self.matrix, child_1),
            Chromosome::new(&self.matrix, child_2),
        )
    }

    fn mutate(&self, individual: &mut Chromosome) {
        let len = self.matrix.len();

        let (r, r1): (usize, usize) = (|&len| loop {
            let rr = rand::rng().random_range(1..len);
            let rr1 = rand::rng().random_range(1..len);

            if rr != rr1 {
                return (rr, rr1);
            }
        })(&len);

        let mut gnome = individual.gnome.clone();

        gnome.swap(r, r1);

        let fitness = Chromosome::fitness(&gnome, &self.matrix);

        individual.gnome = gnome;
        individual.fitness = fitness;
    }
}
