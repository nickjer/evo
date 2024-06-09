use crate::active_plant::ActivePlant;
use crate::either::Either;
use crate::inactive_plant::InactivePlant;
use derive_more::{Display, From, Into};

#[derive(
    Debug, Copy, Clone, Default, Display, Hash, PartialEq, Eq, PartialOrd, Ord, From, Into,
)]
pub struct PlantId(usize);

#[derive(Debug, Clone, Default)]
pub struct Plants(Vec<Either<ActivePlant, InactivePlant>>);

impl Plants {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, genome: Either<ActivePlant, InactivePlant>) {
        self.0.push(genome);
    }
}

impl std::ops::Index<PlantId> for Plants {
    type Output = Either<ActivePlant, InactivePlant>;

    #[inline]
    fn index(&self, plant_id: PlantId) -> &Self::Output {
        self.0.index(usize::from(plant_id))
    }
}

impl std::ops::IndexMut<PlantId> for Plants {
    #[inline]
    fn index_mut(&mut self, plant_id: PlantId) -> &mut Self::Output {
        self.0.index_mut(usize::from(plant_id))
    }
}
