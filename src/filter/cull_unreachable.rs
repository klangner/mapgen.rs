//! Remove unreachable areas from the map.
//! 
//! This modifier reiquires starting position on the map.
//! It will add wall on every tile which is not accessible from the starting point.
//! 

use rand::prelude::StdRng;
use crate::MapFilter;
use crate::{MapInfo, Tile};
use crate::dijkstra::DijkstraMap;


/// Remove unreachable areas from the map.
pub struct CullUnreachable {}

impl MapFilter for CullUnreachable {
    fn modify_map(&self, _: &mut StdRng, map: &MapInfo)  -> MapInfo {
        self.build(map)
    }
}

impl CullUnreachable {
    #[allow(dead_code)]
    pub fn new() -> Box<CullUnreachable> {
        Box::new(CullUnreachable{})
    }

    fn build(&self, map: &MapInfo) -> MapInfo {
        let mut new_map = map.clone();

        let dijkstra_map = DijkstraMap::new(map);
        for (i, tile) in new_map.tiles.iter_mut().enumerate() {
            if tile.is_walkable() {
                let distance_to_start = dijkstra_map.tiles[i];
                // We can't get to this tile - so we'll make it a wall
                if distance_to_start == std::f32::MAX {
                    *tile = Tile::wall();
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
    use crate::geometry::Point;
    use crate::map_info::MapInfo;

    #[test]
    fn test_culling() {
        let map_str = "
        ##########
        #  #     #
        ##########
        ";
        let mut map = MapInfo::from_string(map_str);
        map.starting_point = Some(Point::new(9, 1));
        let expected_map_str = "
        ##########
        ####     #
        ##########
        ";
        let expected_map = MapInfo::from_string(expected_map_str);


        let modifier = CullUnreachable::new();
        let mut rng = StdRng::seed_from_u64(0);
        let new_map = modifier.modify_map(&mut rng, &map);

        assert_eq!(new_map.tiles, expected_map.tiles);
    }
}