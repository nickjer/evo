use crate::genome::Genome;
use derive_more::Constructor;
use getset::CopyGetters;
use serde::Serialize;

#[derive(Debug, Clone, Constructor, CopyGetters, Serialize)]
pub struct InactiveGenome {
    genome: Genome,
    #[getset(get_copy = "pub")]
    max_yield: usize,
}
