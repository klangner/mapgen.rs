//! Generators for dungeon type maps.
//!

use std::fmt;

use crate::{
    geometry::{Rect, Vec2u},
    layer::WalkableLayer,
};

#[derive(Default, Clone)]
pub struct RoomsMap {
    pub width: u32,
    pub height: u32,
    pub rooms: Vec<Rect>,
    pub corridors: Vec<Vec<Vec2u>>,
    pub walkable_layer: WalkableLayer,
}

impl RoomsMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            walkable_layer: WalkableLayer::new(width, height),
            rooms: Vec::new(),
            corridors: Vec::new(),
        }
    }

    /// Create room on the map at given location
    /// Room is created by setting all tiles in the room to the Floor
    pub fn add_room(&mut self, rect: Rect) {
        self.rooms.push(rect);
        for x in rect.x1..rect.x2 {
            for y in rect.y1..rect.y2 {
                self.set_walkable(x, y, true);
            }
        }
    }

    pub fn add_corridor(&mut self, from: Vec2u, to: Vec2u) {
        let mut corridor = Vec::new();
        let mut x = from.x;
        let mut y = from.y;

        while x != to.x || y != to.y {
            if x < to.x {
                x += 1;
            } else if x > to.x {
                x -= 1;
            } else if y < to.y {
                y += 1;
            } else if y > to.y {
                y -= 1;
            }

            corridor.push(Vec2u::new(x, y));
            self.walkable_layer.set_walkable(x, y, true);
        }
    }

    pub fn is_walkable(&self, x: u32, y: u32) -> bool {
        self.walkable_layer.is_walkable(x, y)
    }

    pub fn is_blocked(&self, x: u32, y: u32) -> bool {
        !self.is_walkable(x, y)
    }

    /// Modify tile at the given location
    pub fn set_walkable(&mut self, x: u32, y: u32, set: bool) {
        self.walkable_layer.set_walkable(x, y, set);
    }

    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        self.walkable_layer.xy_idx(x, y)
    }
}

impl fmt::Display for RoomsMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            let bytes: Vec<u8> = (0..self.width)
                .map(|x| if self.is_blocked(x, y) { '#' } else { ' ' } as u8)
                .collect();
            let line = String::from_utf8(bytes).expect("Can't convert map to string");
            let _ = writeln!(f, "{}", line);
        }
        Ok(())
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_map() {
        let map = RoomsMap::new(10, 10);
        for i in 0..10 {
            for j in 0..10 {
                assert!(map.is_blocked(i, j));
            }
        }
    }

    #[test]
    fn test_create_room() {
        let mut map = RoomsMap::new(5, 5);
        map.add_room(Rect::new(1, 1, 3, 3));
        for x in 0..map.width {
            for y in 0..map.height {
                if x == 0 || y == 0 || x == 4 || y == 4 {
                    assert!(map.is_blocked(x, y));
                } else {
                    assert!(!map.is_blocked(x, y));
                }
            }
        }
    }
}
