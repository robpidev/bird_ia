use std::iter::once;

use rand::RngCore;

mod neuron;
use neuron::Neuron;

// === Network ===

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|l| l.neurons.iter())
            .flat_map(|n| once(&n.bias).chain(&n.weights))
            .copied()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|l| Layer::from_weights(l[0].neurons, l[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("Too many weights");
        }

        Self { layers }
    }
}

// === Layer ===

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Self { neurons }
    }

    fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

// === Topology ===

pub struct LayerTopology {
    pub neurons: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights() {
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.1,
                        weights: vec![0.1, 0.2, 0.3],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.5,
                        weights: vec![0.6, 0.7, 0.8],
                    }],
                },
            ],
        };

        let actual: Vec<_> = network.weights().collect();
        let expected = vec![0.1, 0.1, 0.2, 0.3, 0.5, 0.6, 0.7, 0.8];

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_weights() {
        let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

        let weights = vec![0.1, 0.1, 0.2, 0.3, 0.5, 0.6, 0.7, 0.8];
        let network = Network::from_weights(layers, weights.clone());
        let actual: Vec<_> = network.weights().collect();

        assert_eq!(actual, weights);
    }
}
