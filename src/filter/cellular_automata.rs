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
//! use mapgen::{Map, MapFilter, NoData};
//! use mapgen::filter::CellularAutomata;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = CellularAutomata::<NoData>::new();
//! let map = gen.modify_map(&mut rng, &Map::new(80, 50));
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use std::marker::PhantomData;

use rand::prelude::*;
use crate::BuilderData;
use crate::MapFilter;
use crate::{Map, Tile};


/// Map filter
pub struct CellularAutomata<D: BuilderData> {
    num_iteraction: u32,
    phantom: PhantomData<D>,
}

impl<D: BuilderData> MapFilter<D> for CellularAutomata<D> {
    fn modify_map(&self, _rng: &mut StdRng, map: &Map<D>)  -> Map<D> {
        self.build(map)
    }
}

impl<D: BuilderData> CellularAutomata<D> {
    /// Create generator which will create map with the given dimension.
    pub fn new() -> Box<CellularAutomata<D>> {
        Box::new(CellularAutomata {
            num_iteraction: 15,
            phantom: PhantomData,
        })
    }

    /// Generate map
    fn build(&self, map: &Map<D>) -> Map<D> {
        let mut new_map = map.clone();
        for _ in 0..self.num_iteraction {
            new_map = apply_iteration(&new_map);
        }

        new_map
    }

}

fn apply_iteration<D: BuilderData>(map: &Map<D>) -> Map<D> {
    let mut new_map = map.clone();

    for y in 1..map.height-1 {
        for x in 1..map.width-1 {
            let idxs = [
                (x-1, y-1), (x, y-1), (x+1, y-1), 
                (x-1, y), (x+1, y), 
                (x-1, y+1), (x, y+1), (x+1, y+1)];
            let neighbors = idxs.iter()
                .filter(|(x, y)| map.at(*x, *y).is_blocked())
                .count();
            
            if neighbors > 4 || neighbors == 0 {
                new_map.set_tile(x, y, Tile::wall())
            }
            else {
                new_map.set_tile(x, y, Tile::floor());
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
    use crate::map::NoData;

    use super::*;

    #[test]
    fn test_iteration_wal() {
        let map = Map::<NoData>::new(3, 3);
        let new_map = apply_iteration(&map);
        assert!(new_map.at(1, 1).is_blocked());
    }


    #[test]
    fn test_iteration_floor() {
        let mut map = Map::<NoData>::new(3, 3);
        for i in 0..3 {
            for j in 0..2 {
                map.set_tile(i, j, Tile::floor());
            }
        }
        let new_map = apply_iteration(&map);
        assert!(new_map.at(1, 1).is_walkable());
    }

}