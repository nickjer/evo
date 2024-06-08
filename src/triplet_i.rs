use crate::tile_id_builder::TileIdBuilder;
use crate::tile_list::TileId;

#[derive(Debug, Copy, Clone)]
pub struct TripletI(TileId, TileId);

impl TripletI {
    pub fn build(tile_id_builder: TileIdBuilder) -> [Self; 4] {
        [
            Self(
                tile_id_builder.clone().up(1).build(),
                tile_id_builder.clone().up(1).up(1).build(),
            ),
            Self(
                tile_id_builder.clone().right(1).build(),
                tile_id_builder.clone().right(1).right(1).build(),
            ),
            Self(
                tile_id_builder.clone().down(1).build(),
                tile_id_builder.clone().down(1).down(1).build(),
            ),
            Self(
                tile_id_builder.clone().left(1).build(),
                tile_id_builder.clone().left(1).left(1).build(),
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
