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

    pub fn translate(self, amount: f32) -> Self {
        Self {
            doublet_ee: self.doublet_ee + amount,
            doublet_es: self.doublet_es + amount,
            doublet_eo: self.doublet_eo + amount,
            doublet_oe: self.doublet_oe + amount,
            doublet_os: self.doublet_os + amount,
            doublet_oo: self.doublet_oo + amount,
        }
    }

    pub fn scale(self, amount: f32) -> Self {
        Self {
            doublet_ee: self.doublet_ee * amount,
            doublet_es: self.doublet_es * amount,
            doublet_eo: self.doublet_eo * amount,
            doublet_oe: self.doublet_oe * amount,
            doublet_os: self.doublet_os * amount,
            doublet_oo: self.doublet_oo * amount,
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

    pub fn min(&self) -> f32 {
        self.doublet_ee
            .min(self.doublet_es)
            .min(self.doublet_eo)
            .min(self.doublet_oe)
            .min(self.doublet_os)
            .min(self.doublet_oo)
    }

    pub fn max(&self) -> f32 {
        self.doublet_ee
            .max(self.doublet_es)
            .max(self.doublet_eo)
            .max(self.doublet_oe)
            .max(self.doublet_os)
            .max(self.doublet_oo)
    }

    pub fn score(&self, entity_i: GreedyEntity, entity_j: GreedyEntity) -> f32 {
        match (entity_i, entity_j) {
            (GreedyEntity::Empty, GreedyEntity::Empty) => self.doublet_ee,
            (GreedyEntity::Empty, GreedyEntity::MyCell(_)) => self.doublet_es,
            (GreedyEntity::Empty, GreedyEntity::OtherCell(_)) => self.doublet_eo,
            (GreedyEntity::OtherCell(_), GreedyEntity::Empty) => self.doublet_oe,
            (GreedyEntity::OtherCell(_), GreedyEntity::MyCell(_)) => self.doublet_os,
            (GreedyEntity::OtherCell(_), GreedyEntity::OtherCell(_)) => self.doublet_oo,
            (GreedyEntity::MyCell(_), _) => panic!("Invalid entity pair"),
        }
    }
}
