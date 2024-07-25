//! Random rooms map generator.
//!
//! Try to generate rooms of different size to fill the map area.
//! Rooms will not overlap.
//!
//! Example generator usage:
//! ```
//! use mapgen::rooms::BspInterior;
//! use fastrand::Rng;
//!
//! let mut rng = Rng::with_seed(100);
//! let bsp = BspInterior::default();
//! let map = bsp.generate(80, 50, &mut rng);
//!
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//!

use fastrand::Rng;

use crate::geometry::{Rect, Vec2u};

use super::RoomsMap;

pub struct BspInterior {
    min_room_size: u32,
}

impl BspInterior {
    pub fn new(min_room_size: u32) -> Self {
        Self { min_room_size }
    }

    pub fn generate(&self, map_width: u32, max_height: u32, rng: &mut Rng) -> RoomsMap {
        // Create room with dimensions
        let mut map = RoomsMap::new(map_width, max_height);
        let mut rects = vec![Rect::new(1, 1, map.width - 2, map.height - 2)];
        let first_room = rects[0];
        // Divide the first room
        self.add_subrects(first_room, rng, &mut rects);

        let rooms_copy = rects.clone();
        for r in rooms_copy.iter() {
            let room = *r;
            map.add_room(room);
        }

        // Now we want corridors
        for i in 0..map.rooms.len() - 1 {
            let room = map.rooms[i];
            let next_room = map.rooms[i + 1];
            let start_x = rng.choice(room.x1..room.x2).unwrap();
            let start_y = rng.choice(room.y1..room.y2).unwrap();
            let end_x = rng.choice(next_room.x1..next_room.x2).unwrap();
            let end_y = rng.choice(next_room.y1..next_room.y2).unwrap();
            map.add_corridor(Vec2u::new(start_x, start_y), Vec2u::new(end_x, end_y));
        }

        map
    }

    fn add_subrects(&self, rect: Rect, rng: &mut Rng, rects: &mut Vec<Rect>) {
        // Remove the last rect from the list
        if !rects.is_empty() {
            rects.remove(rects.len() - 1);
        }

        // Calculate boundaries
        let width = rect.x2 - rect.x1;
        let height = rect.y2 - rect.y1;
        let half_width = width / 2;
        let half_height = height / 2;

        let split = rng.choice(1..5).unwrap();

        if split <= 2 {
            // Horizontal split
            let h1 = Rect::new(rect.x1, rect.y1, half_width - 1, height);
            rects.push(h1);
            if half_width > self.min_room_size {
                self.add_subrects(h1, rng, rects);
            }
            let h2 = Rect::new(rect.x1 + half_width, rect.y1, half_width, height);
            rects.push(h2);
            if half_width > self.min_room_size {
                self.add_subrects(h2, rng, rects);
            }
        } else {
            // Vertical split
            let v1 = Rect::new(rect.x1, rect.y1, width, half_height - 1);
            rects.push(v1);
            if half_height > self.min_room_size {
                self.add_subrects(v1, rng, rects);
            }
            let v2 = Rect::new(rect.x1, rect.y1 + half_height, width, half_height);
            rects.push(v2);
            if half_height > self.min_room_size {
                self.add_subrects(v2, rng, rects);
            }
        }
    }
}

impl Default for BspInterior {
    fn default() -> Self {
        Self { min_room_size: 8 }
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
        let bsp = BspInterior::default();
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
