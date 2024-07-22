//! Generators for dungeon type maps.
//! 

use std::fmt;


#[derive(Default, Debug, Clone, PartialEq)]
pub struct WalkableLayer {
    pub width: usize,
    pub height: usize,
    pub walkables: Vec<bool>,
}

pub struct DataLayer<T> {
    pub width: usize,
    pub height: usize,
    pub walkables: Vec<T>,
}

impl WalkableLayer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            walkables : vec![false; width*height],
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
    
    /// Create layer from given string
    #[allow(clippy::needless_range_loop)]
    pub fn from_string(map_string: &str) -> Self {
        let lines: Vec<&str> = map_string.split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();
        let cols = lines.iter().map(|l| l.len()).max().get_or_insert(1).to_owned();
        let rows = lines.len();
        let mut map = Self::new(cols, rows);

        for i in 0..rows {
            let line = lines[i].as_bytes();
            for j in 0..line.len() {
                if line[j] as char == ' ' {
                    map.set_walkable(j, i, true);
                }
            }
        }
        map
    }

    /// Get available exists from the given tile
    pub fn get_available_exits(&self, x: usize, y: usize) -> Vec<(usize, usize, f32)> {
        let mut exits = Vec::new();

        // Cardinal directions
        if x > 0 && self.is_walkable(x-1, y) { exits.push((x-1, y, 1.0)) };
        if self.is_walkable(x+1, y) { exits.push((x+1, y, 1.0)) };
        if y > 0 && self.is_walkable(x, y-1) { exits.push((x, y-1, 1.0)) };
        if self.is_walkable(x, y+1) { exits.push((x, y+1, 1.0)) };

        // Diagonals
        if x > 0 && y > 0 && self.is_walkable(x-1, y-1) { exits.push((x-1, y-1, 1.45)); }
        if y > 0 && self.is_walkable(x+1, y-1) { exits.push((x+1, y-1, 1.45)); }
        if x > 0 && self.is_walkable(x-1, y+1) { exits.push((x-1, y+1, 1.45)); }
        if self.is_walkable(x+1, y+1) { exits.push((x+1, y+1, 1.45)); }

        exits
    }    
 
}

impl<T: Clone> DataLayer<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            width,
            height,
            walkables : vec![default; width*height],
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