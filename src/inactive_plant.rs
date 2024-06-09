use crate::genomes::GenomeId;
use crate::plants::PlantId;
use derive_more::Constructor;
use getset::CopyGetters;

#[derive(Debug, Clone, CopyGetters, Default, Constructor)]
pub struct InactivePlant {
    id: PlantId,
    #[get_copy = "pub"]
    genome_id: GenomeId,
}
