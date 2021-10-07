use rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Standard,
    },
    prelude::Distribution,
    Rng, SeedableRng,
};

pub trait TestSpec<T> {
    fn test_cases(random: &mut Random) -> Vec<T>;
}

pub struct Random {
    rnd: rand::rngs::StdRng,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Random {
            rnd: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    pub fn next<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rnd.gen()
    }

    pub fn next_range<T>(&mut self, from: T, to: T) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        self.rnd.gen_range(from..=to)
    }
}
