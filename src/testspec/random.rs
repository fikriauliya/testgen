use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    Rng, SeedableRng,
};

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
