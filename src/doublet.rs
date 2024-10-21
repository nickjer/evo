use crate::square_grid::SquareGrid;
use crate::step::Step::*;
use crate::tiles::TileId;

#[derive(Debug, Copy, Clone)]
pub struct Doublet(TileId);

impl Doublet {
    pub fn from(tile_id: TileId, grid: &SquareGrid) -> [Self; 4] {
        [
            Self(grid.id_from(tile_id, &[Up])),
            Self(grid.id_from(tile_id, &[Right])),
            Self(grid.id_from(tile_id, &[Down])),
            Self(grid.id_from(tile_id, &[Left])),
        ]
    }

    pub fn j(&self) -> TileId {
        self.0
    }
}
