//! Remove unreachable areas from the map.
//! 
//! This modifier reiquires starting position on the map.
//! It will add wall on every tile which is not accessible from the starting point.
//! 

use std::marker::PhantomData;

use rand::prelude::StdRng;
use crate::MapFilter;
use crate::{BuilderData, Map, Tile};
use crate::dijkstra::DijkstraMap;


/// Remove unreachable areas from the map.
pub struct CullUnreachable<D: BuilderData> {
    phantom: PhantomData<D>,
}

impl<D: BuilderData> MapFilter<D> for CullUnreachable<D> {
    fn modify_map(&self, _: &mut StdRng, map: &Map<D>)  -> Map<D> {
        self.build(map)
    }
}

impl<D: BuilderData> CullUnreachable<D> {
    #[allow(dead_code)]
    pub fn new() -> Box<CullUnreachable<D>> {
        Box::new(CullUnreachable {
            phantom: PhantomData,
        })
    }

    fn build(&self, map: &Map<D>) -> Map<D> {
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
    use crate::map::{Map, NoData};

    #[test]
    fn test_culling() {
        let map_str = "
        ##########
        #  #     #
        ##########
        ";
        let mut map = Map::<NoData>::from_string(map_str);
        map.starting_point = Some(Point::new(9, 1));
        let expected_map_str = "
        ##########
        ####     #
        ##########
        ";
        let expected_map = Map::<NoData>::from_string(expected_map_str);


        let modifier = CullUnreachable::new();
        let mut rng = StdRng::seed_from_u64(0);
        let new_map = modifier.modify_map(&mut rng, &map);

        assert_eq!(new_map.tiles, expected_map.tiles);
    }
}