//! Random rooms map generator.
//! 
//! Try to generate rooms of different size to fill the map area. 
//! Rooms will not overlap.
//! 
//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::dungeon::{
//!     MapGenerator,
//!     simple_rooms::SimpleRoomsGen
//! };
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = SimpleRoomsGen::new();
//! let map = gen.generate_map(80, 50, &mut rng);
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use super::MapGenerator;
use crate::common::geometry::Rect;
use crate::common::random::Rng;
use super::map::{Map};


pub struct SimpleRoomsGen {
    max_rooms: usize,
    min_room_size: usize,
    max_room_size: usize,
}

impl MapGenerator for SimpleRoomsGen {
    fn generate_map(&self, width: usize, height: usize, rng : &mut StdRng) -> Map {
        self.build_rooms(width, height, rng)
    }
}


impl SimpleRoomsGen {
    pub fn new() -> Box<SimpleRoomsGen> {
        Box::new(SimpleRoomsGen{
            max_rooms: 30,
            min_room_size: 6,
            max_room_size: 10
        })
    }

    fn build_rooms(&self, width: usize, height: usize, rng : &mut StdRng) -> Map {
        let mut map = Map::new(width, height);

        // Create room dimensions
        for _ in 0..self.max_rooms {
            let w = rng.random_range(self.min_room_size, self.max_room_size);
            let h = rng.random_range(self.min_room_size, self.max_room_size);
            let x = rng.random_range(1, width - w);
            let y = rng.random_range(1, height - h);
            let new_room = Rect::new(x, y, w, h);
            let intersects = map.rooms.iter().any(|r| new_room.intersect(r));
            if !intersects {
                map.add_room(new_room);
            }
        }
        
        map
    }
}