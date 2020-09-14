//! Map structure contains information about tiles and other elements on the map.
//! 
//! Map is created with generators and then can by modified with MapModifiers.
//! 
//! This structure is not intented to be your map in the game (But can be used as one). 
//! Rather the information from this map will be copied to the structures required by
//! specific game.
//! 

use std::fmt;
use crate::common::geometry::{Point, Rect};


#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub enum TileType {
    Wall, Floor
}

/// Map data
#[derive(Default, Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub width : usize,
    pub height : usize,
    pub starting_point: Option<Point>,
    pub exit_point: Option<Point>
}

impl Map {

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: usize, height: usize) -> Map {
        let map_tile_count = width*height;
        Map{
            tiles : vec![TileType::Wall; map_tile_count],
            width,
            height,
            starting_point: None,
            exit_point: None,
        }
    }

    /// Create map from given string
    pub fn from_string(map_string: &str) -> Map {
        let lines: Vec<&str> = map_string.split("\n")
            .map(|l| l.trim())
            .filter(|l| l.len() > 0)
            .collect();
        let cols = lines.iter().map(|l| l.len()).max().get_or_insert(1).to_owned();
        let rows = lines.len();
        let mut map = Map::new(cols, rows);

        for i in 0..rows {
            let line = lines[i].as_bytes();
            for j in 0..line.len() {
                if line[j] as char == ' ' {
                    map.set_tile(j, i, TileType::Floor);
                }
            }
        }
        map
    }

    /// Get TileType at the given location
    pub fn at(&self, x: usize, y: usize) -> TileType {
        if x >= self.width || y >= self.height {
            TileType::Wall
        } else {
            let idx = (y as usize) * self.width + (x as usize);
            self.tiles[idx]
        }
    }

    /// Get available exists from the given tile
    pub fn get_available_exits(&self, x: usize, y: usize) -> Vec<(usize, usize, f32)> {
        let mut exits = Vec::new();

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((x-1, y, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((x+1, y, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((x, y-1, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((x, y+1, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push((x-1, y-1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push((x+1, y-1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push((x-1, y+1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push((x+1, y+1, 1.45)); }

        exits
    }    
 
    // Check if given tile can be accessed
    fn is_exit_valid(&self, x:usize, y:usize) -> bool {
        self.at(x, y) == TileType::Floor
    }

    /// Modify tile at the given location
    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * self.width + (x as usize);
            self.tiles[idx] = tile;
        }
    }
    
    /// Create room on the map at given location
    /// Room is created by setting all tiles in the room to the Floor
    pub fn create_room(&mut self, rect: &Rect) {
        for x in rect.x1..rect.x2 {
            for y in rect.y1..rect.y2 {
                self.set_tile(x as usize, y as usize, TileType::Floor);
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            let bytes: Vec<u8> = (0..self.width)
                .map(|x| if self.at(x, y) == TileType::Wall {'#'} else {' '} as u8)
                .collect();
            let line = String::from_utf8(bytes).expect("Can't convert map to string");
            let _ = write!(f, "{}\n", line);
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
        let map = Map::new(10, 10);
        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(map.at(i, j), TileType::Wall);
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
        let map = Map::from_string(map_str);

        assert_eq!(map.width, 10);
        assert_eq!(map.height, 3);
        for i in 0..10 {
            assert_eq!(map.at(i, 0), TileType::Wall);
            assert_eq!(map.at(i, 2), TileType::Wall);
            if i == 0 || i == 9 {
                assert_eq!(map.at(i, 1), TileType::Wall);
            } else {
                assert_eq!(map.at(i, 1), TileType::Floor);
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
        let map = Map::from_string(map_str);
        let exists = map.get_available_exits(1, 1);
        let expected_exists = vec![(2, 1, 1.0), (1, 2, 1.0), (2, 2, 1.45)];
        assert_eq!(exists, expected_exists);
    }

        #[test]
    fn test_create_room() {
        let mut map = Map::new(5, 5);
        map.create_room(&Rect::new(1, 1, 3, 3));
        for x in 0..map.width {
            for y in 0..map.height {
                if x == 0 || y == 0 || x == 4 || y == 4 {
                    assert_eq!(map.at(x, y), TileType::Wall);
                } else {
                    assert_eq!(map.at(x, y), TileType::Floor);
                }
            }
        }
    }

}