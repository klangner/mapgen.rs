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
//! use mapgen::common::geometry::Point;
//! use mapgen::dungeon::{
//!     MapBuilder,
//!     map::{Map, TileType},
//!     cellular_automata::CellularAutomataGen,
//!     starting_point::{AreaStartingPosition, XStart, YStart},
//! };
//! 
//! let map = MapBuilder::new(Box::new(CellularAutomataGen::new()))
//!             .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
//!             .build_map(80, 50);
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! assert_eq!(map.starting_point.is_some(), true);
//! ```
//! 

pub mod map;
pub mod cellular_automata;
pub mod cull_unreachable;
pub mod distant_exit;
pub mod random_rooms;
pub mod starting_point;
mod dijkstra;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;
use map::Map;


/// Trait which should be implemented by any map generator which want to be used
/// by MapBuilder
pub trait MapGenerator {
    fn generate_map(&self, width: usize, height: usize, rng: &mut StdRng) -> Map;
}

/// Trait which should be implemented by map modifier. 
/// Modifier takes initiall map and apply changes to it.
pub trait MapModifier {
    fn modify_map(&self, rng: &mut StdRng, map: &Map) -> Map;
}

/// Used to chain MapBuilder and MapModifiers to create the final map.
pub struct MapBuilder {
    generator: Box<dyn MapGenerator>,
    modifiers: Vec<Box<dyn MapModifier>>,
}

impl MapBuilder {
    /// Create Map Builder with initial map generator
    pub fn new(generator : Box<dyn MapGenerator>) -> MapBuilder {
        MapBuilder { 
            generator, 
            modifiers: Vec::new(),
        }
    }

    pub fn with(&mut self, modifier : Box<dyn MapModifier>) -> &mut MapBuilder {
        self.modifiers.push(modifier);
        self
    }

    /// Build map using random number seeded with system time
    pub fn build_map(&mut self, width: usize, height: usize) -> Map {
        let system_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Can't access system time");
        let mut rng = StdRng::seed_from_u64(system_time.as_millis() as u64);
        self.build_map_with_rng(width, height, &mut rng)
    }

    /// Build map using provided random number generator
    pub fn build_map_with_rng(&mut self, width: usize, height: usize, rng: &mut StdRng) -> Map {
        let mut map = self.generator.generate_map(width, height, rng);
        
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
    use cellular_automata::CellularAutomataGen;
    use starting_point::{AreaStartingPosition, XStart, YStart};

    #[test]
    fn test_ca_map() {
        let map = MapBuilder::new(Box::new(CellularAutomataGen::new()))
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .build_map(80, 50);

        assert_eq!(map.width, 80);
        assert_eq!(map.height, 50);
        assert_eq!(map.starting_point.is_some(), true);
    }

}