use crate::animal::Animal;
use crate::food::Food;
use rand::RngCore;

pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    /// Create a random world with animals and foods
    pub fn random(rng: &mut dyn RngCore, animals_count: usize, food_count: usize) -> Self {
        let animals = (0..animals_count).map(|_| Animal::random(rng)).collect();
        let foods = (0..food_count).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}
