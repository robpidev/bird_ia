use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use rand::RngCore;

use crate::eye::Eye;

pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            // every eye's cell is a input
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            // hidden layer
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            // this cotrols the speed and direction
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
        ]
    }

    pub(crate) fn from_crhomosome(crhomosome: ga::Crhomosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), crhomosome),
        }
    }

    pub(crate) fn as_crhomosome(&self) -> ga::Crhomosome {
        self.nn.weights().collect()
    }
}
