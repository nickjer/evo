use crate::owner::Owner;
use crate::plants::PlantId;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
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

    pub fn score(&self, plant_s: PlantId, owner_i: Owner, owner_j: Owner) -> f32 {
        match (owner_i, owner_j) {
            (Owner::Empty, Owner::Empty) => self.doublet_ee,
            (Owner::Empty, Owner::Cell(plant)) if plant == plant_s => self.doublet_es,
            (Owner::Empty, Owner::Cell(_)) => self.doublet_eo,
            (Owner::Cell(_), Owner::Empty) => self.doublet_oe,
            (Owner::Cell(_), Owner::Cell(plant)) if plant == plant_s => self.doublet_os,
            (Owner::Cell(_), Owner::Cell(_)) => self.doublet_oo,
        }
    }
}
