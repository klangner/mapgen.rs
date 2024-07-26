//! Generators for dungeon type maps.
//!

use std::fmt;

use glam::UVec2;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct WalkableLayer {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<bool>,
}

pub struct DataLayer<T> {
    pub width: u32,
    pub height: u32,
    pub walkables: Vec<T>,
}

impl WalkableLayer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![false; (width * height) as usize],
        }
    }

    pub fn is_walkable(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            let idx = self.xy_idx(x, y);
            self.tiles[idx]
        }
    }

    pub fn is_blocked(&self, x: u32, y: u32) -> bool {
        !self.is_walkable(x, y)
    }

    /// Modify tile at the given location
    pub fn set_walkable(&mut self, x: u32, y: u32, set: bool) {
        if x < self.width && y < self.height {
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = set;
        }
    }

    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    /// Create layer from given string
    #[allow(clippy::needless_range_loop)]
    pub fn from_string(map_string: &str) -> Self {
        let lines: Vec<&str> = map_string
            .split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();
        let cols = lines
            .iter()
            .map(|l| l.len() as u32)
            .max()
            .get_or_insert(1)
            .to_owned();
        let rows = lines.len() as u32;
        let mut map = Self::new(cols, rows);

        for i in 0..rows as usize {
            let line = lines[i].as_bytes();
            for j in 0..line.len() {
                if line[j] as char == ' ' {
                    map.set_walkable(j as u32, i as u32, true);
                }
            }
        }
        map
    }

    pub fn idx_point(&self, idx: usize) -> UVec2 {
        UVec2 {
            x: idx as u32 % self.width,
            y: idx as u32 / self.width,
        }
    }

    /// Get available exists from the given tile
    pub fn get_available_exits(&self, x: u32, y: u32) -> Vec<(u32, u32, f32)> {
        let mut exits = Vec::new();

        // Cardinal directions
        if x > 0 && self.is_walkable(x - 1, y) {
            exits.push((x - 1, y, 1.0))
        };
        if self.is_walkable(x + 1, y) {
            exits.push((x + 1, y, 1.0))
        };
        if y > 0 && self.is_walkable(x, y - 1) {
            exits.push((x, y - 1, 1.0))
        };
        if self.is_walkable(x, y + 1) {
            exits.push((x, y + 1, 1.0))
        };

        // Diagonals
        if x > 0 && y > 0 && self.is_walkable(x - 1, y - 1) {
            exits.push((x - 1, y - 1, 1.45));
        }
        if y > 0 && self.is_walkable(x + 1, y - 1) {
            exits.push((x + 1, y - 1, 1.45));
        }
        if x > 0 && self.is_walkable(x - 1, y + 1) {
            exits.push((x - 1, y + 1, 1.45));
        }
        if self.is_walkable(x + 1, y + 1) {
            exits.push((x + 1, y + 1, 1.45));
        }

        exits
    }
}

impl<T: Clone> DataLayer<T> {
    pub fn new(width: u32, height: u32, default: T) -> Self {
        Self {
            width,
            height,
            walkables: vec![default; (width * height) as usize],
        }
    }
}

impl fmt::Display for WalkableLayer {
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
    fn test_new_layer() {
        let map = WalkableLayer::new(10, 10);
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
        let map = WalkableLayer::from_string(map_str);

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
    fn test_exists() {
        let map_str = "
        ##########
        #        #
        #        #
        ##########
        ";
        let map = WalkableLayer::from_string(map_str);
        let exists = map.get_available_exits(1, 1);
        let expected_exists = vec![(2, 1, 1.0), (1, 2, 1.0), (2, 2, 1.45)];
        assert_eq!(exists, expected_exists);
    }

    #[test]
    fn test_available_exists() {
        let map_str = "
         #########
        #    #   #
        ##########
        ";
        let map = WalkableLayer::from_string(map_str);
        let exists = map.get_available_exits(0, 0);

        assert_eq!(exists.len(), 1);
    }
}
