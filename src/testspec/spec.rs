use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    Rng, SeedableRng,
};

use crate::problemspec::spec::MultitaskProblemSpec;

pub trait SingletaskTestSpec<T> {
    fn sample_test_cases() -> Vec<T>;
    fn test_cases(random: &mut Random) -> Vec<T>;
}

pub trait MultitaskTestSpec<T>
where
    T: MultitaskProblemSpec<T>,
{
    fn sample_test_cases() -> Vec<T>;
    fn test_cases_subtask_1(_random: &mut Random) -> Option<Vec<T>> {
        None
    }
    fn test_cases_subtask_2(_random: &mut Random) -> Option<Vec<T>> {
        None
    }
    fn test_cases_subtask_3(_random: &mut Random) -> Option<Vec<T>> {
        None
    }
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
