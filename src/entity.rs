use crate::cell_kind::CellKind;
use crate::plants::PlantId;
use derive_more::IsVariant;

#[derive(Debug, Copy, Clone, PartialEq, IsVariant)]
pub enum Entity {
    Empty,
    Cell(PlantId, CellKind),
}

#[derive(Debug, Copy, Clone)]
pub enum GreedyEntity {
    Empty,
    MyCell,
    OtherCell,
}

impl Entity {
    pub fn into_greedy(self, plant_id: PlantId) -> GreedyEntity {
        match self {
            Entity::Empty => GreedyEntity::Empty,
            Entity::Cell(cell_id, _) if cell_id == plant_id => GreedyEntity::MyCell,
            Entity::Cell(..) => GreedyEntity::OtherCell,
        }
    }
}
