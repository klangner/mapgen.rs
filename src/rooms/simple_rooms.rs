//! Random rooms map generator.
//!
//! Try to generate rooms of different size to fill the map area.
//! Rooms will not overlap.
//!
//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::rooms::SimpleRooms;
//!
//! let mut rng = StdRng::seed_from_u64(100);
//! let simple_rooms = SimpleRooms::default();
//! let map = simple_rooms.generate_rooms(80, 50, &mut rng);
//!
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//!

use super::RoomBasedMap;
use crate::geometry::Rect;
use crate::random::Rng;
use rand::prelude::*;

pub struct SimpleRooms {
    max_rooms: usize,
    min_room_size: usize,
    max_room_size: usize,
}

impl SimpleRooms {
    pub fn new(max_rooms: usize, min_room_size: usize, max_room_size: usize) -> Self {
        Self {
            max_rooms,
            min_room_size,
            max_room_size,
        }
    }

    pub fn generate_rooms(
        &self,
        map_width: usize,
        max_height: usize,
        rng: &mut StdRng,
    ) -> RoomBasedMap {
        // Create room with dimensions
        let mut map = RoomBasedMap::new(map_width, max_height);

        for _ in 0..self.max_rooms {
            let w = rng.random_range(self.min_room_size, self.max_room_size);
            let h = rng.random_range(self.min_room_size, self.max_room_size);
            let x = rng.random_range(1, map.width - w);
            let y = rng.random_range(1, map.height - h);
            let new_room = Rect::new(x, y, w, h);
            let intersects = map.rooms.iter().any(|r| new_room.intersect(r));
            if !intersects {
                map.add_room(new_room);
            }
        }

        map
    }
}

impl Default for SimpleRooms {
    fn default() -> Self {
        Self {
            max_rooms: 30,
            min_room_size: 6,
            max_room_size: 10,
        }
    }
}
