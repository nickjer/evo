use crate::doublet_fn::DoubletFn;
use crate::plants::PlantId;
use crate::singlet_fn::SingletFn;
use crate::tile_list::TileId;
use crate::tiles::Tiles;
use crate::triplet_fn::TripletFn;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Genome {
    singlet_fn: SingletFn,
    doublet_fn: DoubletFn,
    triplet_l_fn: TripletFn,
    triplet_i_fn: TripletFn,
    score_map: HashMap<TileId, (usize, f32)>,
}

impl Genome {
    pub fn new(
        singlet_fn: SingletFn,
        doublet_fn: DoubletFn,
        triplet_l_fn: TripletFn,
        triplet_i_fn: TripletFn,
    ) -> Self {
        Self {
            singlet_fn,
            doublet_fn,
            triplet_l_fn,
            triplet_i_fn,
            score_map: HashMap::default(),
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

    pub fn score(&mut self, plant_id: PlantId, tiles: &Tiles, tile_id: TileId) -> f32 {
        let nonce = tiles.nonce(tile_id);
        if let Some(&(cached_nonce, cached_score)) = self.score_map.get(&tile_id) {
            if cached_nonce == nonce {
                return cached_score;
            }
        }

        let owner_id_1 = tiles.owner(tile_id);
        let mut score = 0.0;
        score += self.singlet_fn.score(owner_id_1);
        score += tiles.doublets(tile_id).iter().fold(0.0, |sum, &doublet| {
            let tile_id_2 = doublet.j();
            sum + self
                .doublet_fn
                .score(plant_id, owner_id_1, tiles.owner(tile_id_2))
        });
        score += tiles.triplets_l(tile_id).iter().fold(0.0, |sum, &triplet| {
            let tile_id_2 = triplet.j();
            let tile_id_3 = triplet.k();
            sum + self.triplet_l_fn.score(
                plant_id,
                owner_id_1,
                tiles.owner(tile_id_2),
                tiles.owner(tile_id_3),
            )
        });
        score += tiles.triplets_i(tile_id).iter().fold(0.0, |sum, &triplet| {
            let tile_id_2 = triplet.j();
            let tile_id_3 = triplet.k();
            sum + self.triplet_i_fn.score(
                plant_id,
                owner_id_1,
                tiles.owner(tile_id_2),
                tiles.owner(tile_id_3),
            )
        });
        self.score_map.insert(tile_id, (nonce, score));
        score
    }
}
