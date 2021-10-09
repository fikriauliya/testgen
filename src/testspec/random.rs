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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_to_be_deterministic() {
        assert_eq!(Random::new(0).next::<i32>(), -852725889);
        assert_eq!(Random::new(0).next::<u32>(), 3442241407);
        assert_eq!(Random::new(0).next::<f32>(), 0.80145925);
        assert_eq!(Random::new(0).next::<f64>(), 0.7311134158637046);
        assert_eq!(Random::new(0).next::<bool>(), true);
        assert_eq!(Random::new(0).next::<char>(), '\u{da18a}');
    }

    #[test]
    fn test_next_range_returns_number_within_range() {
        assert!(Random::new(0).next_range(0, 1) <= 1);
        assert!(Random::new(0).next_range(0, 1) >= 0);
    }
}
