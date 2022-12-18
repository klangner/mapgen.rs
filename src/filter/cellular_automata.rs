//! Cellular automata map filter.
//! 
//! Check this [article](http://www.roguebasin.com/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels)
//! for more information about the algorithm behind this generator.
//! 
//! This algorithm requires that map first is filtered with some noise.
//! For example `UniformNoise`. It can also be apply to any other non empty map.
//! 
//! Example usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{MapBuffer, MapFilter};
//! use mapgen::filter::CellularAutomata;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = CellularAutomata::new();
//! let map = gen.modify_map(&mut rng, &MapBuffer::new(80, 50));
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use crate::MapFilter;
use crate::MapBuffer;


/// Map filter
pub struct CellularAutomata {
    num_iteraction: u32,
}

impl MapFilter for CellularAutomata {
    fn modify_map(&self, _rng: &mut StdRng, map: &MapBuffer)  -> MapBuffer {
        self.build(map)
    }
}

impl CellularAutomata {
    /// Create generator which will create map with the given dimension.
    pub fn new() -> Box<CellularAutomata> {
        Box::new(CellularAutomata { num_iteraction: 15})
    }

    /// Generate map
    fn build(&self, map: &MapBuffer) -> MapBuffer {
        let mut new_map = map.clone();
        for _ in 0..self.num_iteraction {
            new_map = apply_iteration(&new_map);
        }

        new_map
    }

}

fn apply_iteration(map: &MapBuffer) -> MapBuffer {
    let mut new_map = map.clone();

    for y in 1..map.height-1 {
        for x in 1..map.width-1 {
            let idxs = [
                (x-1, y-1), (x, y-1), (x+1, y-1), 
                (x-1, y), (x+1, y), 
                (x-1, y+1), (x, y+1), (x+1, y+1)];
            let neighbors = idxs.iter()
                .filter(|(x, y)| map.is_blocked(*x, *y))
                .count();
            
            let walkable = neighbors < 5 && neighbors > 0;
            new_map.set_walkable(x, y, walkable);
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
        let map = MapBuffer::new(3, 3);
        let new_map = apply_iteration(&map);
        assert!(new_map.is_blocked(1, 1));
    }


    #[test]
    fn test_iteration_floor() {
        let mut map = MapBuffer::new(3, 3);
        for i in 0..3 {
            for j in 0..2 {
                map.set_walkable(i, j, true);
            }
        }
        let new_map = apply_iteration(&map);
        assert!(new_map.is_walkable(1, 1));
    }

}