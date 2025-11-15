use crate::chromosome::Crhomosome;
use rand::{RngCore, seq::IndexedRandom};

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Crhomosome;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |i| i.fitness())
            .expect("got an empy population")
    }
}
