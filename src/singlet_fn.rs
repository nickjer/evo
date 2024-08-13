use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct SingletFn {
    #[serde(rename = "e")]
    singlet_e: f32,
    #[serde(rename = "o")]
    singlet_o: f32,
}

impl SingletFn {
    pub fn from_fn(mut f: impl FnMut() -> f32) -> Self {
        Self {
            singlet_e: f(),
            singlet_o: f(),
        }
    }

    pub fn mutate(&self, mut mutator: impl FnMut(f32) -> f32) -> Self {
        Self {
            singlet_e: mutator(self.singlet_e),
            singlet_o: mutator(self.singlet_o),
        }
    }

    pub fn score(&self, entity_i: Entity) -> f32 {
        match entity_i {
            Entity::Empty => self.singlet_e,
            Entity::Cell(_) => self.singlet_o,
        }
    }
}
