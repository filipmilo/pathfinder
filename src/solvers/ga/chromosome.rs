use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Chromosome {
    pub gnome: Vec<usize>,
    pub fitness: u32,
}

impl Ord for Chromosome {
    fn cmp(&self, other: &Self) -> Ordering {
        other.fitness.cmp(&self.fitness)
    }
}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for Chromosome {}

impl Chromosome {
    pub fn new(matrix: &[Vec<u32>], gnome: Vec<usize>) -> Self {
        let fitness = Self::fitness(&gnome, matrix);

        Self { gnome, fitness }
    }

    pub fn fitness(gnome: &[usize], matrix: &[Vec<u32>]) -> u32 {
        gnome
            .windows(2)
            .map(|current| matrix[current[0]][current[1]])
            .sum()
    }
}
