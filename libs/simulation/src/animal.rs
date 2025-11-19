use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Animal {
    pub(crate) positon: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            positon: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.positon
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
