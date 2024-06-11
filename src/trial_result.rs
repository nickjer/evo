use crate::active_genome::ActiveGenome;
use crate::either::Either;
use crate::inactive_genome::InactiveGenome;
use derive_more::Constructor;
use serde::Serialize;

#[derive(Debug, Constructor, Serialize)]
pub struct TrialResult<'a> {
    top_genomes: Vec<&'a Either<ActiveGenome, InactiveGenome>>,
}
