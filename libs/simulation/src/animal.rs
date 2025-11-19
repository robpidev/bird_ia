use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};

use crate::eye::Eye;

pub struct Animal {
    pub(crate) positon: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::Network,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = nn::Network::random(
            rng,
            &[
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
            ],
        );

        Self {
            positon: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eye,
            brain,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.positon
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
