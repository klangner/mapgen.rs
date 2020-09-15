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
//!     bsp_interior::BspInteriorGen
//! };
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = BspInteriorGen::new();
//! let map = gen.generate_map(80, 50, &mut rng);
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use super::MapGenerator;
use crate::common::geometry::{Point, Rect};
use crate::common::random::Rng;
use super::map::Map;


pub struct BspInteriorGen {
    min_room_size: usize,
}

impl MapGenerator for BspInteriorGen {
    fn generate_map(&self, width: usize, height: usize, rng : &mut StdRng) -> Map {
        self.build(rng, width, height)
    }
}

impl BspInteriorGen {

    pub fn new() -> Box<BspInteriorGen> {
        Box::new(BspInteriorGen{
            min_room_size: 8,
        })
    }

    fn build(&self, rng: &mut StdRng, width: usize, height: usize) -> Map {
        let mut map = Map::new(width, height);
        let mut rects: Vec<Rect> = Vec::new();
        rects.push( Rect::new(1, 1, map.width-2, map.height-2) ); 
        let first_room = rects[0];
        // Divide the first room
        self.add_subrects(first_room, rng, &mut rects); 

        let rooms_copy = rects.clone();
        for r in rooms_copy.iter() {
            let room = *r;
            map.add_room(room);
        }

        // Now we want corridors
        for i in 0..map.rooms.len()-1 {
            let room = map.rooms[i];
            let next_room = map.rooms[i+1];
            let start_x = room.x1 + rng.random_range(1, room.width());
            let start_y = room.y1 + rng.random_range(1, room.height());
            let end_x = next_room.x1 + (rng.random_range(1, next_room.width()));
            let end_y = next_room.y1 + (rng.random_range(1, next_room.width()));
            map.add_corridor(Point::new(start_x, start_y), Point::new(end_x, end_y));
        }

       map 
    }

    fn add_subrects(&self, rect: Rect, rng: &mut StdRng, rects: &mut Vec<Rect>) {
        // Remove the last rect from the list
        if !rects.is_empty() {
            rects.remove(rects.len() - 1);
        }

        // Calculate boundaries
        let width  = rect.x2 - rect.x1;
        let height = rect.y2 - rect.y1;
        let half_width = width / 2;
        let half_height = height / 2;

        let split = rng.roll_dice(1, 4);

        if split <= 2 {
            // Horizontal split
            let h1 = Rect::new( rect.x1, rect.y1, half_width-1, height );
            rects.push( h1 );
            if half_width > self.min_room_size { self.add_subrects(h1, rng, rects); }
            let h2 = Rect::new( rect.x1 + half_width, rect.y1, half_width, height );
            rects.push( h2 );
            if half_width > self.min_room_size { self.add_subrects(h2, rng, rects); }
        } else {
            // Vertical split
            let v1 = Rect::new( rect.x1, rect.y1, width, half_height-1 );
            rects.push(v1);
            if half_height > self.min_room_size { self.add_subrects(v1, rng, rects); }
            let v2 = Rect::new( rect.x1, rect.y1 + half_height, width, half_height );
            rects.push(v2);
            if half_height > self.min_room_size { self.add_subrects(v2, rng, rects); }
        }
    }
}