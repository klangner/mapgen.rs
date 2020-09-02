//! Map structure contains information about tiles and other elements on the map.
//! 
//! Map is created with generators and then can by modified with MapModifiers.
//! 
//! This structure is not intented to be your map in the game (But can be used as one). 
//! Rather the information from this map will be copied to the structures required by
//! specific game.
//! 

/// Position on the map
#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize
}

impl Point {
    /// Create new point
    pub fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }

    /// Euclidean distance to a given point
    pub fn distance_to(self, point: &Point) -> f32 {
        let a = (self.x as f32 - point.x as f32).powf(2.0);
        let b = (self.y as f32 - point.y as f32).powf(2.0);
        (a + b).sqrt()
    }
}

/// Possible tile type on the map
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

    /// Get TileType at the given location
    pub fn at(&self, x: usize, y: usize) -> TileType {
        let idx = y * self.width + x;
        self.tiles[idx]
    }

    /// Modify tile at the given location
    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        let idx = y * self.width + x;
        self.tiles[idx] = tile;
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 7);
        let distance = p1.distance_to(&p2);
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_new_map() {
        let map = Map::new(10, 10);
        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(map.at(i, j), TileType::Wall);
            }
        }
    }
}