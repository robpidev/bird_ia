use lib_simulation as sim;
use rand::{rng, rngs::ThreadRng};
use wasm_bindgen::prelude::wasm_bindgen;

// === Simulation ===
#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn train(&mut self) -> Data {
        let (statas, info) = self.sim.train(&mut self.rng);

        Data {
            stats: Stats::from(statas),
            info: Information::from(info),
        }
    }
}

// === World ===
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,

    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();

        Self { animals, foods }
    }
}

// === Animal ===
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

// === Food ===
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}

// === Stats ===
#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Stats {
    pub min: f32,
    pub max: f32,
    pub avg: f32,
}

impl From<sim::Statistics> for Stats {
    fn from(stats: sim::Statistics) -> Self {
        Self {
            min: stats.min_fitness,
            max: stats.max_fitness,
            avg: stats.average_fitness,
        }
    }
}

// === Information ===
#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Information {
    pub generation: usize,
}

impl From<sim::Information> for Information {
    fn from(info: sim::Information) -> Self {
        Self {
            generation: info.generation(),
        }
    }
}

// === Data ===
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Data {
    pub stats: Stats,
    pub info: Information,
}
