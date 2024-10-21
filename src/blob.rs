use crate::square_grid::SquareGrid;
use crate::step::Step::*;
use crate::tiles::TileId;
use getset::Getters;

#[derive(Debug, Copy, Clone, Getters)]
pub struct Blob {
    #[get = "pub"]
    tile_ids: [TileId; 12],
}

impl Blob {
    pub fn from(tile_id: TileId, grid: &SquareGrid) -> Self {
        let tile_ids = [
            grid.id_from(tile_id, &[Up]),
            grid.id_from(tile_id, &[Right]),
            grid.id_from(tile_id, &[Down]),
            grid.id_from(tile_id, &[Left]),
            grid.id_from(tile_id, &[Up, Right]),
            grid.id_from(tile_id, &[Right, Down]),
            grid.id_from(tile_id, &[Down, Left]),
            grid.id_from(tile_id, &[Left, Up]),
            grid.id_from(tile_id, &[Up, Up]),
            grid.id_from(tile_id, &[Right, Right]),
            grid.id_from(tile_id, &[Down, Down]),
            grid.id_from(tile_id, &[Left, Left]),
        ];
        Self { tile_ids }
    }
}
