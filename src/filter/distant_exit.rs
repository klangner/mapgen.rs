//! Add exit point to the map
//! 
//! This modifier will try to add exit point as far as possible from the starting point.
//! It means that starting point have to be set before this Modyfier will start.
//! 

use std::f32;
use rand::prelude::StdRng;
use crate::geometry::Vec2u;
use crate::MapFilter;
use crate::MapBuffer;
use crate::dijkstra::DijkstraMap;


/// Add exist position to the map based on the distance from the start point.
pub struct DistantExit {} 

impl MapFilter for DistantExit {
    fn modify_map(&self, _: &mut StdRng, map: &MapBuffer)  -> MapBuffer {
        self.build(map)
    }
}

impl DistantExit {
    #[allow(dead_code)]
    pub fn new() -> Box<DistantExit> {
        Box::new(DistantExit{})
    }

    fn build(&self, map: &MapBuffer) -> MapBuffer {
        let mut new_map = map.clone();

        let mut best_idx = 0;
        let mut best_value = 0.0;
        let starting_point = map.starting_point.unwrap_or(Vec2u::default());
        let dijkstra_map = DijkstraMap::new(&map.walkable_layer, &starting_point);
        for (i, &value) in dijkstra_map.tiles.iter().enumerate() {
            if value < f32::MAX && value > best_value {
                best_value = value;
                best_idx = i;
            } 
        }
        let x = best_idx % map.width;
        let y = best_idx / map.width;
        new_map.exit_point = Some(Vec2u::new(x, y));
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
    use super::MapFilter;
    use crate::geometry::Vec2u;

    #[test]
    fn test_exit() {
        let map_str = "
        ##########
        #        #
        #  #     #
        ##########
        ";
        let mut map = MapBuffer::from_string(map_str);
        map.starting_point = Some(Vec2u::new(9, 2));

        let modifier = DistantExit::new();
        let mut rng = StdRng::seed_from_u64(0);
        let new_map = modifier.modify_map(&mut rng, &map);

        assert_eq!(new_map.exit_point, Some(Vec2u::new(1, 2)));
    }
}