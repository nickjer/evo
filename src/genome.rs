use crate::doublet_fn::DoubletFn;
use crate::entity::GreedyEntity;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::singlet_fn::SingletFn;
use crate::tiles::TileId;
use crate::triplet_fn::TripletFn;
use getset::Getters;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Getters, Serialize)]
pub struct Genome {
    #[serde(rename = "singlet")]
    #[getset(get = "pub")]
    singlet_fn: SingletFn,

    #[serde(rename = "doublet")]
    #[getset(get = "pub")]
    doublet_fn: DoubletFn,

    #[serde(rename = "triplet_l")]
    #[getset(get = "pub")]
    triplet_l_fn: TripletFn,

    #[serde(rename = "triplet_i")]
    #[getset(get = "pub")]
    triplet_i_fn: TripletFn,
}

impl Genome {
    pub fn new(
        singlet_fn: SingletFn,
        doublet_fn: DoubletFn,
        triplet_l_fn: TripletFn,
        triplet_i_fn: TripletFn,
    ) -> Self {
        let min = singlet_fn
            .min()
            .min(doublet_fn.min())
            .min(triplet_l_fn.min())
            .min(triplet_i_fn.min());
        let max = singlet_fn
            .max()
            .max(doublet_fn.max())
            .max(triplet_l_fn.max())
            .max(triplet_i_fn.max());
        let scale = 1.0 / (max - min);
        Self {
            singlet_fn: singlet_fn.translate(-min).scale(scale),
            doublet_fn: doublet_fn.translate(-min).scale(scale),
            triplet_l_fn: triplet_l_fn.translate(-min).scale(scale),
            triplet_i_fn: triplet_i_fn.translate(-min).scale(scale),
        }
    }

    pub fn mutate(&self, mut mutator: impl FnMut(f32) -> f32) -> Self {
        Self::new(
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
