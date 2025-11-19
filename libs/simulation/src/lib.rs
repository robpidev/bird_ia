mod animal;
mod eye;
mod food;
mod world;

use nalgebra::{self as na};
use rand::{Rng, RngCore};

pub use self::animal::Animal;
pub use self::food::Food;
pub use self::world::World;

// === Simulation ===
pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movement();
    }

    fn process_movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.positon += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.positon.x = na::wrap(animal.positon.x, 0.0, 1.0);
            animal.positon.y = na::wrap(animal.positon.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position(), &food.position());

                if distance < 0.01 {
                    food.position = rng.random();
                }
            }
        }
    }
}
