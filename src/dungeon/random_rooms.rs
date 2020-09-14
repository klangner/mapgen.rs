//! Random rooms map generator.
//! 
//! Try to generate rooms of different size to fille the whole map area.
//! 
//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::dungeon::{
//!     MapGenerator,
//!     random_rooms::RandomRoomsGen
//! };
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = RandomRoomsGen::new();
//! let map = gen.generate_map(80, 50, &mut rng);
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use super::MapGenerator;
use crate::common::geometry::Rect;
use crate::common::random;
use super::map::{Map};


pub struct RandomRoomsGen {
    max_rooms: usize,
    min_room_size: usize,
    max_room_size: usize,
}

impl MapGenerator for RandomRoomsGen {
    fn generate_map(&self, width: usize, height: usize, rng : &mut StdRng) -> Map {
        self.build_rooms(width, height, rng)
    }
}


impl RandomRoomsGen {
    pub fn new() -> RandomRoomsGen {
        RandomRoomsGen{
            max_rooms: 30,
            min_room_size: 6,
            max_room_size: 10
        }
    }

    fn build_rooms(&self, width: usize, height: usize, rng : &mut StdRng) -> Map {
        let mut map = Map::new(width, height);
        let mut rooms : Vec<Rect> = Vec::new();

        // Create room dimensions
        for _ in 0..self.max_rooms {
            let w = random::random_range(rng, self.min_room_size, self.max_room_size);
            let h = random::random_range(rng, self.min_room_size, self.max_room_size);
            let x = random::random_range(rng, 0, width - w);
            let y = random::random_range(rng, 0, height - h);
            let new_room = Rect::new(x as i32, y as i32, w as i32, h as i32);
            let intersects = rooms.iter().any(|r| new_room.intersect(r));
            if !intersects {
                rooms.push(new_room);
            }
        }

        // Apply rooms to the map
        for room in rooms {
            map.create_room(&room);
        }
        
        map
    }
}