use rand::{
    distributions::{uniform::SampleUniform, Alphanumeric, Standard},
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

    pub fn next_string(&mut self, len: usize) -> String {
        (&mut self.rnd)
            .sample_iter(Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
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
    fn test_next_returns_random_number() {
        let mut rnd = Random::new(0);
        assert_ne!(rnd.next::<i32>(), rnd.next::<i32>());
    }

    #[test]
    fn test_next_range_returns_number_within_range() {
        assert!(Random::new(0).next_range(0, 1) <= 1);
        assert!(Random::new(0).next_range(0, 1) >= 0);
    }

    #[test]
    fn test_next_string_returns_correct_length() {
        assert_eq!(Random::new(0).next_string(0).len(), 0);
        assert_eq!(Random::new(0).next_string(1).len(), 1);
        assert_eq!(Random::new(0).next_string(2).len(), 2);
    }

    #[test]
    fn test_next_string_returns_random_string() {
        let mut rnd = Random::new(0);
        let s1 = rnd.next_string(10);
        let s2 = rnd.next_string(10);
        assert_ne!(s1, s2);
    }
}
