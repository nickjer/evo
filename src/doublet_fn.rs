use crate::entity::GreedyEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct DoubletFn {
    #[serde(rename = "ee")]
    doublet_ee: f32,
    #[serde(rename = "es")]
    doublet_es: f32,
    #[serde(rename = "eo")]
    doublet_eo: f32,
    #[serde(rename = "oe")]
    doublet_oe: f32,
    #[serde(rename = "os")]
    doublet_os: f32,
    #[serde(rename = "oo")]
    doublet_oo: f32,
}

impl DoubletFn {
    pub fn from_fn(mut f: impl FnMut() -> f32) -> Self {
        Self {
            doublet_ee: f(),
            doublet_es: f(),
            doublet_eo: f(),
            doublet_oe: f(),
            doublet_os: f(),
            doublet_oo: f(),
        }
    }

    pub fn mutate(&self, mut mutator: impl FnMut(f32) -> f32) -> Self {
        Self {
            doublet_ee: mutator(self.doublet_ee),
            doublet_es: mutator(self.doublet_es),
            doublet_eo: mutator(self.doublet_eo),
            doublet_oe: mutator(self.doublet_oe),
            doublet_os: mutator(self.doublet_os),
            doublet_oo: mutator(self.doublet_oo),
        }
    }

    pub fn score(&self, entity_i: GreedyEntity, entity_j: GreedyEntity) -> f32 {
        match (entity_i, entity_j) {
            (GreedyEntity::Empty, GreedyEntity::Empty) => self.doublet_ee,
            (GreedyEntity::Empty, GreedyEntity::MyCell) => self.doublet_es,
            (GreedyEntity::Empty, GreedyEntity::OtherCell) => self.doublet_eo,
            (GreedyEntity::OtherCell, GreedyEntity::Empty) => self.doublet_oe,
            (GreedyEntity::OtherCell, GreedyEntity::MyCell) => self.doublet_os,
            (GreedyEntity::OtherCell, GreedyEntity::OtherCell) => self.doublet_oo,
            (GreedyEntity::MyCell, _) => panic!("Invalid entity pair"),
        }
    }
}
