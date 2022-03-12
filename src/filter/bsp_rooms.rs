//! Random rooms map generator.
//! 
//! Try to generate rooms of different size to fill the map area. 
//! Rooms will not overlap.
//! 
//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{Map, MapFilter, NoData};
//! use mapgen::filter::BspRooms;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = BspRooms::<NoData>::new();
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


pub struct BspRooms<D: BuilderData> {
    max_split: usize,
    phantom: PhantomData<D>,
}

impl<D: BuilderData> MapFilter<D> for BspRooms<D> {
    fn modify_map(&self, rng: &mut StdRng, map: &Map<D>)  -> Map<D> {
        self.build_rooms(map, rng)
    }
}

impl<D: BuilderData> BspRooms<D> {
    pub fn new() -> Box<BspRooms<D>> {
        Box::new(BspRooms {
            max_split: 240,
            phantom: PhantomData,
        })
    }

    fn build_rooms(&self, map: &Map<D>, rng : &mut StdRng) -> Map<D> {
        let mut new_map = map.clone();
        let mut rects: Vec<Rect> = Vec::new();
        // Start with a single map-sized rectangle
        rects.push( Rect::new(2, 2, new_map.width-5, new_map.height-5) ); 
        let first_room = rects[0];
        rects.append(&mut self.split_into_subrects(first_room)); // Divide the first room

        // Up to max_split times, we get a random rectangle and divide it. If its possible to squeeze a
        // room in there, we place it and add it to the rooms list.
        for _ in 0..self.max_split {
            let rect = self.get_random_rect(rng, &rects);
            let candidate = self.get_random_sub_rect(rect, rng);

            if self.is_possible(candidate, &new_map) {
                new_map.add_room(candidate);
                rects.append(&mut self.split_into_subrects(rect));
            }
        }

        new_map
    }

    fn split_into_subrects(&self, rect: Rect) -> Vec<Rect> {
        let mut rects: Vec<Rect> = Vec::new();
        let width = rect.width();
        let height = rect.height();
        let half_width = usize::max(width / 2, 1);
        let half_height = usize::max(height / 2, 1);

        rects.push(Rect::new( rect.x1, rect.y1, half_width, half_height ));
        rects.push(Rect::new( rect.x1, rect.y1 + half_height, half_width, half_height ));
        rects.push(Rect::new( rect.x1 + half_width, rect.y1, half_width, half_height ));
        rects.push(Rect::new( rect.x1 + half_width, rect.y1 + half_height, half_width, half_height ));

        rects
    }

    fn get_random_rect(&self, rng : &mut StdRng, rects: &Vec<Rect>) -> Rect {
        if rects.len() == 1 { return rects[0]; }
        let idx = rng.random_range(0, rects.len());
        rects[idx]
    }

    fn get_random_sub_rect(&self, rect: Rect, rng: &mut StdRng) -> Rect {
        let mut result = rect;
        let rect_width = rect.width();
        let rect_height = rect.height();

        let w = usize::max(3, rng.random_range(1, usize::min(rect_width as usize, 20))) + 1;
        let h = usize::max(3, rng.random_range(1, usize::min(rect_height as usize, 20))) + 1;

        result.x1 += rng.random_range(0, 6);
        result.y1 += rng.random_range(0, 6);
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    fn is_possible(&self, rect: Rect, map: &Map<D>) -> bool {
        let mut expanded = rect;
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y2 += 2;

        let mut can_build = true;

        for r in map.rooms.iter() {
            if r.intersect(&rect) { can_build = false; }
        }

        for y in expanded.y1 ..= expanded.y2 {
            for x in expanded.x1 ..= expanded.x2 {
                if x > map.width - 2 { can_build = false; }
                if y > map.height - 2 { can_build = false; }
                if x < 1 { can_build = false; }
                if y < 1 { can_build = false; }
                if can_build {
                    if map.at(x as usize, y as usize).is_walkable() {
                        can_build = false;
                    }
                }
            }
        }

        can_build
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{Map, NoData};

    #[test]
    fn no_corridors_on_borders() {
         let mut rng = StdRng::seed_from_u64(907647352);
        let gen = BspRooms::<NoData>::new();
        let map = gen.modify_map(&mut rng, &Map::new(80, 50));
        for i in 0..80 {
            assert!(map.at(i, 0).is_blocked());
            assert!(map.at(i, 49).is_blocked());
        } 
        for j in 0..50 {
            assert!(map.at(0, j).is_blocked());
            assert!(map.at(79, j).is_blocked());
        } 
    }

}