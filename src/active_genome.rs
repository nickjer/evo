use crate::genome::Genome;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::tiles::TileId;
use getset::{CopyGetters, Getters};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone, CopyGetters, Getters, Serialize)]
pub struct ActiveGenome {
    #[serde(flatten)]
    #[getset(get = "pub")]
    genome: Genome,
    num_plants: usize,
    #[getset(get_copy = "pub")]
    max_yield: usize,
    #[serde(skip)]
    score_map: RefCell<HashMap<TileId, (usize, f32)>>,
}

impl ActiveGenome {
    pub fn new(genome: Genome) -> Self {
        Self {
            genome,
            num_plants: 0,
            max_yield: 0,
            score_map: RefCell::new(HashMap::default()),
        }
    }

    pub fn mutate(&self, mut mutator: impl FnMut(f32) -> f32) -> Genome {
        self.genome.mutate(&mut mutator)
    }

    pub fn increment(&mut self) -> usize {
        self.num_plants += 1;
        self.num_plants
    }

    pub fn decrement(&mut self) -> usize {
        self.num_plants -= 1;
        self.num_plants
    }

    pub fn set_max_yield(&mut self, max_yield: usize) {
        self.max_yield = std::cmp::max(self.max_yield, max_yield);
    }

    pub fn score(&self, plant_id: PlantId, grid: &Grid, tile_id: TileId) -> f32 {
        let nonce = grid.nonce(tile_id);
        let mut score_map = self.score_map.borrow_mut();
        if let Some(&(cached_nonce, cached_score)) = score_map.get(&tile_id) {
            if cached_nonce == nonce {
                return cached_score;
            }
        }

        let score = self.genome.score(plant_id, grid, tile_id);
        score_map.insert(tile_id, (nonce, score));
        score
    }
}
