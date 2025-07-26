use super::chromosome::Chromosome;

pub trait GeneticAlgorithm {
    fn solve(&self) -> (u32, Vec<usize>);
    fn random_gnome(&self) -> Vec<usize>;
    fn crossover(&self, parent_1: &Chromosome, parent_2: &Chromosome) -> (Chromosome, Chromosome);
    fn mutate(&self, individual: &mut Chromosome);
    fn select(&self, population: &[Chromosome]) -> (usize, usize);
}
