use crate::square_grid::SquareGrid;
use crate::step::Step::*;
use crate::tiles::TileId;

#[derive(Debug, Copy, Clone)]
pub struct TripletI(TileId, TileId);

impl TripletI {
    pub fn from(tile_id: TileId, grid: &SquareGrid) -> [Self; 4] {
        [
            Self(
                grid.id_from(tile_id, &[Up]),
                grid.id_from(tile_id, &[Up, Up]),
            ),
            Self(
                grid.id_from(tile_id, &[Right]),
                grid.id_from(tile_id, &[Right, Right]),
            ),
            Self(
                grid.id_from(tile_id, &[Down]),
                grid.id_from(tile_id, &[Down, Down]),
            ),
            Self(
                grid.id_from(tile_id, &[Left]),
                grid.id_from(tile_id, &[Left, Left]),
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
