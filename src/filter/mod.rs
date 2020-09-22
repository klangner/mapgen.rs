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
//! use mapgen::{MapFilter, MapBuilder, Map, TileType};
//! use mapgen::filter::{
//!     CellularAutomata,
//!     starting_point::{AreaStartingPosition, XStart, YStart}
//! };
//! use mapgen::geometry::Point;
//! 
//! let map = MapBuilder::new()
//!             .with(CellularAutomata::new())
//!             .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
//!             .build_map(80, 50);
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! assert_eq!(map.starting_point.is_some(), true);
//! ```
//! 

pub mod bsp_interior;
pub mod bsp_rooms;
pub mod cellular_automata;
pub mod cull_unreachable;
pub mod distant_exit;
pub mod drunkard;
pub mod simple_rooms;
pub mod rooms_corridors_nearest;
pub mod starting_point;

pub use bsp_interior::BspInterior;
pub use bsp_rooms::BspRooms;
pub use cellular_automata::CellularAutomata;
pub use cull_unreachable::CullUnreachable;
pub use distant_exit::DistantExit;
pub use drunkard::DrunkardsWalk;
pub use simple_rooms::SimpleRooms;
pub use rooms_corridors_nearest::NearestCorridors;
pub use starting_point::AreaStartingPosition;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;
use crate::map::Map;


/// Trait which should be implemented by map modifier. 
/// Modifier takes initiall map and apply changes to it.
pub trait MapFilter {
    fn modify_map(&self, rng: &mut StdRng, map: &Map) -> Map;
}

/// Used to chain MapBuilder and MapModifiers to create the final map.
pub struct MapBuilder {
    modifiers: Vec<Box<dyn MapFilter>>,
}

impl MapBuilder {
    /// Create Map Builder with initial map generator
    pub fn new() -> MapBuilder {
        MapBuilder { 
            modifiers: Vec::new(),
        }
    }

    pub fn with(&mut self, modifier : Box<dyn MapFilter>) -> &mut MapBuilder {
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
        let mut map = Map::new(width, height);
        
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
    use cellular_automata::CellularAutomata;
    use starting_point::{AreaStartingPosition, XStart, YStart};

    #[test]
    fn test_ca_map() {
        let map = MapBuilder::new()
            .with(CellularAutomata::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .build_map(80, 50);

        assert_eq!(map.width, 80);
        assert_eq!(map.height, 50);
        assert_eq!(map.starting_point.is_some(), true);
    }

}