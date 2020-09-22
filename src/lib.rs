//! A collection of map generators.
//! 
//! The generators are implemented for the following types of maps:
//!   * Dungeon maps
//! 
 
pub mod filter;
pub mod geometry;
pub mod map;

pub use map::{Map, TileType};
pub use filter::{MapFilter, MapBuilder};

pub (crate) mod dijkstra;
pub (crate) mod random;