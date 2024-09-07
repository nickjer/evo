use crate::doublet_fn::DoubletFn;
use crate::entity::GreedyEntity;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::rand::Rng;
use crate::singlet_fn::SingletFn;
use crate::tiles::TileId;
use crate::triplet_fn::TripletFn;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Config {
    score_weight: f32,
    singlet: SingletFn,
    doublet: DoubletFn,
    triplet_l: TripletFn,
    triplet_i: TripletFn,
}

#[derive(Debug, Copy, Clone, Getters, Serialize, Deserialize)]
#[serde(from = "Config", into = "Config")]
pub struct TripletGenome {
    #[getset(get = "pub")]
    score_weight: f32,

    #[getset(get = "pub")]
    singlet_fn: SingletFn,

    #[getset(get = "pub")]
    doublet_fn: DoubletFn,

    #[getset(get = "pub")]
    triplet_l_fn: TripletFn,

    #[getset(get = "pub")]
    triplet_i_fn: TripletFn,
}

impl From<Config> for TripletGenome {
    fn from(config: Config) -> Self {
        Self::new(
            config.score_weight,
            config.singlet,
            config.doublet,
            config.triplet_l,
            config.triplet_i,
        )
    }
}

impl From<TripletGenome> for Config {
    fn from(genome: TripletGenome) -> Self {
        Config {
            score_weight: genome.score_weight,
            singlet: genome.singlet_fn,
            doublet: genome.doublet_fn,
            triplet_l: genome.triplet_l_fn,
            triplet_i: genome.triplet_i_fn,
        }
    }
}

impl TripletGenome {
    pub fn random(rng: &mut Rng) -> Self {
        let score_weight = rng.norm() * 2.0;
        let singlet_fn = SingletFn::from_fn(|| rng.norm() * 2.0);
        let doublet_fn = DoubletFn::from_fn(|| rng.norm() * 2.0);
        let triplet_l_fn = TripletFn::from_fn(|| rng.norm() * 2.0);
        let triplet_i_fn = TripletFn::from_fn(|| rng.norm() * 2.0);
        Self::new(
            score_weight,
            singlet_fn,
            doublet_fn,
            triplet_l_fn,
            triplet_i_fn,
        )
    }

    fn rescale(self) -> Self {
        let min = self
            .singlet_fn
            .min()
            .min(self.doublet_fn.min())
            .min(self.triplet_l_fn.min())
            .min(self.triplet_i_fn.min());
        let max = self
            .singlet_fn
            .max()
            .max(self.doublet_fn.max())
            .max(self.triplet_l_fn.max())
            .max(self.triplet_i_fn.max());
        let scale = 1.0 / (max - min);
        Self {
            score_weight: self.score_weight.abs() / scale,
            singlet_fn: self.singlet_fn.translate(-min).scale(scale),
            doublet_fn: self.doublet_fn.translate(-min).scale(scale),
            triplet_l_fn: self.triplet_l_fn.translate(-min).scale(scale),
            triplet_i_fn: self.triplet_i_fn.translate(-min).scale(scale),
        }
    }

    fn new(
        score_weight: f32,
        singlet_fn: SingletFn,
        doublet_fn: DoubletFn,
        triplet_l_fn: TripletFn,
        triplet_i_fn: TripletFn,
    ) -> Self {
        Self {
            score_weight,
            singlet_fn,
            doublet_fn,
            triplet_l_fn,
            triplet_i_fn,
        }
        .rescale()
    }

    pub fn mutate(&self, rng: &mut Rng) -> Self {
        let score_weight = self.score_weight + rng.norm() * 0.1 * self.score_weight;
        let mut mutator = |value| value + rng.norm() * 0.01;
        Self::new(
            score_weight,
            self.singlet_fn.mutate(&mut mutator),
            self.doublet_fn.mutate(&mut mutator),
            self.triplet_l_fn.mutate(&mut mutator),
            self.triplet_i_fn.mutate(&mut mutator),
        )
    }

    pub fn score(
        &self,
        plant_id: PlantId,
        grid: &Grid,
        tile_id: TileId,
        points: usize,
    ) -> Option<f32> {
        let entity_1 = grid.entity(tile_id).into_greedy(plant_id);
        match entity_1 {
            GreedyEntity::Empty => {}
            GreedyEntity::MyCell => return None,
            GreedyEntity::OtherCell => {
                if points <= 1 {
                    return None;
                }
            }
        }

        let mut score = 0.0;
        score += self.singlet_fn().score(entity_1);
        score += grid.doublets(tile_id).iter().fold(0.0, |sum, &doublet| {
            let tile_id_2 = doublet.j();
            let entity_2 = grid.entity(tile_id_2).into_greedy(plant_id);
            sum + self.doublet_fn().score(entity_1, entity_2)
        });
        score += grid.triplets_l(tile_id).iter().fold(0.0, |sum, &triplet| {
            let tile_id_2 = triplet.j();
            let tile_id_3 = triplet.k();
            let entity_2 = grid.entity(tile_id_2).into_greedy(plant_id);
            let entity_3 = grid.entity(tile_id_3).into_greedy(plant_id);
            sum + self.triplet_l_fn().score(entity_1, entity_2, entity_3)
        });
        score += grid.triplets_i(tile_id).iter().fold(0.0, |sum, &triplet| {
            let tile_id_2 = triplet.j();
            let tile_id_3 = triplet.k();
            let entity_2 = grid.entity(tile_id_2).into_greedy(plant_id);
            let entity_3 = grid.entity(tile_id_3).into_greedy(plant_id);
            sum + self.triplet_i_fn().score(entity_1, entity_2, entity_3)
        });
        Some(score)
    }
}
