//! Add starting point to the map
//! 
//! This modifier will try to add starting point by finding the floor title closes 
//! to the given point.
//! 
//! Example modifier usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{MapFilter, MapBuffer};
//! use mapgen::filter::starting_point::{AreaStartingPosition, XStart, YStart};
//! use mapgen::geometry::Vec2u;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let mut map = MapBuffer::new(80, 50);
//! map.set_walkable(10, 10, true);
//! let modifier = AreaStartingPosition::new(XStart::LEFT, YStart::TOP);
//! let new_map = modifier.modify_map(&mut rng, &map);
//! 
//! assert_eq!(new_map.starting_point, Some(Vec2u::new(10, 10)));
//! ```
//! 

use rand::prelude::StdRng;
use crate::MapFilter;
use crate::geometry::Vec2u;
use crate::MapBuffer;


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
    fn modify_map(&self, _: &mut StdRng, map: &MapBuffer)  -> MapBuffer {
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

    fn build(&self, map : &MapBuffer) -> MapBuffer {
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
        for (idx, &w) in map.walkable_layer.walkables.iter().enumerate() {
            if w {
                available_floors.push(
                    (
                        idx,
                        Vec2u::new(idx % map.width, idx / map.width)
                            .distance_to(&Vec2u::new(seed_x, seed_y))
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
        new_map.starting_point = Some(Vec2u::new(start_x, start_y));
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
        #   ##   #
        #  # #   #
        ##########
        ";
        let mut map = MapBuffer::from_string(map_str);
        map.starting_point = Some(Vec2u::new(9, 2));

        let modifier = AreaStartingPosition::new(XStart::CENTER, YStart::TOP);
        let mut rng = StdRng::seed_from_u64(0);
        let new_map = modifier.modify_map(&mut rng, &map);

        assert_eq!(new_map.starting_point, Some(Vec2u::new(6, 1)));
    }
}