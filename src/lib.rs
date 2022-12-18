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
//! use mapgen::filter::{
//!     NoiseGenerator,
//!     CellularAutomata,
//!     starting_point::{AreaStartingPosition, XStart, YStart}
//! };
//! use mapgen::geometry::Point;
//! 
//! let map = MapBuilder::new(80, 50)
//!             .with(NoiseGenerator::uniform())
//!             .with(CellularAutomata::new())
//!             .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
//!             .build();
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! assert_eq!(map.starting_point.is_some(), true);
//! ```
//!  

pub mod filter;
pub mod geometry;
pub mod metric;
pub mod world_map;

pub (crate) mod dijkstra;
pub (crate) mod random;
pub (crate) mod map_buffer;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;

pub use map_buffer::{MapBuffer, Symmetry};
pub use world_map::WorldMap;
pub use filter::*;


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

    pub fn with(&mut self, modifier : Box<dyn MapFilter>) -> &mut MapBuilder {
        self.modifiers.push(modifier);
        self
    }

    /// Build map using random number seeded with system time
    pub fn build(&mut self) -> WorldMap {
        let system_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Can't access system time");
        let mut rng = StdRng::seed_from_u64(system_time.as_millis() as u64);
        self.build_with_rng(&mut rng)
    }

    /// Build map using provided random number generator
    pub fn build_with_rng(&mut self, rng: &mut StdRng) -> WorldMap {
        let mut map = MapBuffer::new(self.width, self.height);
        
        // Build additional layers in turn
        for modifier in self.modifiers.iter() {
            map = modifier.modify_map(rng, &map);
        }

        WorldMap::new(self.width, self.height, map.walkables, map.tile_types, map.starting_point, map.exit_point)
    }

}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use filter::{
        CellularAutomata,
        NoiseGenerator,
        {AreaStartingPosition, XStart, YStart},
    };

    #[test]
    fn test_ca_map() {
        let map = MapBuilder::new(80, 50)
            .with(NoiseGenerator::new(0.55))
            .with(CellularAutomata::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .build();

        assert_eq!(map.width, 80);
        assert_eq!(map.height, 50);
        assert!(map.starting_point.is_some());
    }

}