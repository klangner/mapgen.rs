//! Find exit point to the map
//!
//! This module will try to add exit point as far as possible from the starting point.
//!

use glam::UVec2;

use crate::layer::WalkableLayer;
use crate::path::DijkstraMap;
use std::f32;

pub struct DistantExit;

impl DistantExit {
    pub fn find(starting_point: &UVec2, map: &WalkableLayer) -> UVec2 {
        let mut best_idx = 0;
        let mut best_value = 0.0;
        let dijkstra_map = DijkstraMap::new(map, starting_point);
        for (i, &value) in dijkstra_map.tiles.iter().enumerate() {
            if value < f32::MAX && value > best_value {
                best_value = value;
                best_idx = i;
            }
        }
        map.idx_point(best_idx)
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use glam::UVec2;

    use crate::{layer::WalkableLayer, poi::DistantExit};

    #[test]
    fn test_exit() {
        let map_str = "
        ##########
        #        #
        #  #     #
        ##########
        ";
        let map = WalkableLayer::from_string(map_str);
        let starting_point = UVec2::new(9, 2);
        let exit_point = DistantExit::find(&starting_point, &map);

        assert_eq!(exit_point, UVec2::new(1, 2));
    }
}
