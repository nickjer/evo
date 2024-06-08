use crate::plants::PlantId;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Owner {
    Empty,
    Cell(PlantId),
}

impl Owner {
    pub fn is_empty(&self) -> bool {
        matches!(self, Owner::Empty)
    }
}
