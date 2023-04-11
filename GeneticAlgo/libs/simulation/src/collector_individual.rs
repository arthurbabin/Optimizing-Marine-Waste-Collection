use crate::*;

pub struct CollectorIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}
impl ga::Individual for CollectorIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl CollectorIndividual {
    pub fn from_collector(collector: &Collector) -> Self {
        Self {
            fitness: collector.proficiency as f32,
            chromosome: collector.as_chromosome(),
        }
    }

    pub fn into_collector(self, rng: &mut dyn RngCore) -> Collector {
        Collector::from_chromosome(self.chromosome, rng)
    }
}
