//! Random rooms map generator.
//!
//! Try to generate rooms of different size to fill the map area.
//! Rooms will not overlap.
//!
//! Example generator usage:
//! ```
//! use mapgen::rooms::BspRooms;
//! use fastrand::Rng;
//!
//! let mut rng = Rng::with_seed(100);
//! let bsp = BspRooms::default();
//! let map = bsp.generate(80, 50, &mut rng);
//!
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//!

use fastrand::Rng;
use crate::geometry::Rect;
use super::RoomsMap;

pub struct BspRooms {
    max_split: u32,
}

impl BspRooms {
    pub fn new(max_split: u32) -> Self {
        Self { max_split }
    }

    pub fn generate(&self, map_width: u32, max_height: u32, rng: &mut Rng) -> RoomsMap {
        let mut map = RoomsMap::new(map_width, max_height);

        // Start with a single map-sized rectangle
        let mut rects = vec![Rect::new(2, 2, map.width - 5, map.height - 5)];
        let first_room = rects[0];
        rects.append(&mut self.split_into_subrects(first_room)); // Divide the first room

        // Up to max_split times, we get a random rectangle and divide it. If its possible to squeeze a
        // room in there, we place it and add it to the rooms list.
        for _ in 0..self.max_split {
            let rect = self.get_random_rect(rng, &rects);
            let candidate = self.get_random_sub_rect(rect, rng);

            if self.is_possible(candidate, &map) {
                map.add_room(candidate);
                rects.append(&mut self.split_into_subrects(rect));
            }
        }

        map
    }

    fn split_into_subrects(&self, rect: Rect) -> Vec<Rect> {
        let mut rects: Vec<Rect> = Vec::new();
        let width = rect.width();
        let height = rect.height();
        let half_width = u32::max(width / 2, 1);
        let half_height = u32::max(height / 2, 1);

        rects.push(Rect::new(rect.x1, rect.y1, half_width, half_height));
        rects.push(Rect::new(
            rect.x1,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
        rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1,
            half_width,
            half_height,
        ));
        rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));

        rects
    }

    fn get_random_rect(&self, rng: &mut Rng, rects: &[Rect]) -> Rect {
        if rects.len() == 1 {
            return rects[0];
        }
        let idx = rng.choice(0..rects.len()).unwrap();
        rects[idx]
    }

    fn get_random_sub_rect(&self, rect: Rect, rng: &mut Rng) -> Rect {
        let mut result = rect;
        let rect_width = rect.width();
        let rect_height = rect.height();

        let w = u32::max(3, rng.choice(1..u32::min(rect_width, 20)).unwrap_or(1)) + 1;
        let h = u32::max(3, rng.choice(1..u32::min(rect_height, 20)).unwrap_or(1)) + 1;

        result.x1 += rng.u32(0..6);
        result.y1 += rng.u32(0..6);
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    fn is_possible(&self, rect: Rect, map: &RoomsMap) -> bool {
        let mut expanded = rect;
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y2 += 2;

        let mut can_build = true;

        for r in map.rooms.iter() {
            if r.intersect(&rect) {
                can_build = false;
            }
        }

        for y in expanded.y1..=expanded.y2 {
            for x in expanded.x1..=expanded.x2 {
                if x > map.width - 2 {
                    can_build = false;
                }
                if y > map.height - 2 {
                    can_build = false;
                }
                if x < 1 {
                    can_build = false;
                }
                if y < 1 {
                    can_build = false;
                }
                if can_build && map.is_walkable(x, y) {
                    can_build = false;
                }
            }
        }

        can_build
    }
}

impl Default for BspRooms {
    fn default() -> Self {
        Self { max_split: 240 }
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_corridors_on_borders() {
        let mut rng = Rng::with_seed(907647352);
        let bsp = BspRooms::default();
        let map = bsp.generate(80, 50, &mut rng);
        for i in 0..80 {
            assert!(map.is_blocked(i, 0));
            assert!(map.is_blocked(i, 49));
        }
        for j in 0..50 {
            assert!(map.is_blocked(0, j));
            assert!(map.is_blocked(79, j));
        }
    }
}
