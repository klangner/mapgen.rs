//! Find starting point on the map
//! 
//! Example modifier usage:
//! ```
//! use mapgen::{geometry::Vec2u, layer::WalkableLayer, poi::*};
//! 
//! let mut map = WalkableLayer::new(80, 50);
//! map.set_walkable(10, 10, true);
//! let point = AreaStartingPosition::find(XStart::LEFT, YStart::TOP, &map);
//! 
//! assert_eq!(point, Vec2u::new(10, 10));
//! ```
//! 

use crate::{geometry::Vec2u, layer::WalkableLayer};


/// Initial x region position
pub enum XStart { LEFT, CENTER, RIGHT }

/// Initial y region position
pub enum YStart { TOP, CENTER, BOTTOM }

/// Add starting position to the map
pub struct AreaStartingPosition;

impl AreaStartingPosition {
    /// Create new modifier with given region
    pub fn find(x_start : XStart, y_start : YStart, map: &WalkableLayer) -> Vec2u {
        let seed_x = match x_start {
            XStart::LEFT => 1,
            XStart::CENTER => map.width / 2,
            XStart::RIGHT => map.width - 2
        };

        let seed_y = match y_start {
            YStart::TOP => 1,
            YStart::CENTER => map.height / 2,
            YStart::BOTTOM => map.height - 2
        };

        let mut available_floors : Vec<(usize, f32)> = Vec::new();
        for (idx, &w) in map.walkables.iter().enumerate() {
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

        Vec2u::new(start_x, start_y)
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::{geometry::Vec2u, layer::WalkableLayer, poi::*};

    #[test]
    fn test_exit() {
        let map_str = "
        ##########
        #   ##   #
        #  # #   #
        ##########
        ";
        let map = WalkableLayer::from_string(map_str);
        let starting_point = AreaStartingPosition::find(XStart::CENTER, YStart::TOP, &map);

        assert_eq!(starting_point, Vec2u::new(6, 1));
    }
}