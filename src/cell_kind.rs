#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellKind {
    Branch,
}

impl CellKind {
    pub fn cost_to_grow(&self) -> usize {
        match self {
            CellKind::Branch => 1,
        }
    }

    pub fn cost_to_kill(&self) -> usize {
        match self {
            CellKind::Branch => 1,
        }
    }

    pub fn yield_per_empty_tile(&self) -> usize {
        match self {
            CellKind::Branch => 1,
        }
    }
}
