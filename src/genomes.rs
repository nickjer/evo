use crate::genome::Genome;
use derive_more::{Display, From, Into};

#[derive(
    Debug, Copy, Clone, Default, Display, Hash, PartialEq, Eq, PartialOrd, Ord, From, Into,
)]
pub struct GenomeId(usize);

#[derive(Debug, Clone, Default)]
pub struct Genomes(Vec<Genome>);

impl Genomes {
    pub fn add(&mut self, genome: Genome) -> GenomeId {
        self.0.push(genome);
        GenomeId::from(self.0.len() - 1)
    }
}

impl std::ops::Index<GenomeId> for Genomes {
    type Output = Genome;

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
