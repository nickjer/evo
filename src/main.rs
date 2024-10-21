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
use crate::genomes::{DoubletGenome, TripletGenome};
use crate::position::Position;
use crate::rand::Rng;
use crate::world::World;
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
    world.seed_rate(0.1).mutation_rate(0.1);

    for plant_config in config.static_plants {
        println!("Adding static plant at {:?}", plant_config.position);
        world.add_plant(plant_config.genome, plant_config.position);
    }
    let mut world = world.build();

    add_random_plants(&mut world, &config.random_plants, &mut rng)?;

    let max_steps = config.max_steps;
    let snapshot_interval = config.snapshot_interval;
    world.run(&mut rng, max_steps, snapshot_interval);
    Ok(())
}

fn add_random_plants(
    world: &mut World,
    configs: &[RandomPlantsConfig],
    rng: &mut Rng,
) -> Result<()> {
    let random_genomes = configs
        .iter()
        .flat_map(|config| (0..config.total).map(|_| config.kind.as_str()))
        .map(|kind| match kind {
            "doublet_genome" => Ok(DoubletGenome::random(rng).into()),
            "triplet_genome" => Ok(TripletGenome::random(rng).into()),
            _ => Err(anyhow::anyhow!("Unknown plant kind: {}", kind)),
        })
        .collect::<Result<Vec<GenomeKind>>>()?;

    let mut empty_tiles = world.empty_tiles();
    if random_genomes.len() > empty_tiles.len() {
        anyhow::bail!(
            "Not enough empty tiles to place {} random plants",
            random_genomes.len()
        );
    }

    rng.shuffle(&mut empty_tiles);
    random_genomes
        .into_iter()
        .zip(empty_tiles)
        .for_each(|(genome, tile_id)| {
            let genome_id = world.add_genome(genome);
            world.add_plant(genome_id, tile_id);
        });

    Ok(())
}
