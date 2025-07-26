use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
    seq::SliceRandom,
};

use crate::solvers::ga::chromosome::Chromosome;

use super::ga_trait::GeneticAlgorithm;

pub struct SequentialGASolver {
    matrix: Vec<Vec<u32>>,
}

impl SequentialGASolver {
    pub fn new(matrix: Vec<Vec<u32>>) -> Self {
        Self { matrix }
    }
}

impl GeneticAlgorithm for SequentialGASolver {
    fn solve(&self) -> (u32, Vec<usize>) {
        let gen_threshold = 100000;

        let mut population: Vec<Chromosome> = (1..1000)
            .map(|_| Chromosome::new(&self.matrix, self.random_gnome()))
            .collect();

        population.sort();

        let pop_len = population.len();

        for _ in 0..gen_threshold {
            let mut new_population = population.clone();
            let mut replaced = 3;

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
        let inverted: Vec<f64> = weights.iter().map(|&w| 1.0 / (w as f64 + 1.0)).collect();
        let sum: f64 = inverted.iter().sum();
        let normalized: Vec<u64> = inverted
            .iter()
            .map(|&inv| ((inv / sum) * 1000.0).round() as u64)
            .collect();
        let dist = WeightedIndex::new(&normalized).unwrap();
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
