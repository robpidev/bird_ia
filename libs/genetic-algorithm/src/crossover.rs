use rand::{Rng, RngCore};

use crate::chromosome::Crhomosome;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Crhomosome,
        parent_b: &Crhomosome,
    ) -> Crhomosome;
}

pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Crhomosome,
        parent_b: &Crhomosome,
    ) -> Crhomosome {
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.random_bool(0.5) { a } else { b })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::{
        chromosome::Crhomosome,
        crossover::{CrossoverMethod, UniformCrossover},
    };

    #[test]
    fn uniform_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Crhomosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Crhomosome = (1..=100).map(|n| -n as f32).collect();

        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);
        let diff_a = child
            .iter()
            .zip(parent_a.iter())
            .filter(|(c, p)| c != p)
            .count();
        let diff_b = child
            .iter()
            .zip(parent_b.iter())
            .filter(|(c, p)| c != p)
            .count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
