mod active_genome;
mod active_plant;
mod blob;
mod doublet;
mod doublet_fn;
mod either;
mod entity;
mod genome;
mod genomes;
mod grid;
mod inactive_genome;
mod inactive_plant;
mod neighbors;
mod organisms;
mod plants;
mod position;
mod rand;
mod simple_graph;
mod singlet_fn;
mod step;
mod tile_id_builder;
mod tiles;
mod trial_result;
mod triplet_fn;
mod triplet_i;
mod triplet_l;
mod world;
mod world_builder;

use crate::genome::Genome;
use crate::position::Position;
use crate::rand::Rng;
use crate::world_builder::WorldBuilder;
use anyhow::{Context, Result};
use config::File;
use serde::Deserialize;

#[derive(Deserialize)]
struct RandomPlantsConfig {
    total: usize,
}

#[derive(Deserialize)]
struct StaticPlantsConfig {
    position: Position,

    #[serde(flatten)]
    genome: Genome,
}

#[derive(Deserialize)]
struct Config {
    x_size: usize,
    y_size: usize,
    snapshot_interval: usize,
    max_steps: usize,
    rng_seed: u64,
    random_plants: RandomPlantsConfig,
    static_plants: Vec<StaticPlantsConfig>,
}

fn main() -> Result<()> {
    let config = config::Config::builder()
        .add_source(File::with_name("config"))
        .build()?;
    let config: Config = config
        .try_deserialize()
        .context("Failed to deserialize config file")?;

    let x_size = config.x_size;
    let y_size = config.y_size;
    let mut world = WorldBuilder::new(x_size, y_size);
    world.seed_rate(0.1).mutation_rate(0.1);

    for plant_config in &config.static_plants {
        println!("Adding static plant at {:?}", plant_config.position);
        world.add_plant(plant_config.genome, plant_config.position);
    }

    let mut rng = Rng::from_seed(config.rng_seed);
    let num_plants = config.random_plants.total;
    for _ in 0..num_plants {
        let genome = Genome::random(&mut rng);
        let position = Position::new(rng.uniform(x_size), rng.uniform(y_size));
        println!("Adding random plant at {:?}", position);
        world.add_plant(genome, position);
    }
    let max_steps = config.max_steps;
    let snapshot_interval = config.snapshot_interval;
    world.build().run(&mut rng, max_steps, snapshot_interval);
    Ok(())
}
