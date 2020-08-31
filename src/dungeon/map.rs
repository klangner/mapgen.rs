#[derive(PartialEq, Copy, Clone, Eq, Hash, Debug)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default, Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub width : usize,
    pub height : usize,
}

impl Map {

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: usize, height: usize) -> Map {
        let map_tile_count = width*height;
        Map{
            tiles : vec![TileType::Wall; map_tile_count],
            width,
            height
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
    fn test_new_map() {
        let map = Map::new(10, 10);
        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(map.at(i, j), TileType::Wall);
            }
        }
    }
}