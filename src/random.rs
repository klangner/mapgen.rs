//! Helper function for random number generator
//!
use rand::prelude::*;

pub trait Rng {
    /// Generate random number between start and end (bot inclusive).
    fn roll_dice(&mut self, min: u32, max: u32) -> u32;
    /// Generate random number between start (inclusive) and end (exclusive).
    fn random_range(&mut self, start: u32, end: u32) -> u32;
}

impl Rng for StdRng {
    fn roll_dice(&mut self, min: u32, max: u32) -> u32 {
        self.random_range(min, max + 1)
    }

    fn random_range(&mut self, start: u32, end: u32) -> u32 {
        let max = end - start;
        if max == 0 {
            start
        } else {
            (self.next_u32() % max) + start
        }
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::Rng;
    use rand::prelude::*;

    #[test]
    fn test_range() {
        let mut rng = StdRng::seed_from_u64(100);
        let x = rng.random_range(5, 8);
        assert!((5..8).contains(&x));
    }

    #[test]
    fn test_range_average() {
        let num_op = 10000;
        let mut rng = StdRng::seed_from_u64(1000);
        let xs: Vec<u32> = (0..num_op).map(|_| rng.random_range(5, 10)).collect();
        let mean = xs.iter().sum::<u32>() / num_op;
        let min = *xs.iter().min().expect("no min");
        let max = *xs.iter().max().expect("no max");
        assert_eq!(mean, 7);
        assert_eq!(min, 5);
        assert_eq!(max, 9);
    }

    #[test]
    fn test_roll_dice() {
        let num_op = 1000;
        let mut rng = StdRng::seed_from_u64(2000);
        let xs: Vec<u32> = (0..num_op).map(|_| rng.roll_dice(1, 7)).collect();
        let mean = xs.iter().sum::<u32>() as f32 / num_op as f32 + 0.5;
        let min = *xs.iter().min().expect("no min");
        let max = *xs.iter().max().expect("no max");
        assert_eq!(mean as usize, 4);
        assert_eq!(min, 1);
        assert_eq!(max, 7);
    }
}
