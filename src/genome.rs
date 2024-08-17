use crate::doublet_fn::DoubletFn;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::singlet_fn::SingletFn;
use crate::tiles::TileId;
use crate::triplet_fn::TripletFn;
use derive_more::Constructor;
use getset::Getters;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Constructor, Getters, Serialize)]
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
    pub fn mutate(&self, mut mutator: impl FnMut(f32) -> f32) -> Self {
        Self::new(
            self.singlet_fn.mutate(&mut mutator),
            self.doublet_fn.mutate(&mut mutator),
            self.triplet_l_fn.mutate(&mut mutator),
            self.triplet_i_fn.mutate(&mut mutator),
        )
    }

    pub fn score(&self, plant_id: PlantId, grid: &Grid, tile_id: TileId) -> f32 {
        let entity_id_1 = grid.entity(tile_id);
        let mut score = 0.0;
        score += self.singlet_fn().score(entity_id_1);
        score += grid.doublets(tile_id).iter().fold(0.0, |sum, &doublet| {
            let tile_id_2 = doublet.j();
            sum + self
                .doublet_fn()
                .score(plant_id, entity_id_1, grid.entity(tile_id_2))
        });
        score += grid.triplets_l(tile_id).iter().fold(0.0, |sum, &triplet| {
            let tile_id_2 = triplet.j();
            let tile_id_3 = triplet.k();
            sum + self.triplet_l_fn().score(
                plant_id,
                entity_id_1,
                grid.entity(tile_id_2),
                grid.entity(tile_id_3),
            )
        });
        score += grid.triplets_i(tile_id).iter().fold(0.0, |sum, &triplet| {
            let tile_id_2 = triplet.j();
            let tile_id_3 = triplet.k();
            sum + self.triplet_i_fn().score(
                plant_id,
                entity_id_1,
                grid.entity(tile_id_2),
                grid.entity(tile_id_3),
            )
        });
        score
    }
}
