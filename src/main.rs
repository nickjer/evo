mod active_genome;
mod active_plant;
mod blob;
mod cell_kind;
mod doublet;
mod doublet_fn;
mod either;
mod entity;
mod genome;
mod genomes;
mod grid;
mod inactive_genome;
mod inactive_plant;
mod organisms;
mod plants;
mod position;
mod rand;
mod simple_graph;
mod singlet_fn;
mod square_grid;
mod step;
mod tiles;
mod trial_result;
mod triplet_fn;
mod triplet_i;
mod triplet_l;
mod world;
mod world_builder;

use crate::genome::GenomeKind;
use crate::position::Position;
use crate::rand::Rng;
use crate::world_builder::WorldBuilder;
use anyhow::{Context, Result};
use config::File;
use serde::Deserialize;

#[derive(Deserialize)]
struct RandomPlantsConfig {
    kind: String,
    total: usize,
}

#[derive(Deserialize)]
struct StaticPlantsConfig {
    position: Position,

    #[serde(flatten)]
    genome: GenomeKind,
}

#[derive(Deserialize)]
struct Config {
    x_size: usize,
    y_size: usize,
    snapshot_interval: usize,
    max_steps: usize,
    rng_seed: u64,
    take_top: usize,
    seed_rate: f32,
    mutation_rate: f32,
    random_plants: Vec<RandomPlantsConfig>,
    static_plants: Vec<StaticPlantsConfig>,
}

fn main() -> Result<()> {
    let config = config::Config::builder()
        .add_source(File::with_name("config"))
        .build()?;
    let config: Config = config
        .try_deserialize()
        .context("Failed to deserialize config file")?;
    let mut rng = Rng::from_seed(config.rng_seed);

    let x_size = config.x_size;
    let y_size = config.y_size;
    let mut world = WorldBuilder::new(x_size, y_size);
    world
        .take_top(config.take_top)
        .seed_rate(config.seed_rate)
        .mutation_rate(config.mutation_rate);

    for plant_config in config.static_plants {
        println!("Adding static plant at {:?}", plant_config.position);
        world.add_plant(plant_config.genome, plant_config.position)?;
    }
    for plant_config in config.random_plants {
        let total = plant_config.total;
        let kind = &plant_config.kind;
        println!("Adding {total} {kind} random plants");
        world.add_random_plants(kind, total, &mut rng)?;
    }

    let mut world = world.build();
    let max_steps = config.max_steps;
    let snapshot_interval = config.snapshot_interval;
    world.run(&mut rng, max_steps, snapshot_interval);
    Ok(())
}
