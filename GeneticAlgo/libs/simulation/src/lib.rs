pub use self::{collector::*, eye::*,brain::*, waste::*, sea::*};

mod collector;
mod collector_individual;
mod eye;
mod brain;
mod waste;
mod sea;

use self::collector_individual::*;
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};

use std::f32::consts::FRAC_PI_2;
const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

/// Number of steps for each generation
const GENERATION_LENGTH: usize = 2500; 

pub struct Simulation{
    pub(crate) sea: Sea,
    pub(crate) ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    pub(crate) age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let sea = Sea::random(rng);

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(0.01, 0.3),
        );
        Self {
            sea: Sea::random(rng),
            ga: ga,
            age: 0,
        }
    }

    pub fn sea(&self) -> &Sea{
        &self.sea
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    fn process_movements(&mut self) {
        for collector in &mut self.sea.collectors {
            collector.position += 
                collector.rotation * na::Vector2::new(0.0, collector.speed);
            collector.position.x = na::wrap(collector.position.x, 0.0, 1.0);
            collector.position.y = na::wrap(collector.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for collector in &mut self.sea.collectors {
            for waste in &mut self.sea.wastes {
                let distance = na::distance(
                    &collector.position,
                    &waste.position,
                );

                if distance <= 0.01 {
                    collector.proficiency += 1;
                    waste.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for collector in &mut self.sea.collectors {
            let vision = collector.eye.process_vision(
                collector.position,
                collector.rotation,
                &self.sea.wastes,
            );

            let response = collector.brain.nn.propagate(vision);

            let speed = response[0].clamp(
                -SPEED_ACCEL,
                SPEED_ACCEL,
            );

            let rotation = response[1].clamp(
                -ROTATION_ACCEL,
                ROTATION_ACCEL,
            );

            collector.speed =
                (collector.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            collector.rotation = na::Rotation2::new(
                collector.rotation.angle() + rotation,
            );
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {    self.age = 0;
        // Transforms `Vec<Collector>` to `Vec<CollectorIndividual>`
        // in order perform evolution with the genetic algorithm 
        let current_population: Vec<_> = self
            .sea
            .collectors
            .iter()
            .map(CollectorIndividual::from_collector)
            .collect();

        // Evolve
        let (evolved_population, stats) = self.ga.evolve(
            rng,
            &current_population,
        );

        // Transforms `Vec<CollectorIndividual>` back into `Vec<Collector>`
        self.sea.collectors = evolved_population
            .into_iter()
            .map(|individual| individual.into_collector(rng))
            .collect();

        // Restart wastes
        for waste in &mut self.sea.wastes {
            waste.position = rng.gen();
        }

        stats
    }
}
