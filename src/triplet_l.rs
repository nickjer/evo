use crate::square_grid::SquareGrid;
use crate::step::Step::*;
use crate::tiles::TileId;

#[derive(Debug, Copy, Clone)]
pub struct TripletL(TileId, TileId);

impl TripletL {
    pub fn from(tile_id: TileId, grid: &SquareGrid) -> [Self; 8] {
        [
            Self(
                grid.id_from(tile_id, &[Up]),
                grid.id_from(tile_id, &[Up, Right]),
            ),
            Self(
                grid.id_from(tile_id, &[Up]),
                grid.id_from(tile_id, &[Up, Left]),
            ),
            Self(
                grid.id_from(tile_id, &[Right]),
                grid.id_from(tile_id, &[Right, Down]),
            ),
            Self(
                grid.id_from(tile_id, &[Right]),
                grid.id_from(tile_id, &[Right, Up]),
            ),
            Self(
                grid.id_from(tile_id, &[Down]),
                grid.id_from(tile_id, &[Down, Left]),
            ),
            Self(
                grid.id_from(tile_id, &[Down]),
                grid.id_from(tile_id, &[Down, Right]),
            ),
            Self(
                grid.id_from(tile_id, &[Left]),
                grid.id_from(tile_id, &[Left, Up]),
            ),
            Self(
                grid.id_from(tile_id, &[Left]),
                grid.id_from(tile_id, &[Left, Down]),
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
