//! Cellular automata map generator and modifier.
//! 
//! Check this [article](http://www.roguebasin.com/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels)
//! for more information about the algorithm behind this generator.
//! 
//! Since this algorithm works in interations it is possible to take existing map 
//! and apply single interaction to it. This is the idea behind MapModifier implementation.
//! 
//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::dungeon::{
//!     MapGenerator,
//!     cellular_automata::CellularAutomataGen
//! };
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = CellularAutomataGen::new();
//! let map = gen.generate_map(80, 50, &mut rng);
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use super::{MapGenerator, MapModifier};
use super::map::{Map, TileType};


/// Map generator and modifier
pub struct CellularAutomataGen {}

impl MapGenerator for CellularAutomataGen {
    fn generate_map(&self, width: usize, height: usize, rng : &mut StdRng) -> Map {
        self.build(width, height, rng)
    }
}

impl CellularAutomataGen {
    /// Create generator which will create map with the given dimension.
    pub fn new() -> CellularAutomataGen {
        CellularAutomataGen {}
    }

    /// Generate map
    fn build(&self, width: usize, height: usize, rng: &mut StdRng) -> Map {
        let mut map = Map::new(width, height);
        // First we completely randomize the map, setting 55% of it to be floor.
        for y in 1..height-1 {
            for x in 1..width-1 {
                let roll = rng.next_u32() % 100;
                if roll > 55 { map.set_tile(x, y, TileType::Floor) } 
                else { map.set_tile(x, y, TileType::Wall) }
            }
        }

        // Now we iteratively apply cellular automata rules
        for _ in 0..15 {
            map = apply_iteration(&map);
        }

        map
    }

}

impl MapModifier for CellularAutomataGen {
    fn modify_map(&self, _rng: &mut StdRng, map : &Map) -> Map {
        apply_iteration(map)
    }
}


fn apply_iteration(map: &Map) -> Map {
    let mut new_map = map.clone();

    for y in 1..map.height-1 {
        for x in 1..map.width-1 {
            let idxs = [
                (x-1, y-1), (x, y-1), (x+1, y-1), 
                (x-1, y), (x+1, y), 
                (x-1, y+1), (x, y+1), (x+1, y+1)];
            let neighbors = idxs.iter()
                .filter(|(x, y)| map.at(*x, *y) == TileType::Wall)
                .count();
            
            if neighbors > 4 || neighbors == 0 {
                new_map.set_tile(x, y, TileType::Wall)
            }
            else {
                new_map.set_tile(x, y, TileType::Floor);
            }
        }
    }

    new_map
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iteration_wal() {
        let map = Map::new(3, 3);
        let new_map = apply_iteration(&map);
        assert_eq!(new_map.at(1, 1), TileType::Wall);
    }


    #[test]
    fn test_iteration_floor() {
        let mut map = Map::new(3, 3);
        for i in 0..3 {
            for j in 0..2 {
                map.set_tile(i, j, TileType::Floor);
            }
        }
        let new_map = apply_iteration(&map);
        assert_eq!(new_map.at(1, 1), TileType::Floor);
    }

}