use crate::entity::Entity;
use crate::plants::PlantId;
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

    pub fn score(
        &self,
        plant_s: PlantId,
        entity_i: Entity,
        entity_j: Entity,
        entity_k: Entity,
    ) -> f32 {
        match (entity_i, entity_j, entity_k) {
            (Entity::Empty, Entity::Empty, Entity::Empty) => self.triplet_eee,
            (Entity::Empty, Entity::Empty, Entity::Cell(plant_2)) if plant_2 == plant_s => {
                self.triplet_ees
            }
            (Entity::Empty, Entity::Empty, Entity::Cell(_)) => self.triplet_eeo,
            (Entity::Empty, Entity::Cell(plant_1), Entity::Empty) if plant_1 == plant_s => {
                self.triplet_ese
            }
            (Entity::Empty, Entity::Cell(plant_1), Entity::Cell(plant_2))
                if plant_1 == plant_s && plant_2 == plant_s =>
            {
                self.triplet_ess
            }
            (Entity::Empty, Entity::Cell(plant_1), Entity::Cell(_)) if plant_1 == plant_s => {
                self.triplet_eso
            }
            (Entity::Empty, Entity::Cell(_), Entity::Empty) => self.triplet_eoe,
            (Entity::Empty, Entity::Cell(_), Entity::Cell(plant_2)) if plant_2 == plant_s => {
                self.triplet_eos
            }
            (Entity::Empty, Entity::Cell(_), Entity::Cell(_)) => self.triplet_eoo,
            (Entity::Cell(_), Entity::Empty, Entity::Empty) => self.triplet_oee,
            (Entity::Cell(_), Entity::Empty, Entity::Cell(plant_2)) if plant_2 == plant_s => {
                self.triplet_oes
            }
            (Entity::Cell(_), Entity::Empty, Entity::Cell(_)) => self.triplet_oeo,
            (Entity::Cell(_), Entity::Cell(plant_1), Entity::Empty) if plant_1 == plant_s => {
                self.triplet_ose
            }
            (Entity::Cell(_), Entity::Cell(plant_1), Entity::Cell(plant_2))
                if plant_1 == plant_s && plant_2 == plant_s =>
            {
                self.triplet_oss
            }
            (Entity::Cell(_), Entity::Cell(plant_1), Entity::Cell(_)) if plant_1 == plant_s => {
                self.triplet_oso
            }
            (Entity::Cell(_), Entity::Cell(_), Entity::Empty) => self.triplet_ooe,
            (Entity::Cell(_), Entity::Cell(_), Entity::Cell(plant_2)) if plant_2 == plant_s => {
                self.triplet_oos
            }
            (Entity::Cell(_), Entity::Cell(_), Entity::Cell(_)) => self.triplet_ooo,
        }
    }
}
