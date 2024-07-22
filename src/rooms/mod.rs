//! Generators for dungeon type maps.
//! 

pub mod bsp_interior;
pub mod bsp_rooms;
pub mod simple_rooms;
pub mod rooms_corridors_nearest;

use std::fmt;

pub use bsp_interior::BspInterior;
pub use bsp_rooms::BspRooms;
pub use simple_rooms::SimpleRooms;
pub use rooms_corridors_nearest::NearestCorridors;

use crate::geometry::{Point, Rect};


#[derive(Default, Clone)]
pub struct RoomBasedMap {
    pub width: usize,
    pub height: usize,
    pub walkables: Vec<bool>,
    pub rooms: Vec<Rect>,
    pub corridors: Vec<Vec<Point>>,
}

impl RoomBasedMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            walkables : vec![false; width*height],
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

    pub fn add_corridor(&mut self, from: Point, to:Point) {
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

            corridor.push(Point::new(x, y));
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            let idx = (y as usize) * self.width + (x as usize);
            self.walkables[idx]
        }
    }

    pub fn is_blocked(&self, x: usize, y: usize) -> bool {
        !self.is_walkable(x, y)
    }

    /// Modify tile at the given location
    pub fn set_walkable(&mut self, x: usize, y: usize, set: bool) {
        if x < self.width && y < self.height {
            let idx = self.xy_idx(x as usize, y as usize);
            self.walkables[idx] = set;
        }
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x        
    }
    
}

impl fmt::Display for RoomBasedMap {
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
        let map = RoomBasedMap::new(10, 10);
        for i in 0..10 {
            for j in 0..10 {
                assert!(map.is_blocked(i, j));
            }
        }
    }

    #[test]
    fn test_create_room() {
        let mut map = RoomBasedMap::new(5, 5);
        map.add_room(Rect::new(1, 1, 3, 3));
        for x in 0..map.width {
            for y in 0..map.height {
                if x == 0 || y == 0 || x == 4 || y == 4 {
                    assert!(map.is_blocked(x, y));
                } else {
                    assert!(map.is_blocked(x, y) == false);
                }
            }
        }
    }
}