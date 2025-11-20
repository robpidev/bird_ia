use lib_genetic_algorithm as ga;

use crate::Animal;

pub struct AnimalIndividual {
    pub fitness: f32,
    pub chromosome: ga::Crhomosome,
}

impl AnimalIndividual {
    pub fn into_animal(self, rng: &mut dyn rand::RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Crhomosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Crhomosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl From<&Animal> for AnimalIndividual {
    fn from(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_crhomosome(),
        }
    }
}
