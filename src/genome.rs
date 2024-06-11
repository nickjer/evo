use crate::doublet_fn::DoubletFn;
use crate::singlet_fn::SingletFn;
use crate::triplet_fn::TripletFn;
use derive_more::Constructor;
use getset::Getters;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Constructor, Getters, Serialize)]
pub struct Genome {
    #[getset(get = "pub")]
    singlet_fn: SingletFn,
    #[getset(get = "pub")]
    doublet_fn: DoubletFn,
    #[getset(get = "pub")]
    triplet_l_fn: TripletFn,
    #[getset(get = "pub")]
    triplet_i_fn: TripletFn,
}

impl Genome {
    pub fn mutate(&self, mut mutator: impl FnMut(f32) -> f32) -> Self {
        Self::new(
            self.singlet_fn.mutate(&mut mutator),
            self.doublet_fn.mutate(&mut mutator),
            self.triplet_l_fn.mutate(&mut mutator),
            self.triplet_i_fn.mutate(&mut mutator),
        )
    }
}
