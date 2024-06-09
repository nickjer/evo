mod active_genome;
mod active_plant;
mod blob;
mod doublet;
mod doublet_fn;
mod either;
mod genome;
mod genomes;
mod grid;
mod inactive_genome;
mod inactive_plant;
mod neighbors;
mod organisms;
mod owner;
mod plants;
mod position;
mod rand;
mod simple_graph;
mod singlet_fn;
mod step;
mod tile_id_builder;
mod tiles;
mod triplet_fn;
mod triplet_i;
mod triplet_l;
mod world;
mod world_builder;

use crate::doublet_fn::DoubletFn;
use crate::genome::Genome;
use crate::position::Position;
use crate::rand::Rng;
use crate::singlet_fn::SingletFn;
use crate::triplet_fn::TripletFn;
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
    singlet: SingletFn,
    doublet: DoubletFn,
    triplet_l: TripletFn,
    triplet_i: TripletFn,
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
        world.add_plant(
            Genome::new(
                plant_config.singlet,
                plant_config.doublet,
                plant_config.triplet_l,
                plant_config.triplet_i,
            ),
            plant_config.position,
        );
    }

    let mut rng = Rng::from_seed(config.rng_seed);
    let num_plants = config.random_plants.total;
    for _ in 0..num_plants {
        let singlet_fn = SingletFn::from_fn(|| rng.norm() * 2.0);
        let doublet_fn = DoubletFn::from_fn(|| rng.norm() * 2.0);
        let triplet_l_fn = TripletFn::from_fn(|| rng.norm() * 2.0);
        let triplet_i_fn = TripletFn::from_fn(|| rng.norm() * 2.0);

        let genome = Genome::new(singlet_fn, doublet_fn, triplet_l_fn, triplet_i_fn);
        let position = Position::new(rng.uniform(x_size), rng.uniform(y_size));
        world.add_plant(genome, position);
    }
    let max_steps = config.max_steps;
    let snapshot_interval = config.snapshot_interval;
    world.build().run(&mut rng, max_steps, snapshot_interval);
    Ok(())
}
