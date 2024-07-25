//! This module generate map based on rooms and corridors.
//! This process is done in few steps:
//! * Generate room with one of the generators ([SimpleRooms], [BspInterior], etc)
//! * Generate corridors to connect rooms e.g [NearestCorridors]
//! * Then we can use add start end exit point [crate::poi::AreaStartingPosition]
//!     and [crate::poi::CullUnreachable]
//!

pub mod bsp_interior;
pub mod bsp_rooms;
pub mod corridors_nearest;
pub mod simple_rooms;
pub mod tile_map;

pub use bsp_interior::BspInterior;
pub use bsp_rooms::BspRooms;
pub use corridors_nearest::NearestCorridors;
pub use simple_rooms::SimpleRooms;
pub use tile_map::RoomsMap;
