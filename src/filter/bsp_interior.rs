//! Random rooms map generator.
//! 
//! Try to generate rooms of different size to fill the map area. 
//! Rooms will not overlap.
//! 
//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{Map, MapFilter};
//! use mapgen::filter::{
//!     BspInterior
//! };
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = BspInterior::new();
//! let map = gen.modify_map(&mut rng, &Map::new(80, 50));
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use crate::MapFilter;
use crate::geometry::{Point, Rect};
use crate::random::Rng;
use crate::Map;


pub struct BspInterior {
    min_room_size: usize,
}

impl MapFilter for BspInterior {
    fn modify_map(&self, rng: &mut StdRng, map: &Map) -> Map {
        self.build(rng, map)
    }
}

impl BspInterior {

    pub fn new() -> Box<BspInterior> {
        Box::new(BspInterior{
            min_room_size: 8,
        })
    }

    fn build(&self, rng: &mut StdRng, map: &Map) -> Map {
        let mut new_map = map.clone();
        let mut rects: Vec<Rect> = Vec::new();
        rects.push( Rect::new(1, 1, new_map.width-2, new_map.height-2) ); 
        let first_room = rects[0];
        // Divide the first room
        self.add_subrects(first_room, rng, &mut rects); 

        let rooms_copy = rects.clone();
        for r in rooms_copy.iter() {
            let room = *r;
            new_map.add_room(room);
        }

        // Now we want corridors
        for i in 0..new_map.rooms.len()-1 {
            let room = new_map.rooms[i];
            let next_room = new_map.rooms[i+1];
            let start_x = room.x1 + rng.random_range(1, room.width());
            let start_y = room.y1 + rng.random_range(1, room.height());
            let end_x = next_room.x1 + (rng.random_range(1, next_room.width()));
            let end_y = next_room.y1 + (rng.random_range(1, next_room.width()));
            new_map.add_corridor(Point::new(start_x, start_y), Point::new(end_x, end_y));
        }

       new_map 
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


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TileType, Map};

    #[test]
    fn no_corridors_on_borders() {
         let mut rng = StdRng::seed_from_u64(907647352);
        let gen = BspInterior::new();
        let map = gen.modify_map(&mut rng, &Map::new(80, 50));
        for i in 0..80 {
            assert_eq!(map.at(i, 0), TileType::Wall);
            assert_eq!(map.at(i, 49), TileType::Wall);
        } 
        for j in 0..50 {
            assert_eq!(map.at(0, j), TileType::Wall);
            assert_eq!(map.at(79, j), TileType::Wall);
        } 
    }

}