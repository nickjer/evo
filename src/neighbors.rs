use crate::tile_id_builder::TileIdBuilder;
use crate::tiles::TileId;
use derive_more::IntoIterator;
use nohash::IntSet;

pub const NEIGHBOR_COUNT: usize = 4;

#[derive(Debug, Copy, Clone, IntoIterator)]
pub struct Neighbors([TileId; NEIGHBOR_COUNT]);

impl Neighbors {
    pub fn build(tile_id_builder: TileIdBuilder) -> Self {
        Self([
            tile_id_builder.clone().left(1).build(),
            tile_id_builder.clone().right(1).build(),
            tile_id_builder.clone().up(1).build(),
            tile_id_builder.clone().down(1).build(),
        ])
    }

    pub fn as_set(&self) -> IntSet<TileId> {
        self.0.iter().copied().collect()
    }
}
