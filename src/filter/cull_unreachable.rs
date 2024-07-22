//! Remove unreachable areas from the map.
//! 
//! This modifier reiquires starting position on the map.
//! It will add wall on every tile which is not accessible from the starting point.
//! 

use rand::prelude::StdRng;
use crate::geometry::Vec2u;
use crate::path::DijkstraMap;
use crate::MapFilter;
use crate::MapBuffer;


/// Remove unreachable areas from the map.
pub struct CullUnreachable {}

impl MapFilter for CullUnreachable {
    fn modify_map(&self, _: &mut StdRng, map: &MapBuffer)  -> MapBuffer {
        self.build(map)
    }
}

impl CullUnreachable {
    #[allow(dead_code)]
    pub fn new() -> Box<CullUnreachable> {
        Box::new(CullUnreachable{})
    }

    fn build(&self, map: &MapBuffer) -> MapBuffer {
        let mut new_map = map.clone();

        let starting_point = map.starting_point.unwrap_or(Vec2u::default());
        let dijkstra_map = DijkstraMap::new(&map.walkable_layer, &starting_point);
        for i in 0..new_map.walkable_layer.walkables.len() {
            if new_map.walkable_layer.walkables[i] {
                let distance_to_start = dijkstra_map.tiles[i];
                // We can't get to this tile - so we'll make it a wall
                if distance_to_start == std::f32::MAX {
                    new_map.walkable_layer.walkables[i] = false;
                }
            }
        }
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
    fn test_culling() {
        let map_str = "
        ##########
        #  #     #
        ##########
        ";
        let mut map = MapBuffer::from_string(map_str);
        map.starting_point = Some(Vec2u::new(9, 1));
        let expected_map_str = "
        ##########
        ####     #
        ##########
        ";
        let expected_map = MapBuffer::from_string(expected_map_str);


        let modifier = CullUnreachable::new();
        let mut rng = StdRng::seed_from_u64(0);
        let new_map = modifier.modify_map(&mut rng, &map);

        assert_eq!(new_map.walkable_layer, expected_map.walkable_layer);
    }
}