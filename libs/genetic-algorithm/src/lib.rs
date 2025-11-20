mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;

// === Exports ===
pub use chromosome::Crhomosome;
pub use crossover::UniformCrossover;
pub use individual::Individual;
pub use mutation::GaussianMutation;
pub use selection::RouletteWheelSelection;

// === Internal ===
use crossover::CrossoverMethod;
use mutation::MutationMethod;
use rand::RngCore;
use selection::SelectionMethod;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn envolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        (0..population.len())
            .map(|_| {
                // Selection
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::{
        chromosome::Crhomosome, crossover::UniformCrossover, mutation::GaussianMutation,
        selection::RouletteWheelSelection,
    };

    use super::*;

    #[test]
    fn genetic_algorithm() {
        #[derive(Clone, Debug, PartialEq)]
        struct TestIndividual {
            chromosome: Crhomosome,
        }

        impl Individual for TestIndividual {
            fn create(chromosome: Crhomosome) -> Self {
                Self { chromosome }
            }

            fn chromosome(&self) -> &Crhomosome {
                &self.chromosome
            }

            fn fitness(&self) -> f32 {
                self.chromosome().iter().sum()
            }
        }

        fn individual(genes: &[f32]) -> TestIndividual {
            TestIndividual::create(genes.iter().cloned().collect())
        }

        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        for _ in 0..10 {
            population = ga.envolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[1.606008, 2.789879, 3.6941864]),
            individual(&[1.0839049, 2.4461222, -0.8869108]),
            individual(&[0.99193525, 2.588976, 3.5712361]),
            individual(&[1.646358, 2.392836, 3.9752667]),
        ];

        assert_eq!(population, expected_population);
    }
}
