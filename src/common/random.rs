//! Helper function for random number generator
//! 
use rand::prelude::*;


/// Generate random number between start (inclusive) and end (exclusive).
pub fn random_range(rng: &mut StdRng, start: usize, end: usize) -> usize {
    let max = (end - start) as u32;
    if max == 0 {
        start
    } else {
        ((rng.next_u32() % max) + start as u32) as usize
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use rand::prelude::*;
    use super::*;

    #[test]
    fn test_range() {
        let system_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Can't access system time");
        let mut rng = StdRng::seed_from_u64(system_time.as_millis() as u64);
        let x = random_range(&mut rng, 5, 8);
        assert!(x >= 5 && x < 8);
    }

}