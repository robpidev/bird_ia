use lib_genetic_algorithm as ga;
use nalgebra as na;
use rand::{Rng, RngCore};

use crate::{brain::Brain, eye::Eye};

pub struct Animal {
    pub(crate) positon: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,
}

impl Animal {
    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            positon: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);
        Self::new(eye, brain, rng)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.positon
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub(crate) fn from_chromosome(chromosome: ga::Crhomosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_crhomosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_crhomosome(&self) -> ga::Crhomosome {
        self.brain.as_crhomosome()
    }
}
