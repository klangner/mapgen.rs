//! Apply noise to the map.
//! Each cell will be set to Floor with the given probabilty.
//!
//! Example usage:
//! ```
//! use mapgen::{CaveMap, MapFilter};
//! use mapgen::cave::NoiseGenerator;
//! use fastrand::Rng;
//!
//! let mut rng = Rng::with_seed(100);
//! let gen = NoiseGenerator::uniform();
//! let map = gen.modify_map(&mut rng, &CaveMap::new(80, 50));
//!
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//!

use fastrand::Rng;

use crate::CaveMap;
use crate::MapFilter;

/// Map noise generator
pub struct NoiseGenerator {
    prob: f32,
}

impl MapFilter for NoiseGenerator {
    fn modify_map(&self, rng: &mut Rng, map: &CaveMap) -> CaveMap {
        self.build(map, rng)
    }
}

impl NoiseGenerator {
    /// Create noise with custom probability
    pub fn new(prob: f32) -> Box<NoiseGenerator> {
        Box::new(NoiseGenerator { prob })
    }

    /// Create uniform noise (Probablity 0.5)
    pub fn uniform() -> Box<NoiseGenerator> {
        Box::new(NoiseGenerator { prob: 0.5 })
    }

    /// Generate map
    fn build(&self, map: &CaveMap, rng: &mut Rng) -> CaveMap {
        let mut new_map = map.clone();
        let p = (self.prob * 100.0) as u32;
        for y in 1..new_map.height - 1 {
            for x in 1..new_map.width - 1 {
                let roll = rng.u32(0..u32::MAX) % 100;
                if roll > p {
                    new_map.set_walkable(x, y, true)
                } else {
                    new_map.set_walkable(x, y, false)
                }
            }
        }

        new_map
    }
}
