use crate::genomes::{GenomeId, TripletGenome};
use derive_more::Constructor;
use getset::CopyGetters;
use serde::Serialize;

#[derive(Debug, Clone, Constructor, CopyGetters, Serialize)]
pub struct InactiveGenome {
    id: GenomeId,
    #[serde(flatten)]
    #[getset(get = "pub")]
    genome: TripletGenome,
    #[getset(get_copy = "pub")]
    max_yield: usize,
    created_at: usize,
    died_at: usize,
    parent_genome_id: Option<GenomeId>,
}
