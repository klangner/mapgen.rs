//! Random rooms map generator.
//!
//! Try to generate rooms of different size to fill the map area.
//! Rooms will not overlap.
//!
//! Example generator usage:
//! ```
//! use mapgen::rooms::SimpleRooms;
//! use fastrand::Rng;
//!
//! let mut rng = Rng::with_seed(100);
//! let simple_rooms = SimpleRooms::default();
//! let map = simple_rooms.generate(80, 50, &mut rng);
//!
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//!

use fastrand::Rng;

use super::RoomsMap;
use crate::geometry::Rect;

pub struct SimpleRooms {
    max_rooms: u32,
    min_room_size: u32,
    max_room_size: u32,
}

impl SimpleRooms {
    pub fn new(max_rooms: u32, min_room_size: u32, max_room_size: u32) -> Self {
        Self {
            max_rooms,
            min_room_size,
            max_room_size,
        }
    }

    pub fn generate(&self, map_width: u32, max_height: u32, rng: &mut Rng) -> RoomsMap {
        // Create room with dimensions
        let mut map = RoomsMap::new(map_width, max_height);

        for _ in 0..self.max_rooms {
            let w = rng.choice(self.min_room_size..self.max_room_size).unwrap();
            let h = rng.choice(self.min_room_size..self.max_room_size).unwrap();
            let x = rng.choice(1..map.width - w).unwrap();
            let y = rng.choice(1..map.height - h).unwrap();
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
