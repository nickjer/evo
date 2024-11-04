use crate::cell_kind::CellKind;
use crate::plants::PlantId;
use derive_more::{IsVariant, Unwrap};

#[derive(Debug, Copy, Clone, PartialEq, IsVariant, Unwrap)]
pub enum Entity {
    Empty,
    Cell(PlantId, CellKind),
}

#[derive(Debug, Copy, Clone)]
pub enum GreedyEntity {
    Empty,
    MyCell(CellKind),
    OtherCell(CellKind),
}

impl Entity {
    pub fn into_greedy(self, plant_id: PlantId) -> GreedyEntity {
        match self {
            Entity::Empty => GreedyEntity::Empty,
            Entity::Cell(cell_id, cell_kind) if cell_id == plant_id => {
                GreedyEntity::MyCell(cell_kind)
            }
            Entity::Cell(_, cell_kind) => GreedyEntity::OtherCell(cell_kind),
        }
    }
}
