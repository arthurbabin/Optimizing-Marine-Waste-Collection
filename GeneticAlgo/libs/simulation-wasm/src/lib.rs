use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use serde::Serialize;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[derive(Clone, Debug, Serialize)]
pub struct Sea {
    pub collectors: Vec<Collector>,
    pub wastes: Vec<Waste>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Collector {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct Waste {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);

        format!(
            "min={:.2}, max={:.2}, avg={:.2}",
            stats.min_fitness(),
            stats.max_fitness(),
            stats.avg_fitness()
        )
    }
    pub fn sea(&self) -> JsValue {
        let sea= Sea::from(self.sim.sea());
        JsValue::from_serde(&sea).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }
}

impl From<&sim::Sea> for Sea {
    fn from(sea: &sim::Sea) -> Self {
        let collectors = sea
            .collectors()
            .iter()
            .map(Collector::from)
            .collect();

        let wastes = sea 
            .wastes()
            .iter()
            .map(Waste::from)
            .collect();

        Self { collectors, wastes }
    }
}

impl From<&sim::Collector> for Collector {
    fn from(collector: &sim::Collector) -> Self {
        Self {
            x: collector.position().x,
            y: collector.position().y,
            rotation: collector.rotation().angle(),
        }
    }
}

impl From<&sim::Waste> for Waste {
    fn from(waste: &sim::Waste) -> Self {
        Self {
            x: waste.position().x,
            y: waste.position().y,
        }
    }
}
