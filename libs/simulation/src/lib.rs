mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

use std::f32::consts::FRAC_PI_2;

use lib_genetic_algorithm as ga;
use nalgebra::{self as na};
use rand::{Rng, RngCore};

use crate::animal_individual::AnimalIndividual;

pub use self::animal::Animal;
pub use self::food::Food;
pub use self::world::World;

// === Const ===
const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movement();

        self.age += 1;
        if self.age > GENERATION_LENGTH {
            self.envolve(rng);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position(), &food.position());

                if distance < 0.01 {
                    animal.satiation += 1;
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

            let res = animal.brain.nn.propagate(vision);
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

    fn envolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Step 1: Prepare the birds to be sent into the genetic algorithm
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from)
            .collect();

        // Step 2: Envolve birdies
        let envolved_population = self.ga.envolve(rng, &current_population);

        // Step 3: Bring birdies back from the genetic algorithm
        self.world.animals = envolved_population
            .into_iter()
            .map(|i| i.into_animal(rng))
            .collect();

        // Step 4: Restart foods
        for food in &mut self.world.foods {
            food.position = rng.random();
        }
    }
}
