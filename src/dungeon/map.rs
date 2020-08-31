#[derive(PartialEq, Copy, Clone, Eq, Hash, Debug)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default, Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub width : i32,
    pub height : i32,
}

impl Map {

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: i32, height: i32) -> Map {
        let map_tile_count = (width*height) as usize;
        Map{
            tiles : vec![TileType::Wall; map_tile_count],
            width,
            height
        }
    }

    pub fn at(&self, x: i32, y: i32) -> TileType {
        let idx = (y as usize * self.width as usize) + x as usize;
        self.tiles[idx]
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