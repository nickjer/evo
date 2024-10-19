use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_distr::StandardNormal;

#[derive(Debug, Clone)]
pub struct Rng(SmallRng);

impl Rng {
    pub fn from_seed(seed: u64) -> Self {
        Self(SmallRng::seed_from_u64(seed))
    }

    pub fn norm(&mut self) -> f32 {
        StandardNormal.sample(&mut self.0)
    }

    pub fn sample(&mut self) -> f32 {
        Uniform::new(0.0, 1.0).sample(&mut self.0)
    }

    pub fn uniform(&mut self, max: usize) -> usize {
        Uniform::new(0, max).sample(&mut self.0)
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        slice.shuffle(&mut self.0);
    }
}
