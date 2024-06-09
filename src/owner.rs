use crate::plants::PlantId;
use derive_more::IsVariant;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, IsVariant)]
pub enum Owner {
    Empty,
    Cell(PlantId),
}
