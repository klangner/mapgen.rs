//! Remove unreachable areas from the map.
//! It will add wall on every tile which is not accessible from the starting point.
//!

use glam::UVec2;

use crate::layer::WalkableLayer;
use crate::path::DijkstraMap;

pub struct CullUnreachable;

impl CullUnreachable {
    #[allow(dead_code)]
    pub fn new() -> Box<CullUnreachable> {
        Box::new(CullUnreachable {})
    }

    pub fn remove_walkable_tiles(starting_point: &UVec2, map: &WalkableLayer) -> WalkableLayer {
        let mut new_map = map.clone();

        let dijkstra_map = DijkstraMap::new(map, starting_point);
        for i in 0..new_map.tiles.len() {
            if new_map.tiles[i] {
                let distance_to_start = dijkstra_map.tiles[i];
                // We can't get to this tile - so we'll make it a wall
                if distance_to_start == f32::MAX {
                    new_map.tiles[i] = false;
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
    use glam::UVec2;

    use crate::{layer::WalkableLayer, poi::CullUnreachable};

    #[test]
    fn test_culling() {
        let map_str = "
        ##########
        #  #     #
        ##########
        ";
        let map = WalkableLayer::from_string(map_str);
        let starting_point = UVec2::new(9, 1);
        let expected_map_str = "
        ##########
        ####     #
        ##########
        ";
        let expected_map = WalkableLayer::from_string(expected_map_str);

        let new_map = CullUnreachable::remove_walkable_tiles(&starting_point, &map);

        assert_eq!(new_map, expected_map);
    }
}
