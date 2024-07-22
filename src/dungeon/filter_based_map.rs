//! MapInfo structure contains information about elements of the map.
//! Those elements are used by MapFilters to generate map in several steps.
//! E.g. Most MapFilters will only update few MapInfo elements (like which cell is walkable) and some
//! other will depend on provided data (like adding exit point)
//! 
//! This structure is not intented to be your map in the game. 
//! The MapBuilder builds from this data the Map structure which is more suites for it.
//! 

use std::fmt;
use crate::{geometry::{usize_abs, Vec2u}, layer::WalkableLayer};


#[derive(PartialEq, Copy, Clone)]
pub enum Symmetry { None, Horizontal, Vertical, Both }


/// Map data
#[derive(Default, Clone)]
pub struct MapBuffer {
    pub walkable_layer: WalkableLayer,
    pub width : usize,
    pub height : usize,
    pub starting_point: Option<Vec2u>,
    pub exit_point: Option<Vec2u>,
}

impl MapBuffer {

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: usize, height: usize) -> MapBuffer {
        MapBuffer{
            walkable_layer : WalkableLayer::new(width, height),
            width,
            height,
            starting_point: None,
            exit_point: None,
        }
    }

    /// Create map from given string
    pub fn from_string(map_string: &str) -> Self {
        let walkable_layer = WalkableLayer::from_string(map_string); 
        Self {
            width: walkable_layer.width,
            height: walkable_layer.height,
            walkable_layer,
            starting_point: None,
            exit_point: None,
        }
    }

    /// Get TileType at the given location
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.walkable_layer.is_walkable(x, y)
    }

    pub fn is_blocked(&self, x: usize, y: usize) -> bool {
        !self.is_walkable(x, y)
    }

    /// Modify tile at the given location
    pub fn set_walkable(&mut self, x: usize, y: usize, set: bool) {
        if x < self.width && y < self.height {
            self.walkable_layer.set_walkable(x, y, set);
        }
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x        
    }
    
    pub fn idx_point(&self, idx: usize) -> Vec2u {
        Vec2u {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    pub fn paint(&mut self, mode: Symmetry, brush_size: usize, x: usize, y: usize) {
        match mode {
            Symmetry::None => self.apply_paint(brush_size, x, y),
            Symmetry::Horizontal => {
                let center_x = self.width / 2;
                if x == center_x {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_x = usize_abs(center_x, x);
                    self.apply_paint(brush_size, center_x + dist_x, y);
                    self.apply_paint(brush_size, center_x - dist_x, y);
                }
            }
            Symmetry::Vertical => {
                let center_y = self.height / 2;
                if y == center_y {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_y = usize_abs(center_y, y);
                    self.apply_paint(brush_size, x, center_y + dist_y);
                    self.apply_paint(brush_size, x, center_y - dist_y);
                }
            }
            Symmetry::Both => {
                let center_x = self.width / 2;
                let center_y = self.height / 2;
                if x == center_x && y == center_y {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_x = usize_abs(center_x, x);
                    self.apply_paint(brush_size, center_x + dist_x, y);
                    self.apply_paint(brush_size, center_x - dist_x, y);
                    let dist_y = usize_abs(center_y, y);
                    self.apply_paint(brush_size, x, center_y + dist_y);
                    self.apply_paint(brush_size, x, center_y - dist_y);
                }
            }
        }
    }

    fn apply_paint(&mut self, brush_size: usize, x: usize, y: usize) {
        match brush_size {
            1 => {
                self.set_walkable(x, y, true);
            }
            _ => {
                let half_brush_size = brush_size / 2;
                for brush_y in y-half_brush_size .. y+half_brush_size {
                    for brush_x in x-half_brush_size .. x+half_brush_size {
                        if brush_x > 1 && brush_x < self.width-1 && brush_y > 1 && brush_y < self.height-1 {
                            self.set_walkable(brush_x, brush_y, true);
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for MapBuffer {
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
        let map = MapBuffer::new(10, 10);
        for i in 0..10 {
            for j in 0..10 {
                assert!(map.is_blocked(i, j));
            }
        }
    }

    #[test]
    fn test_from_string() {
        let map_str = "
        ##########
        #        #
        ##########
        ";
        let map = MapBuffer::from_string(map_str);

        assert_eq!(map.width, 10);
        assert_eq!(map.height, 3);
        for i in 0..10 {
            assert!(map.is_blocked(i, 0));
            assert!(map.is_blocked(i, 2));
            if i == 0 || i == 9 {
                assert!(map.is_blocked(i, 1));
            } else {
                assert!(map.is_walkable(i, 1));
            }
        }
    }

    #[test]
    fn convert_xy_idx() {
        let x = 64;
        let y = 45;

        let map = MapBuffer::new(65, 65);

        let idx = map.xy_idx(x, y);

        let Vec2u { x: x2, y: y2 } = map.idx_point(idx);

        assert_eq!(x, x2);
        assert_eq!(y, y2);
    }
}