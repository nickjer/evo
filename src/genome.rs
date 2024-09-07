use crate::genomes::TripletGenome;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::rand::Rng;
use crate::tiles::TileId;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GenomeKind {
    TripletGenome,
}

#[enum_dispatch(GenomeKind)]
pub trait Genome {
    fn mutate(&self, rng: &mut Rng) -> GenomeKind;

    fn score(&self, plant_id: PlantId, grid: &Grid, tile_id: TileId, points: usize) -> Option<f32>;

    fn score_weight(&self) -> f32;
}
