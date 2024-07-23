//! Generators for dungeon type maps.
//!
//! Generators can bu used directly or they can be combined with
//! `MapGenerator`s and `MapModifier`s
//!
//! * MapGenerators are use to create initial map.
//! * MapModifiers modify existing map.
//!
//! Example
//! ```
//! use mapgen::{MapFilter, MapBuilder};
//! use mapgen::dungeon::{
//!     NoiseGenerator,
//!     CellularAutomata,
//! };
//!
//! let map = MapBuilder::new(80, 50)
//!             .with(NoiseGenerator::uniform())
//!             .with(CellularAutomata::new())
//!             .build();
//!
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//!  

pub mod dungeon;
pub mod geometry;
pub mod layer;
pub mod metric;
pub mod poi;
pub mod rooms;

pub(crate) mod path;
pub(crate) mod random;

use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub use dungeon::*;
pub use tile_map::{MapBuffer, Symmetry};

/// Trait which should be implemented by map modifier.
/// Modifier takes initiall map and apply changes to it.
pub trait MapFilter {
    fn modify_map(&self, rng: &mut StdRng, map: &MapBuffer) -> MapBuffer;
}

/// Used to chain MapBuilder and MapModifiers to create the final map.
pub struct MapBuilder {
    width: usize,
    height: usize,
    modifiers: Vec<Box<dyn MapFilter>>,
}

impl MapBuilder {
    /// Create Map Builder with initial map generator
    pub fn new(width: usize, height: usize) -> MapBuilder {
        MapBuilder {
            width,
            height,
            modifiers: Vec::new(),
        }
    }

    pub fn with(&mut self, modifier: Box<dyn MapFilter>) -> &mut MapBuilder {
        self.modifiers.push(modifier);
        self
    }

    /// Build map using random number seeded with system time
    pub fn build(&mut self) -> MapBuffer {
        let system_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Can't access system time");
        let mut rng = StdRng::seed_from_u64(system_time.as_millis() as u64);
        self.build_with_rng(&mut rng)
    }

    /// Build map using provided random number generator
    pub fn build_with_rng(&mut self, rng: &mut StdRng) -> MapBuffer {
        let mut map = MapBuffer::new(self.width, self.height);

        // Build additional layers in turn
        for modifier in self.modifiers.iter() {
            map = modifier.modify_map(rng, &map);
        }

        map
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use dungeon::{CellularAutomata, NoiseGenerator};

    #[test]
    fn test_ca_map() {
        let map = MapBuilder::new(80, 50)
            .with(NoiseGenerator::new(0.55))
            .with(CellularAutomata::new())
            .build();

        assert_eq!(map.width, 80);
        assert_eq!(map.height, 50);
    }
}
