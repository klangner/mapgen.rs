//! Add starting point to the map
//! 
//! This modifier will try to add starting point by finding the floor title closes 
//! to the given point.
//! 
//! Example modifier usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{MapFilter, Map, TileType};
//! use mapgen::filter::starting_point::{AreaStartingPosition, XStart, YStart};
//! use mapgen::geometry::Point;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let mut map = Map::new(80, 50);
//! map.set_tile(10, 10, TileType::Floor);
//! let modifier = AreaStartingPosition::new(XStart::LEFT, YStart::TOP);
//! let new_map = modifier.modify_map(&mut rng, &map);
//! 
//! assert_eq!(new_map.starting_point, Some(Point::new(10, 10)));
//! ```
//! 

use rand::prelude::StdRng;
use crate::MapFilter;
use crate::geometry::Point;
use crate::{Map, TileType};


/// Initial x region position
pub enum XStart { LEFT, CENTER, RIGHT }

/// Initial y region position
pub enum YStart { TOP, CENTER, BOTTOM }

/// Add starting position to the map
pub struct AreaStartingPosition {
    x : XStart, 
    y : YStart
}

impl MapFilter for AreaStartingPosition {
    fn modify_map(&self, _: &mut StdRng, map: &Map)  -> Map {
        self.build(map)
    }
}

impl AreaStartingPosition {
    /// Create new modifier with given region
    pub fn new(x : XStart, y : YStart) -> Box<AreaStartingPosition> {
        Box::new(AreaStartingPosition{
            x, y
        })
    }

    fn build(&self, map : &Map) -> Map {
        let seed_x = match self.x {
            XStart::LEFT => 1,
            XStart::CENTER => map.width / 2,
            XStart::RIGHT => map.width - 2
        };

        let seed_y = match self.y {
            YStart::TOP => 1,
            YStart::CENTER => map.height / 2,
            YStart::BOTTOM => map.height - 2
        };

        let mut available_floors : Vec<(usize, f32)> = Vec::new();
        for (idx, tiletype) in map.tiles.iter().enumerate() {
            if *tiletype == TileType::Floor {
                available_floors.push(
                    (
                        idx,
                        Point::new(idx % map.width, idx / map.width)
                            .distance_to(&Point::new(seed_x, seed_y))
                    )
                );
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

        let start_x = available_floors[0].0 % map.width;
        let start_y = available_floors[0].0 / map.width;

        let mut new_map = map.clone();
        new_map.starting_point = Some(Point::new(start_x, start_y));
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
    use crate::map::Map;

    #[test]
    fn test_exit() {
        let map_str = "
        ##########
        #   ##   #
        #  # #   #
        ##########
        ";
        let mut map = Map::from_string(map_str);
        map.starting_point = Some(Point::new(9, 2));

        let modifier = AreaStartingPosition::new(XStart::CENTER, YStart::TOP);
        let mut rng = StdRng::seed_from_u64(0);
        let new_map = modifier.modify_map(&mut rng, &map);

        assert_eq!(new_map.starting_point, Some(Point::new(6, 1)));
    }
}