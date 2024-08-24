use crate::active_genome::ActiveGenome;
use crate::either::Either;
use crate::inactive_genome::InactiveGenome;
use derive_more::{Display, From, Into, IntoIterator};
use serde::Serialize;

#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    Into,
    Serialize,
)]
pub struct GenomeId(usize);

#[derive(Debug, Clone, Default, IntoIterator)]
pub struct Genomes(#[into_iterator(ref)] Vec<Either<ActiveGenome, InactiveGenome>>);

impl Genomes {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, genome: Either<ActiveGenome, InactiveGenome>) {
        self.0.push(genome);
    }
}

impl std::ops::Index<GenomeId> for Genomes {
    type Output = Either<ActiveGenome, InactiveGenome>;

    #[inline]
    fn index(&self, genome_id: GenomeId) -> &Self::Output {
        self.0.index(usize::from(genome_id))
    }
}

impl std::ops::IndexMut<GenomeId> for Genomes {
    #[inline]
    fn index_mut(&mut self, genome_id: GenomeId) -> &mut Self::Output {
        self.0.index_mut(usize::from(genome_id))
    }
}
