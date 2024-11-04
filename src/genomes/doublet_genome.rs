use crate::doublet_fn::DoubletFn;
use crate::entity::GreedyEntity;
use crate::genome::{Genome, GenomeKind};
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::rand::Rng;
use crate::singlet_fn::SingletFn;
use crate::tiles::TileId;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Config {
    score_weight: f32,
    singlet: SingletFn,
    doublet: DoubletFn,
}

#[derive(Debug, Copy, Clone, Getters, Serialize, Deserialize)]
#[serde(from = "Config", into = "Config")]
pub struct DoubletGenome {
    score_weight: f32,

    #[getset(get = "pub")]
    singlet_fn: SingletFn,

    #[getset(get = "pub")]
    doublet_fn: DoubletFn,
}

impl From<Config> for DoubletGenome {
    fn from(config: Config) -> Self {
        Self::new(config.score_weight, config.singlet, config.doublet)
    }
}

impl From<DoubletGenome> for Config {
    fn from(genome: DoubletGenome) -> Self {
        Config {
            score_weight: genome.score_weight,
            singlet: genome.singlet_fn,
            doublet: genome.doublet_fn,
        }
    }
}

impl DoubletGenome {
    pub fn random(rng: &mut Rng) -> Self {
        let score_weight = rng.norm() * 2.0;
        let singlet_fn = SingletFn::from_fn(|| rng.norm() * 2.0);
        let doublet_fn = DoubletFn::from_fn(|| rng.norm() * 2.0);
        Self::new(score_weight, singlet_fn, doublet_fn)
    }

    fn rescale(self) -> Self {
        let min = self.singlet_fn.min().min(self.doublet_fn.min());
        let max = self.singlet_fn.max().max(self.doublet_fn.max());
        let scale = 1.0 / (max - min);
        Self {
            score_weight: self.score_weight.abs() / scale,
            singlet_fn: self.singlet_fn.translate(-min).scale(scale),
            doublet_fn: self.doublet_fn.translate(-min).scale(scale),
        }
    }

    fn new(score_weight: f32, singlet_fn: SingletFn, doublet_fn: DoubletFn) -> Self {
        Self {
            score_weight,
            singlet_fn,
            doublet_fn,
        }
        .rescale()
    }
}

impl Genome for DoubletGenome {
    fn score_weight(&self) -> f32 {
        self.score_weight
    }

    fn mutate(&self, rng: &mut Rng) -> GenomeKind {
        let score_weight = self.score_weight + rng.norm() * 0.1 * self.score_weight;
        let mut mutator = |value| value + rng.norm() * 0.01;
        Self::new(
            score_weight,
            self.singlet_fn.mutate(&mut mutator),
            self.doublet_fn.mutate(&mut mutator),
        )
        .into()
    }

    fn score(&self, plant_id: PlantId, tile_id: TileId, points: usize, grid: &Grid) -> Option<f32> {
        let entity_1 = grid.entity(tile_id).into_greedy(plant_id);
        match entity_1 {
            GreedyEntity::Empty => {}
            GreedyEntity::MyCell(_) => return None,
            GreedyEntity::OtherCell(_) => {
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
        Some(score)
    }
}
