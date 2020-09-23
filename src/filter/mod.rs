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
//!     NoiseGenerator,
//!     CellularAutomata,
//!     starting_point::{AreaStartingPosition, XStart, YStart}
//! };
//! use mapgen::geometry::Point;
//! 
//! let map = MapBuilder::new()
//!             .with(NoiseGenerator::uniform())
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
pub mod noise_generator;
pub mod simple_rooms;
pub mod rooms_corridors_nearest;
pub mod starting_point;

pub use bsp_interior::BspInterior;
pub use bsp_rooms::BspRooms;
pub use cellular_automata::CellularAutomata;
pub use cull_unreachable::CullUnreachable;
pub use distant_exit::DistantExit;
pub use drunkard::DrunkardsWalk;
pub use noise_generator::NoiseGenerator;
pub use simple_rooms::SimpleRooms;
pub use rooms_corridors_nearest::NearestCorridors;
pub use starting_point::{AreaStartingPosition, XStart, YStart};

