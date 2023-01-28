//! MapInfo structure contains information about elements of the map.
//! Those elements are used by MapFilters to generate map in several steps.
//! E.g. Most MapFilters will only update few MapInfo elements (like which cell is walkable) and some
//! other will depend on provided data (like adding exit point)
//! 
//! This structure is not intented to be your map in the game. 
//! The MapBuilder builds from this data the Map structure which is more suites for it.
//! 

use std::fmt;
use super::geometry::{Point, Rect, usize_abs};


#[derive(PartialEq, Copy, Clone)]
pub enum Symmetry { None, Horizontal, Vertical, Both }


/// Map data
#[derive(Default, Clone)]
pub struct MapBuffer {
    // Defines tiles which are walkable by player
    pub walkables : Vec<bool>,
    // Tile type is mostly defined for visual purposes. It could be specifi wall
    // type, or edge of the lake etc
    pub tile_types : Vec<usize>,
    pub width : usize,
    pub height : usize,
    pub starting_point: Option<Point>,
    pub exit_point: Option<Point>,
    pub rooms: Vec<Rect>,
    pub corridors: Vec<Vec<Point>>,
}

impl MapBuffer {

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: usize, height: usize) -> MapBuffer {
        let map_tile_count = width*height;
        MapBuffer{
            walkables : vec![false; map_tile_count],
            tile_types : vec![0; map_tile_count],
            width,
            height,
            starting_point: None,
            exit_point: None,
            rooms: Vec::new(),
            corridors: Vec::new()
        }
    }

    /// Create map from given string
    #[allow(clippy::needless_range_loop)]
    pub fn from_string(map_string: &str) -> MapBuffer {
        let lines: Vec<&str> = map_string.split('\n')
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();
        let cols = lines.iter().map(|l| l.len()).max().get_or_insert(1).to_owned();
        let rows = lines.len();
        let mut map = MapBuffer::new(cols, rows);

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
 
    // Check if given tile can be accessed
    // fn is_exit_valid(&self, x:usize, y:usize) -> bool {
    //     !self.at(x, y).is_blocked
    // }

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
    
    pub fn idx_point(&self, idx: usize) -> Point {
        Point {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    /// Create room on the map at given location
    /// Room is created by setting all tiles in the room to the Floor
    pub fn add_room(&mut self, rect: Rect) {
        for x in rect.x1..rect.x2 {
            for y in rect.y1..rect.y2 {
                self.set_walkable(x as usize, y as usize, true);
            }
        }
        self.rooms.push(rect);
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

            if self.is_blocked(x, y) {
                corridor.push(Point::new(x, y));
                self.set_walkable(x, y, true);
            }
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
    fn test_exists() {
        let map_str = "
        ##########
        #        #
        #        #
        ##########
        ";
        let map = MapBuffer::from_string(map_str);
        let exists = map.get_available_exits(1, 1);
        let expected_exists = vec![(2, 1, 1.0), (1, 2, 1.0), (2, 2, 1.45)];
        assert_eq!(exists, expected_exists);
    }

    #[test]
    fn test_create_room() {
        let mut map = MapBuffer::new(5, 5);
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

    #[test]
    fn test_add_corridor() {
        let map_str = "
        ##########
        #    #   #
        ##########
        ";
        let mut map = MapBuffer::from_string(map_str);
        let expected_map_str = "
        ##########
        #        #
        ##########
        ";
        let expected_map = MapBuffer::from_string(expected_map_str);

        map.add_corridor(Point::new(1, 1), Point::new(8, 1));

        assert_eq!(map.walkables, expected_map.walkables);
    }


    #[test]
    fn test_available_exists() {
        let map_str = "
         #########
        #    #   #
        ##########
        ";
        let map = MapBuffer::from_string(map_str);
        let exists = map.get_available_exits(0, 0);

        assert_eq!(exists.len(), 1);
    }

    #[test]
    fn convert_xy_idx() {
        let x = 64;
        let y = 45;

        let map = MapBuffer::new(65, 65);

        let idx = map.xy_idx(x, y);

        let Point { x: x2, y: y2 } = map.idx_point(idx);

        assert_eq!(x, x2);
        assert_eq!(y, y2);
    }
}