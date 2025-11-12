use rand::{Rng, RngCore};

pub struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weigth)| input * weigth)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pub fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        // let mut rng = rand::rng();

        let bias = rng.random_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha12Rng;

    macro_rules! assert_almost_eq {
        ($left:expr, $right:expr) => {{
            let left: f32 = $left;
            let right: f32 = $right;
            assert!((left - right).abs() < f32::EPSILON);
        }};
    }

    #[test]
    fn random() {
        let mut rng = ChaCha12Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_almost_eq!(neuron.bias, -0.1671462);

        let expected_weights = vec![-0.3439536, -0.8530848, 0.66806483, 0.4668932];
        neuron
            .weights
            .iter()
            .zip(expected_weights)
            .for_each(|(weight, expected_weight)| assert_almost_eq!(*weight, expected_weight));
    }

    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        // Ensures .max() (our ReLU) works:
        assert_almost_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);

        // `0.5` and `1.0` chosen by a fair dice roll
        assert_almost_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        );
    }
}
