mod animal;
mod eye;
mod food;
mod world;

use std::f32::consts::FRAC_PI_2;

use nalgebra::{self as na};
use rand::{Rng, RngCore};

pub use self::animal::Animal;
pub use self::food::Food;
pub use self::world::World;

// === Const ===
const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

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
        self.process_brains();
        self.process_movement();
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

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.positon, animal.rotation, &self.world.foods);

            let res = animal.brain.propagate(vision);
            let speed = res[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = res[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            // rotation absolute to relative
            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    fn process_movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.positon += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.positon.x = na::wrap(animal.positon.x, 0.0, 1.0);
            animal.positon.y = na::wrap(animal.positon.y, 0.0, 1.0);
        }
    }
}
