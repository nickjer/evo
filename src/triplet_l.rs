use crate::tile_id_builder::TileIdBuilder;
use crate::tiles::TileId;

#[derive(Debug, Copy, Clone)]
pub struct TripletL(TileId, TileId);

impl TripletL {
    pub fn build(tile_id_builder: TileIdBuilder) -> [Self; 8] {
        [
            Self(
                tile_id_builder.clone().up(1).build(),
                tile_id_builder.clone().up(1).right(1).build(),
            ),
            Self(
                tile_id_builder.clone().up(1).build(),
                tile_id_builder.clone().up(1).left(1).build(),
            ),
            Self(
                tile_id_builder.clone().right(1).build(),
                tile_id_builder.clone().right(1).down(1).build(),
            ),
            Self(
                tile_id_builder.clone().right(1).build(),
                tile_id_builder.clone().right(1).up(1).build(),
            ),
            Self(
                tile_id_builder.clone().down(1).build(),
                tile_id_builder.clone().down(1).left(1).build(),
            ),
            Self(
                tile_id_builder.clone().down(1).build(),
                tile_id_builder.clone().down(1).right(1).build(),
            ),
            Self(
                tile_id_builder.clone().left(1).build(),
                tile_id_builder.clone().left(1).up(1).build(),
            ),
            Self(
                tile_id_builder.clone().left(1).build(),
                tile_id_builder.clone().left(1).down(1).build(),
            ),
        ]
    }

    pub fn j(&self) -> TileId {
        self.0
    }

    pub fn k(&self) -> TileId {
        self.1
    }
}
