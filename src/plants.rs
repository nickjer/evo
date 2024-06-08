use crate::genomes::GenomeId;
use crate::plant::Plant;
use derive_more::{Display, From, Into};

#[derive(
    Debug, Copy, Clone, Default, Display, Hash, PartialEq, Eq, PartialOrd, Ord, From, Into,
)]
pub struct PlantId(usize);

#[derive(Debug, Clone, Default, From, Into)]
pub struct Plants {
    list: Vec<Option<Plant>>,
    active_ids: Vec<PlantId>,
}

impl Plants {
    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn active_len(&self) -> usize {
        self.active_ids.len()
    }

    pub fn active_ids(&self) -> Vec<PlantId> {
        self.active_ids.clone()
    }

    pub fn add(&mut self, genome_id: GenomeId) -> &mut Plant {
        let id = PlantId::from(self.list.len());
        self.active_ids.push(id);

        let plant = Plant::new(id, genome_id);
        self.list.push(Some(plant));
        self.list.last_mut().unwrap().as_mut().unwrap()
    }

    pub fn remove(&mut self, plant_id: PlantId) -> Plant {
        self.active_ids.retain(|&id| id != plant_id);
        self.list
            .get_mut(usize::from(plant_id))
            .unwrap()
            .take()
            .unwrap()
    }
}

impl std::ops::Index<PlantId> for Plants {
    type Output = Plant;

    #[inline]
    fn index(&self, plant_id: PlantId) -> &Self::Output {
        self.list.index(usize::from(plant_id)).as_ref().unwrap()
    }
}

impl std::ops::IndexMut<PlantId> for Plants {
    #[inline]
    fn index_mut(&mut self, plant_id: PlantId) -> &mut Self::Output {
        self.list.index_mut(usize::from(plant_id)).as_mut().unwrap()
    }
}
