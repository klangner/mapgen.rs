//! Different metrics for the map
//! Can be used to meausre the quality of the map or the quality of the generator.
//! To meause the quality of the generator; generate lots of maps, measure them
//! and the provide generator score as an average.
//! 

use super::BuilderData;
use super::map::Map;
use super::dijkstra::DijkstraMap;


/// This metric calculates the percentage of walkable cells (Floor).
/// If this number is very low (like < 10%) then it means that the map 
/// is probably to degenerated and shouldn't be used
pub fn density<D>(map: &Map<D>) -> f32 {
    let floor_count = map.tiles.iter()
        .filter(|&t| t.is_walkable())
        .count();
    floor_count as f32 / map.tiles.len() as f32
}


/// Calculate the length of the shortes path from the starting point
/// to the exit.
/// If this path is very short, then the map is probably degenerated.
pub fn path_length<D: BuilderData>(map: &Map<D>) -> f32 {
    if map.starting_point.is_none() {
        return 0.0
    }
    
    match map.exit_point {
        None => 0.0,
        Some(exit) => {
            let dijkstra = DijkstraMap::new(map);
            dijkstra.tiles[map.xy_idx(exit.x, exit.y)]
        },
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{geometry::Point, map::NoData};


    #[test]
    fn test_density_no_floor() {
        let map = Map::<NoData>::new(10, 10);
        let score = density(&map);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_density() {
        let map_str = "
            ##########
            #   ##   #
            ##########
            ";
        let map = Map::<NoData>::from_string(map_str);
        let score = density(&map);
        assert_eq!(score, 0.2);
    }

    #[test]
    fn test_no_path() {
        let map_str = "
            ##########
            #   ##   #
            ##########
            ";
        let map = Map::<NoData>::from_string(map_str);
        let score = path_length(&map);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_path_length() {
        let map_str = "
            ##########
            #   ##   #
            #        #
            ##########
            ";
        let mut map = Map::<NoData>::from_string(map_str);
        map.starting_point = Some(Point::new(1,1));
        map.exit_point = Some(Point::new(8,1));

        let score = path_length(&map);
        assert!(f32::abs(score - 7.9) <= 0.01);
    }
}