use crate::genome::{Genome, GenomeKind};
use crate::genomes::GenomeId;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::rand::Rng;
use crate::tiles::TileId;
use ahash::AHashMap;
use getset::{CopyGetters, Getters};
use serde::Serialize;
use std::cell::RefCell;

#[derive(Debug, Clone, CopyGetters, Getters, Serialize)]
pub struct ActiveGenome {
    id: GenomeId,
    #[serde(flatten)]
    #[getset(get = "pub")]
    genome: GenomeKind,
    num_plants: usize,
    #[getset(get_copy = "pub")]
    max_yield: usize,
    #[getset(get_copy = "pub")]
    created_at: usize,
    #[getset(get_copy = "pub")]
    parent_genome_id: Option<GenomeId>,
    #[serde(skip)]
    score_map: RefCell<AHashMap<(PlantId, TileId, usize), (usize, Option<f32>)>>,
}

impl ActiveGenome {
    pub fn new(
        id: GenomeId,
        genome: GenomeKind,
        parent_genome_id: Option<GenomeId>,
        created_at: usize,
    ) -> Self {
        Self {
            id,
            genome,
            num_plants: 0,
            max_yield: 0,
            created_at,
            parent_genome_id,
            score_map: RefCell::new(AHashMap::new()),
        }
    }

    pub fn mutate(&self, rng: &mut Rng) -> GenomeKind {
        self.genome.mutate(rng)
    }

    pub fn increment(&mut self) -> usize {
        self.num_plants = self.num_plants.checked_add(1).unwrap();
        self.num_plants
    }

    pub fn decrement(&mut self) -> usize {
        self.num_plants = self.num_plants.checked_sub(1).unwrap();
        self.num_plants
    }

    pub fn set_max_yield(&mut self, max_yield: usize) {
        self.max_yield = std::cmp::max(self.max_yield, max_yield);
    }

    pub fn choose_tile(
        &self,
        grid: &Grid,
        available_tiles: &[TileId],
        plant_id: PlantId,
        points: usize,
        rng: &mut Rng,
    ) -> Option<TileId> {
        if points == 0 {
            return None;
        }

        let scores = available_tiles
            .iter()
            .filter_map(|&tile_id| Some(tile_id).zip(self.score(plant_id, grid, tile_id, points)))
            .collect::<Vec<_>>();

        let max_score = scores
            .iter()
            .map(|(_, score)| *score)
            .fold(f32::NEG_INFINITY, f32::max);

        let mut total_cumulative_score = 0.0;
        let cumulative_scores = scores
            .into_iter()
            .map(|(tile_id, score)| {
                let score_weight = self.genome.score_weight();
                total_cumulative_score += (score_weight * (score - max_score)).exp();
                (tile_id, total_cumulative_score)
            })
            .collect::<Vec<_>>();

        let random_score = rng.sample() * total_cumulative_score;
        cumulative_scores
            .into_iter()
            .find(|&(_, cumulative_score)| random_score < cumulative_score)
            .map(|(tile_id, _)| tile_id)
    }

    pub fn score(
        &self,
        plant_id: PlantId,
        grid: &Grid,
        tile_id: TileId,
        points: usize,
    ) -> Option<f32> {
        let nonce = grid.nonce(tile_id);
        let mut score_map = self.score_map.borrow_mut();
        if let Some(&(cached_nonce, cached_score)) = score_map.get(&(plant_id, tile_id, points)) {
            if cached_nonce == nonce {
                return cached_score;
            }
        }

        let score = self.genome.score(plant_id, grid, tile_id, points);
        score_map.insert((plant_id, tile_id, points), (nonce, score));
        score
    }
}
