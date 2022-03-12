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
//! use mapgen::{MapFilter, MapBuilder, Map, NoData, Tile};
//! use mapgen::filter::{
//!     NoiseGenerator,
//!     CellularAutomata,
//!     starting_point::{AreaStartingPosition, XStart, YStart}
//! };
//! use mapgen::geometry::Point;
//! 
//! let map = MapBuilder::<NoData>::new(80, 50)
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
pub mod map;
pub mod metric;

pub use map::{BuilderData, Map, NoData, Symmetry, Tile};
pub use filter::*;

pub (crate) mod dijkstra;
pub (crate) mod random;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;


/// Trait which should be implemented by map modifier. 
/// Modifier takes initiall map and apply changes to it.
pub trait MapFilter<D: BuilderData> {
    fn modify_map(&self, rng: &mut StdRng, map: &Map<D>) -> Map<D>;
}

/// Used to chain MapBuilder and MapModifiers to create the final map.
pub struct MapBuilder<D> {
    width: usize,
    height: usize,
    modifiers: Vec<Box<dyn MapFilter<D>>>,
}

impl<D: BuilderData> MapBuilder<D> {
    /// Create Map Builder with initial map generator
    pub fn new(width: usize, height: usize) -> MapBuilder<D> {
        MapBuilder { 
            width,
            height,
            modifiers: Vec::new(),
        }
    }

    pub fn with(&mut self, modifier : Box<dyn MapFilter<D>>) -> &mut MapBuilder<D> {
        self.modifiers.push(modifier);
        self
    }

    /// Build map using random number seeded with system time
    pub fn build(&mut self) -> Map<D> {
        let system_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Can't access system time");
        let mut rng = StdRng::seed_from_u64(system_time.as_millis() as u64);
        self.build_with_rng(&mut rng)
    }

    /// Build map using provided random number generator
    pub fn build_with_rng(&mut self, rng: &mut StdRng) -> Map<D> {
        let mut map = Map::new(self.width, self.height);
        
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
    use crate::map::NoData;

    use super::*;
    use filter::{
        CellularAutomata,
        NoiseGenerator,
        {AreaStartingPosition, XStart, YStart},
    };

    #[test]
    fn test_ca_map() {
        let map = MapBuilder::<NoData>::new(80, 50)
            .with(NoiseGenerator::new(0.55))
            .with(CellularAutomata::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .build();

        assert_eq!(map.width, 80);
        assert_eq!(map.height, 50);
        assert_eq!(map.starting_point.is_some(), true);
    }

}