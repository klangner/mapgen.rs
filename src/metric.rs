//! Different metrics for the map
//! Can be used to meausre the quality of the map or the quality of the generator.
//! To meause the quality of the generator; generate lots of maps, measure them
//! and the provide generator score as an average.
//!

use crate::{geometry::Vec2u, layer::WalkableLayer, path::DijkstraMap};

/// This metric calculates the percentage of walkable cells (Floor).
/// If this number is very low (like < 10%) then it means that the map
/// is probably to degenerated and shouldn't be used
pub fn density(walkable_layer: &WalkableLayer) -> f32 {
    let floor_count = walkable_layer.walkables.iter().filter(|&&x| x).count();
    floor_count as f32 / walkable_layer.walkables.len() as f32
}

/// Calculate the length of the shortes path from the starting point
/// to the exit.
/// If this path is very short, then the map is probably degenerated.
pub fn path_length(map: &WalkableLayer, starting_point: &Vec2u, exit_point: &Vec2u) -> f32 {
    let dijkstra = DijkstraMap::new(map, starting_point);
    dijkstra.tiles[map.xy_idx(exit_point.x, exit_point.y)]
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vec2u;

    #[test]
    fn test_density_no_floor() {
        let layer = WalkableLayer::new(10, 10);
        let score = density(&layer);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_density() {
        let map_str = "
            ##########
            #   ##   #
            ##########
            ";
        let layer = WalkableLayer::from_string(map_str);
        let score = density(&layer);
        assert_eq!(score, 0.2);
    }

    #[test]
    fn test_path_length() {
        let map_str = "
            ##########
            #   ##   #
            #        #
            ##########
            ";
        let map = WalkableLayer::from_string(map_str);
        let starting_point = Vec2u::new(1, 1);
        let exit_point = Vec2u::new(8, 1);

        let score = path_length(&map, &starting_point, &exit_point);
        assert!(f32::abs(score - 7.9) <= 0.01);
    }
}
