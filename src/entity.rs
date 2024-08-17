use crate::plants::PlantId;
use derive_more::IsVariant;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, IsVariant)]
pub enum Entity {
    Empty,
    Cell(PlantId),
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
            Entity::Cell(cell_id) if cell_id == plant_id => GreedyEntity::MyCell,
            Entity::Cell(_) => GreedyEntity::OtherCell,
        }
    }
}
