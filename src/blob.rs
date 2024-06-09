use crate::tile_id_builder::TileIdBuilder;
use crate::tiles::TileId;
use getset::Getters;

#[derive(Debug, Copy, Clone, Getters)]
pub struct Blob {
    #[get = "pub"]
    tile_ids: [TileId; 12],
}

impl Blob {
    pub fn build(tile_id_builder: TileIdBuilder) -> Self {
        let tile_ids = [
            tile_id_builder.clone().up(1).build(),
            tile_id_builder.clone().right(1).build(),
            tile_id_builder.clone().down(1).build(),
            tile_id_builder.clone().left(1).build(),
            tile_id_builder.clone().up(1).right(1).build(),
            tile_id_builder.clone().right(1).down(1).build(),
            tile_id_builder.clone().down(1).left(1).build(),
            tile_id_builder.clone().left(1).up(1).build(),
            tile_id_builder.clone().up(2).build(),
            tile_id_builder.clone().right(2).build(),
            tile_id_builder.clone().down(2).build(),
            tile_id_builder.clone().left(2).build(),
        ];
        Self { tile_ids }
    }
}
