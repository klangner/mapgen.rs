//! Add exit point to the map
//! 
//! This modifier will try to add exit point as far as possible from the starting point.
//! It means that starting point have to be set before this Modyfier will start.
//! 

use std::f32;
use rand::prelude::StdRng;
use super::{MapModifier};
use super::map::{Map, Point};
use super::dijkstra::DijkstraMap;


/// Add exist position to the map based on the distance from the start point.
pub struct DistantExit {} 

impl MapModifier for DistantExit {
    fn modify_map(&self, _: &mut StdRng, map: &Map)  -> Map {
        self.build(map)
    }
}

impl DistantExit {
    #[allow(dead_code)]
    pub fn new() -> Box<DistantExit> {
        Box::new(DistantExit{})
    }

    fn build(&self, map: &Map) -> Map {
        let mut new_map = map.clone();

        let mut best_idx = 0;
        let mut best_value = 0.0;
        let dijkstra_map = DijkstraMap::new(map);
        for (i, &value) in dijkstra_map.tiles.iter().enumerate() {
            if value < f32::MAX && value > best_value {
                best_value = value;
                best_idx = i;
            } 
        }
        let x = best_idx % map.width;
        let y = best_idx / map.width;
        new_map.exit_point = Some(Point::new(x, y));
        new_map
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use super::*;
    use super::MapModifier;
    use crate::dungeon::map::{Point, Map};

    #[test]
    fn test_exit() {
        let map_str = "
        ##########
        #        #
        #  #     #
        ##########
        ";
        let mut map = Map::from_string(map_str);
        map.starting_point = Some(Point::new(9, 2));

        let modifier = DistantExit::new();
        let mut rng = StdRng::seed_from_u64(0);
        let new_map = modifier.modify_map(&mut rng, &map);

        assert_eq!(new_map.exit_point, Some(Point::new(1, 2)));
    }
}