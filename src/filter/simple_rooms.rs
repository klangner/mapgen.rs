//! Random rooms map generator.
//! 
//! Try to generate rooms of different size to fill the map area. 
//! Rooms will not overlap.
//! 
//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{MapFilter, Map, NoData};
//! use mapgen::filter::SimpleRooms;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = SimpleRooms::<NoData>::new();
//! let map = gen.modify_map(&mut rng, &Map::new(80, 50));
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use std::marker::PhantomData;

use rand::prelude::*;
use crate::BuilderData;
use crate::MapFilter;
use crate::geometry::Rect;
use crate::random::Rng;
use crate::Map;


pub struct SimpleRooms<D: BuilderData> {
    max_rooms: usize,
    min_room_size: usize,
    max_room_size: usize,
    phantom: PhantomData<D>,
}

impl<D: BuilderData> MapFilter<D> for SimpleRooms<D> {
    fn modify_map(&self, rng: &mut StdRng, map: &Map<D>)  -> Map<D> {
        self.build_rooms(map, rng)
    }
}


impl<D: BuilderData> SimpleRooms<D> {
    pub fn new() -> Box<SimpleRooms<D>> {
        Box::new(SimpleRooms{
            max_rooms: 30,
            min_room_size: 6,
            max_room_size: 10,
            phantom: PhantomData,
        })
    }

    fn build_rooms(&self, map: &Map<D>, rng : &mut StdRng) -> Map<D> {
        let mut new_map = map.clone();

        // Create room dimensions
        for _ in 0..self.max_rooms {
            let w = rng.random_range(self.min_room_size, self.max_room_size);
            let h = rng.random_range(self.min_room_size, self.max_room_size);
            let x = rng.random_range(1, new_map.width - w);
            let y = rng.random_range(1, new_map.height - h);
            let new_room = Rect::new(x, y, w, h);
            let intersects = new_map.rooms.iter().any(|r| new_room.intersect(r));
            if !intersects {
                new_map.add_room(new_room);
            }
        }
        
        new_map
    }
}