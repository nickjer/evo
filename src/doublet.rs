use crate::tile_id_builder::TileIdBuilder;
use crate::tiles::TileId;

#[derive(Debug, Copy, Clone)]
pub struct Doublet(TileId);

impl Doublet {
    pub fn build(tile_id_builder: TileIdBuilder) -> [Self; 4] {
        [
            Self(tile_id_builder.clone().up(1).build()),
            Self(tile_id_builder.clone().right(1).build()),
            Self(tile_id_builder.clone().down(1).build()),
            Self(tile_id_builder.clone().left(1).build()),
        ]
    }

    pub fn j(&self) -> TileId {
        self.0
    }
}
