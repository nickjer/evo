use crate::blob::Blob;
use crate::doublet::Doublet;
use crate::entity::Entity;
use crate::neighbors::Neighbors;
use crate::position::Position;
use crate::tile_id_builder::TileIdBuilder;
use crate::tiles::TileId;
use crate::tiles::Tiles;
use crate::triplet_i::TripletI;
use crate::triplet_l::TripletL;
use getset::CopyGetters;

#[derive(Debug, Clone, CopyGetters, Default)]
pub struct Grid {
    col_size: usize,
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
    pub fn new(x_size: usize, y_size: usize) -> Self {
        let mut entities = Tiles::default();
        let mut neighbors = Tiles::default();
        let mut doublets = Tiles::default();
        let mut triplets_l = Tiles::default();
        let mut triplets_i = Tiles::default();
        let mut blobs = Tiles::default();
        let mut nonces = Tiles::default();
        for x in 0..x_size {
            for y in 0..y_size {
                let position = Position::new(x, y);
                let tile_id_builder = TileIdBuilder::new(position, x_size, y_size);

                entities.push(Entity::Empty);
                neighbors.push(Neighbors::build(tile_id_builder.clone()));
                doublets.push(Doublet::build(tile_id_builder.clone()));
                triplets_l.push(TripletL::build(tile_id_builder.clone()));
                triplets_i.push(TripletI::build(tile_id_builder.clone()));
                blobs.push(Blob::build(tile_id_builder.clone()));
                nonces.push(0);
            }
        }

        let col_size = y_size;
        let size = x_size * y_size;
        Grid {
            col_size,
            size,
            entities,
            neighbors,
            doublets,
            triplets_l,
            triplets_i,
            blobs,
            nonces,
        }
    }

    pub fn x_size(&self) -> usize {
        self.size / self.col_size
    }

    pub fn y_size(&self) -> usize {
        self.col_size
    }

    pub fn columns(&self) -> impl Iterator<Item = &[Entity]> {
        self.entities.chunks(self.col_size)
    }

    pub fn is_empty(&self, tile_id: TileId) -> bool {
        self.entities[tile_id].is_empty()
    }

    pub fn empty_tiles(&self) -> Vec<TileId> {
        (&self.entities)
            .into_iter()
            .enumerate()
            .filter(|(_, entity)| entity.is_empty())
            .map(|(tile_id, _)| TileId::from(tile_id))
            .collect()
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
