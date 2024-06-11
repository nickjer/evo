use crate::genome::Genome;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::tiles::TileId;
use getset::{CopyGetters, Getters};
use serde::Serialize;
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
    score_map: HashMap<TileId, (usize, f32)>,
}

impl ActiveGenome {
    pub fn new(genome: Genome) -> Self {
        Self {
            genome,
            num_plants: 0,
            max_yield: 0,
            score_map: HashMap::default(),
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

    pub fn score(&mut self, plant_id: PlantId, grid: &Grid, tile_id: TileId) -> f32 {
        let nonce = grid.nonce(tile_id);
        if let Some(&(cached_nonce, cached_score)) = self.score_map.get(&tile_id) {
            if cached_nonce == nonce {
                return cached_score;
            }
        }

        let owner_id_1 = grid.owner(tile_id);
        let mut score = 0.0;
        score += self.genome.singlet_fn().score(owner_id_1);
        score += grid.doublets(tile_id).iter().fold(0.0, |sum, &doublet| {
            let tile_id_2 = doublet.j();
            sum + self
                .genome
                .doublet_fn()
                .score(plant_id, owner_id_1, grid.owner(tile_id_2))
        });
        score += grid.triplets_l(tile_id).iter().fold(0.0, |sum, &triplet| {
            let tile_id_2 = triplet.j();
            let tile_id_3 = triplet.k();
            sum + self.genome.triplet_l_fn().score(
                plant_id,
                owner_id_1,
                grid.owner(tile_id_2),
                grid.owner(tile_id_3),
            )
        });
        score += grid.triplets_i(tile_id).iter().fold(0.0, |sum, &triplet| {
            let tile_id_2 = triplet.j();
            let tile_id_3 = triplet.k();
            sum + self.genome.triplet_i_fn().score(
                plant_id,
                owner_id_1,
                grid.owner(tile_id_2),
                grid.owner(tile_id_3),
            )
        });
        self.score_map.insert(tile_id, (nonce, score));
        score
    }
}
