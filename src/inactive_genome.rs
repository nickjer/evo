use crate::genome::Genome;
use derive_more::Constructor;

#[derive(Debug, Clone, Constructor)]
pub struct InactiveGenome {
    genome: Genome,
}
