//! WorldMap this is the output of the MapBuilder. 
//! This is optimized map which can't be futher modified by MapFilters.
//! 

use std::fmt;
use super::geometry::Point;


#[derive(Default, Clone)]
pub struct WorldMap {
    // Defines tiles which are walkable by player
    pub walkables : Vec<bool>,
    // Tile type is mostly defined for visual purposes. It could be specifi wall
    // type, or edge of the lake etc
    pub tile_types : Vec<usize>,
    pub width : usize,
    pub height : usize,
    pub starting_point: Option<Point>,
    pub exit_point: Option<Point>,
}

impl WorldMap {
    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: usize, height: usize, walkables: Vec<bool>, tile_types: Vec<usize>,
        starting_point: Option<Point>, exit_point: Option<Point>) -> WorldMap {
        WorldMap{
            walkables,
            tile_types,
            width,
            height,
            starting_point,
            exit_point,
        }
    }

    /// Get TileType at the given location
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

    // Get type type
    pub fn tile_type(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            0
        } else {
            let idx = (y as usize) * self.width + (x as usize);
            self.tile_types[idx]
        }
    }

    /// Modify tile at the given location
    pub fn set_tile(&mut self, x: usize, y: usize, id: usize) {
        if x < self.width && y < self.height {
            let idx = self.xy_idx(x as usize, y as usize);
            self.tile_types[idx] = id;
        }
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x        
    }
}


impl fmt::Display for WorldMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            let bytes: Vec<u8> = (0..self.width)
                .map(|x| if self.is_blocked(x, y) {'#'} else {' '} as u8)
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

}