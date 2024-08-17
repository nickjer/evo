use crate::blob::Blob;
use crate::doublet::Doublet;
use crate::entity::Entity;
use crate::neighbors::Neighbors;
use crate::tiles::TileId;
use crate::tiles::Tiles;
use crate::triplet_i::TripletI;
use crate::triplet_l::TripletL;
use getset::CopyGetters;

#[derive(Debug, Clone, CopyGetters, Default)]
pub struct Grid {
    #[get_copy = "pub"]
    size: usize,
    entities: Tiles<Entity>,
    neighbors: Tiles<Neighbors>,
    doublets: Tiles<[Doublet; 4]>,
    triplets_l: Tiles<[TripletL; 8]>,
    triplets_i: Tiles<[TripletI; 4]>,
    blobs: Tiles<Blob>,
    nonces: Tiles<usize>,
}

impl Grid {
    pub fn entity_chunks(&self, chunk_size: usize) -> Vec<&[Entity]> {
        self.entities.chunks(chunk_size).collect()
    }

    pub fn push(
        &mut self,
        entity: Entity,
        neighbors: Neighbors,
        doublets: [Doublet; 4],
        triplets_l: [TripletL; 8],
        triplets_i: [TripletI; 4],
        blob: Blob,
    ) {
        self.size += 1;
        self.entities.push(entity);
        self.neighbors.push(neighbors);
        self.doublets.push(doublets);
        self.triplets_l.push(triplets_l);
        self.triplets_i.push(triplets_i);
        self.blobs.push(blob);
        self.nonces.push(1);
    }

    pub fn is_empty(&self, tile_id: TileId) -> bool {
        self.entities[tile_id].is_empty()
    }

    pub fn nonce(&self, tile_id: TileId) -> usize {
        self.nonces[tile_id]
    }

    fn touch(&mut self, tile_id: TileId) {
        self.nonces[tile_id] = self.nonces[tile_id].checked_add(1).unwrap();
        self.blobs[tile_id].tile_ids().iter().for_each(|&tile_id| {
            self.nonces[tile_id] = self.nonces[tile_id].checked_add(1).unwrap();
        })
    }

    pub fn entity(&self, tile_id: TileId) -> Entity {
        self.entities[tile_id]
    }

    pub fn replace_entity(&mut self, tile_id: TileId, new_entity: Entity) -> Entity {
        let old_entity = std::mem::replace(&mut self.entities[tile_id], new_entity);
        if old_entity != new_entity {
            self.touch(tile_id)
        }
        old_entity
    }

    pub fn neighbors(&self, tile_id: TileId) -> &Neighbors {
        &self.neighbors[tile_id]
    }

    pub fn doublets(&self, tile_id: TileId) -> &[Doublet; 4] {
        &self.doublets[tile_id]
    }

    pub fn triplets_l(&self, tile_id: TileId) -> &[TripletL; 8] {
        &self.triplets_l[tile_id]
    }

    pub fn triplets_i(&self, tile_id: TileId) -> &[TripletI; 4] {
        &self.triplets_i[tile_id]
    }
}
