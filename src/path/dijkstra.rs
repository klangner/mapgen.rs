//! Calculate Dijkstra influence map
//!
//! http://www.roguebasin.com/index.php?title=The_Incredible_Power_of_Dijkstra_Maps
//!
//! This algorithm calculates cost (distance) of moving from the given starting point
//! to the each point on the map. Point which are not reachable will get f32::MAX value.
//!
//! Example generator usage:
//! ---
//! use rand::prelude::*;
//! use crate::common::geometry::Point;
//! use mapgen::dungeon::{
//!     MapModifier,
//!     map::{Map, TileType},
//!     starting_point::{AreaStartingPosition, XStart, YStart}
//! };
//!
//! let mut rng = StdRng::seed_from_u64(100);
//! let mut map = Map::new(80, 50);
//! map.set_tile(10, 10, TileType::Floor);
//! let modifier = AreaStartingPosition::new(XStart::LEFT, YStart::TOP);
//! let new_map = modifier.modify_map(&mut rng, &map);
//!
//! assert_eq!(new_map.starting_point, Some(Point::new(10, 10)));
//! ---
//!

use std::collections::VecDeque;

use crate::geometry::Vec2u;
use crate::layer::WalkableLayer;

/// Representation of a Dijkstra flow map.
/// map is a vector of floats, having a size equal to size_x * size_y (one per tile).
/// size_x and size_y are stored for overflow avoidance.
/// max_depth is the maximum number of iterations this search shall support.
pub struct DijkstraMap {
    pub tiles: Vec<f32>,
    size_x: u32,
    size_y: u32,
    max_depth: f32,
}

impl DijkstraMap {
    //! Construct a new Dijkstra map, ready to run.
    pub fn new(map: &WalkableLayer, starting_point: &Vec2u) -> DijkstraMap {
        let len = (map.width * map.height) as usize;
        let tiles = vec![f32::MAX; len];
        let mut d = DijkstraMap {
            tiles,
            size_x: map.width,
            size_y: map.height,
            max_depth: len as f32,
        };
        d.build(map, starting_point);
        d
    }

    /// Builds the Dijkstra map: iterate from each starting point, to each exit provided by BaseMap's
    /// exits implementation. Each step adds cost to the current depth, and is discarded if the new
    /// depth is further than the current depth.
    /// WARNING: Will give incorrect results when used with non-uniform exit costs. Much slower
    /// algorithm required to support that.
    fn build(&mut self, map: &WalkableLayer, starting_point: &Vec2u) {
        let mapsize = self.size_x * self.size_y;
        let mut open_list: VecDeque<((u32, u32), f32)> = VecDeque::with_capacity(mapsize as usize);

        open_list.push_back(((starting_point.x, starting_point.y), 0.0));
        let idx = self.xy_idx(starting_point.x, starting_point.y);
        self.tiles[idx] = 0.0;

        while let Some(((x, y), depth)) = open_list.pop_front() {
            let exits = map.get_available_exits(x, y);
            for (x, y, add_depth) in exits {
                let idx = self.xy_idx(x, y);
                let new_depth = depth + add_depth;
                let prev_depth = self.tiles[idx];
                if new_depth >= prev_depth {
                    continue;
                }
                if new_depth >= self.max_depth {
                    continue;
                }
                self.tiles[idx] = new_depth;
                open_list.push_back(((x, y), new_depth));
            }
        }
    }

    fn xy_idx(&self, x: u32, y: u32) -> usize {
        (y * self.size_x + x) as usize
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vec2u;

    #[test]
    fn test_culling() {
        let map_str = "
        ##########
        # #      #
        ##########
        ";
        let map = WalkableLayer::from_string(map_str);
        let dm = DijkstraMap::new(&map, &Vec2u::new(8, 1));

        println!(
            "{:?}",
            &dm.tiles
                .iter()
                .map(|&v| if v == f32::MAX { 9.0 } else { v })
                .collect::<Vec<f32>>()
        );

        assert_eq!(dm.size_x, 10);
        assert_eq!(dm.size_y, 3);
        for i in 0..10 {
            assert_eq!(dm.tiles[i], f32::MAX);
            assert_eq!(dm.tiles[2 * dm.size_x as usize + i], f32::MAX);
            let idx = dm.size_x as usize + i;
            if i < 3 || i == 9 {
                assert_eq!(dm.tiles[idx], f32::MAX);
            } else {
                assert_eq!(dm.tiles[idx], (8 - i) as f32);
            }
        }
    }

    #[test]
    fn test_2() {
        let map_str = "
        ####
        #  #
        #  #
        ####
        ";
        let map = WalkableLayer::from_string(map_str);
        let starting_point = Vec2u::new(2, 2);
        let dm = DijkstraMap::new(&map, &starting_point);
        let expected = [
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            1.45,
            1.0,
            f32::MAX,
            f32::MAX,
            1.0,
            0.0,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
        ];

        assert_eq!(dm.tiles, expected);
    }

    #[test]
    fn test_3() {
        let map_str = "
        ##########
        #        #
        #  #     #
        ##########
        ";
        let map = WalkableLayer::from_string(map_str);
        let starting_point = Vec2u::new(8, 2);
        let dm = DijkstraMap::new(&map, &starting_point);
        let expected = [
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            7.45,
            6.45,
            5.45,
            4.45,
            3.45,
            2.45,
            1.45,
            1.0,
            f32::MAX,
            f32::MAX,
            7.9,
            6.9,
            f32::MAX,
            4.0,
            3.0,
            2.0,
            1.0,
            0.0,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
            f32::MAX,
        ];

        for (v, e) in dm.tiles.iter().zip(expected.iter()) {
            assert!(f32::abs(v - e) <= 0.01);
        }
    }
}
