use crate::entity::Entity;
use crate::genome::Genome;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::rand::Rng;
use crate::tiles::TileId;
use approx::ulps_eq;
use getset::{CopyGetters, Getters};
use nohash::IntSet;
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
        available_tiles: IntSet<TileId>,
        plant_id: PlantId,
        points: usize,
        rng: &mut Rng,
    ) -> Option<TileId> {
        if points == 0 {
            return None;
        }

        let mut count_dupes: usize = 1;
        available_tiles
            .into_iter()
            .filter_map(|tile_id| match grid.entity(tile_id) {
                Entity::Empty => Some((tile_id, self.score(plant_id, grid, tile_id))),
                Entity::Cell(tile_plant_id) if tile_plant_id == plant_id => None,
                Entity::Cell(_) => {
                    if points > 1 {
                        Some((tile_id, self.score(plant_id, grid, tile_id)))
                    } else {
                        None
                    }
                }
            })
            .reduce(|(max_tile_id, max_score), (tile_id, score)| {
                if ulps_eq!(max_score, score, epsilon = 1.0e-6, max_ulps = 10) {
                    count_dupes += 1;
                    let rate = 1.0 / count_dupes as f32;
                    if rng.sample() < rate {
                        (tile_id, score)
                    } else {
                        (max_tile_id, max_score)
                    }
                } else if score > max_score {
                    count_dupes = 0;
                    (tile_id, score)
                } else {
                    (max_tile_id, max_score)
                }
            })
            .map(|(tile_id, _)| tile_id)
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
