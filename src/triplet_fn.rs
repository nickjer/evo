use crate::entity::GreedyEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct TripletFn {
    #[serde(rename = "eee")]
    triplet_eee: f32,
    #[serde(rename = "ees")]
    triplet_ees: f32,
    #[serde(rename = "eeo")]
    triplet_eeo: f32,
    #[serde(rename = "ese")]
    triplet_ese: f32,
    #[serde(rename = "ess")]
    triplet_ess: f32,
    #[serde(rename = "eso")]
    triplet_eso: f32,
    #[serde(rename = "eoe")]
    triplet_eoe: f32,
    #[serde(rename = "eos")]
    triplet_eos: f32,
    #[serde(rename = "eoo")]
    triplet_eoo: f32,
    #[serde(rename = "oee")]
    triplet_oee: f32,
    #[serde(rename = "oes")]
    triplet_oes: f32,
    #[serde(rename = "oeo")]
    triplet_oeo: f32,
    #[serde(rename = "ose")]
    triplet_ose: f32,
    #[serde(rename = "oss")]
    triplet_oss: f32,
    #[serde(rename = "oso")]
    triplet_oso: f32,
    #[serde(rename = "ooe")]
    triplet_ooe: f32,
    #[serde(rename = "oos")]
    triplet_oos: f32,
    #[serde(rename = "ooo")]
    triplet_ooo: f32,
}

impl TripletFn {
    pub fn from_fn(mut f: impl FnMut() -> f32) -> Self {
        Self {
            triplet_eee: f(),
            triplet_ees: f(),
            triplet_eeo: f(),
            triplet_ese: f(),
            triplet_ess: f(),
            triplet_eso: f(),
            triplet_eoe: f(),
            triplet_eos: f(),
            triplet_eoo: f(),
            triplet_oee: f(),
            triplet_oes: f(),
            triplet_oeo: f(),
            triplet_ose: f(),
            triplet_oss: f(),
            triplet_oso: f(),
            triplet_ooe: f(),
            triplet_oos: f(),
            triplet_ooo: f(),
        }
    }

    pub fn translate(self, amount: f32) -> Self {
        Self {
            triplet_eee: self.triplet_eee + amount,
            triplet_ees: self.triplet_ees + amount,
            triplet_eeo: self.triplet_eeo + amount,
            triplet_ese: self.triplet_ese + amount,
            triplet_ess: self.triplet_ess + amount,
            triplet_eso: self.triplet_eso + amount,
            triplet_eoe: self.triplet_eoe + amount,
            triplet_eos: self.triplet_eos + amount,
            triplet_eoo: self.triplet_eoo + amount,
            triplet_oee: self.triplet_oee + amount,
            triplet_oes: self.triplet_oes + amount,
            triplet_oeo: self.triplet_oeo + amount,
            triplet_ose: self.triplet_ose + amount,
            triplet_oss: self.triplet_oss + amount,
            triplet_oso: self.triplet_oso + amount,
            triplet_ooe: self.triplet_ooe + amount,
            triplet_oos: self.triplet_oos + amount,
            triplet_ooo: self.triplet_ooo + amount,
        }
    }

    pub fn scale(self, amount: f32) -> Self {
        Self {
            triplet_eee: self.triplet_eee * amount,
            triplet_ees: self.triplet_ees * amount,
            triplet_eeo: self.triplet_eeo * amount,
            triplet_ese: self.triplet_ese * amount,
            triplet_ess: self.triplet_ess * amount,
            triplet_eso: self.triplet_eso * amount,
            triplet_eoe: self.triplet_eoe * amount,
            triplet_eos: self.triplet_eos * amount,
            triplet_eoo: self.triplet_eoo * amount,
            triplet_oee: self.triplet_oee * amount,
            triplet_oes: self.triplet_oes * amount,
            triplet_oeo: self.triplet_oeo * amount,
            triplet_ose: self.triplet_ose * amount,
            triplet_oss: self.triplet_oss * amount,
            triplet_oso: self.triplet_oso * amount,
            triplet_ooe: self.triplet_ooe * amount,
            triplet_oos: self.triplet_oos * amount,
            triplet_ooo: self.triplet_ooo * amount,
        }
    }

    pub fn mutate(&self, mut mutator: impl FnMut(f32) -> f32) -> Self {
        Self {
            triplet_eee: mutator(self.triplet_eee),
            triplet_ees: mutator(self.triplet_ees),
            triplet_eeo: mutator(self.triplet_eeo),
            triplet_ese: mutator(self.triplet_ese),
            triplet_ess: mutator(self.triplet_ess),
            triplet_eso: mutator(self.triplet_eso),
            triplet_eoe: mutator(self.triplet_eoe),
            triplet_eos: mutator(self.triplet_eos),
            triplet_eoo: mutator(self.triplet_eoo),
            triplet_oee: mutator(self.triplet_oee),
            triplet_oes: mutator(self.triplet_oes),
            triplet_oeo: mutator(self.triplet_oeo),
            triplet_ose: mutator(self.triplet_ose),
            triplet_oss: mutator(self.triplet_oss),
            triplet_oso: mutator(self.triplet_oso),
            triplet_ooe: mutator(self.triplet_ooe),
            triplet_oos: mutator(self.triplet_oos),
            triplet_ooo: mutator(self.triplet_ooo),
        }
    }

    pub fn min(&self) -> f32 {
        self.triplet_eee
            .min(self.triplet_ees)
            .min(self.triplet_eeo)
            .min(self.triplet_ese)
            .min(self.triplet_ess)
            .min(self.triplet_eso)
            .min(self.triplet_eoe)
            .min(self.triplet_eos)
            .min(self.triplet_eoo)
            .min(self.triplet_oee)
            .min(self.triplet_oes)
            .min(self.triplet_oeo)
            .min(self.triplet_ose)
            .min(self.triplet_oss)
            .min(self.triplet_oso)
            .min(self.triplet_ooe)
            .min(self.triplet_oos)
            .min(self.triplet_ooo)
    }

    pub fn max(&self) -> f32 {
        self.triplet_eee
            .max(self.triplet_ees)
            .max(self.triplet_eeo)
            .max(self.triplet_ese)
            .max(self.triplet_ess)
            .max(self.triplet_eso)
            .max(self.triplet_eoe)
            .max(self.triplet_eos)
            .max(self.triplet_eoo)
            .max(self.triplet_oee)
            .max(self.triplet_oes)
            .max(self.triplet_oeo)
            .max(self.triplet_ose)
            .max(self.triplet_oss)
            .max(self.triplet_oso)
            .max(self.triplet_ooe)
            .max(self.triplet_oos)
            .max(self.triplet_ooo)
    }

    pub fn score(
        &self,
        entity_i: GreedyEntity,
        entity_j: GreedyEntity,
        entity_k: GreedyEntity,
    ) -> f32 {
        match (entity_i, entity_j, entity_k) {
            (GreedyEntity::Empty, GreedyEntity::Empty, GreedyEntity::Empty) => self.triplet_eee,
            (GreedyEntity::Empty, GreedyEntity::Empty, GreedyEntity::MyCell) => self.triplet_ees,
            (GreedyEntity::Empty, GreedyEntity::Empty, GreedyEntity::OtherCell) => self.triplet_eeo,
            (GreedyEntity::Empty, GreedyEntity::MyCell, GreedyEntity::Empty) => self.triplet_ese,
            (GreedyEntity::Empty, GreedyEntity::MyCell, GreedyEntity::MyCell) => self.triplet_ess,
            (GreedyEntity::Empty, GreedyEntity::MyCell, GreedyEntity::OtherCell) => {
                self.triplet_eso
            }
            (GreedyEntity::Empty, GreedyEntity::OtherCell, GreedyEntity::Empty) => self.triplet_eoe,
            (GreedyEntity::Empty, GreedyEntity::OtherCell, GreedyEntity::MyCell) => {
                self.triplet_eos
            }
            (GreedyEntity::Empty, GreedyEntity::OtherCell, GreedyEntity::OtherCell) => {
                self.triplet_eoo
            }
            (GreedyEntity::OtherCell, GreedyEntity::Empty, GreedyEntity::Empty) => self.triplet_oee,
            (GreedyEntity::OtherCell, GreedyEntity::Empty, GreedyEntity::MyCell) => {
                self.triplet_oes
            }
            (GreedyEntity::OtherCell, GreedyEntity::Empty, GreedyEntity::OtherCell) => {
                self.triplet_oeo
            }
            (GreedyEntity::OtherCell, GreedyEntity::MyCell, GreedyEntity::Empty) => {
                self.triplet_ose
            }
            (GreedyEntity::OtherCell, GreedyEntity::MyCell, GreedyEntity::MyCell) => {
                self.triplet_oss
            }
            (GreedyEntity::OtherCell, GreedyEntity::MyCell, GreedyEntity::OtherCell) => {
                self.triplet_oso
            }
            (GreedyEntity::OtherCell, GreedyEntity::OtherCell, GreedyEntity::Empty) => {
                self.triplet_ooe
            }
            (GreedyEntity::OtherCell, GreedyEntity::OtherCell, GreedyEntity::MyCell) => {
                self.triplet_oos
            }
            (GreedyEntity::OtherCell, GreedyEntity::OtherCell, GreedyEntity::OtherCell) => {
                self.triplet_ooo
            }
            (GreedyEntity::MyCell, _, _) => panic!("Invalid entity triplet"),
        }
    }
}
