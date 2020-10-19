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
use std::f32::MAX;
use super::map::Map;


/// Representation of a Dijkstra flow map.
/// map is a vector of floats, having a size equal to size_x * size_y (one per tile).
/// size_x and size_y are stored for overflow avoidance.
/// max_depth is the maximum number of iterations this search shall support.
pub struct DijkstraMap {
    pub tiles: Vec<f32>,
    size_x: usize,
    size_y: usize,
    max_depth: f32,
}

impl DijkstraMap {
    /// Construct a new Dijkstra map, ready to run. You must specify the map size, and link to an implementation
    /// of a BaseMap trait that can generate exits lists. It then builds the map, giving you a result.
    pub fn new(map: &Map) -> DijkstraMap {
        let len =  map.width * map.height;
        let tiles = vec![MAX; len];
        let mut d = DijkstraMap {
            tiles: tiles,
            size_x: map.width,
            size_y: map.height,
            max_depth: len as f32,
        };
        d.build(map);
        d
    }

    /// Builds the Dijkstra map: iterate from each starting point, to each exit provided by BaseMap's
    /// exits implementation. Each step adds cost to the current depth, and is discarded if the new
    /// depth is further than the current depth.
    /// WARNING: Will give incorrect results when used with non-uniform exit costs. Much slower
    /// algorithm required to support that.
    fn build(&mut self, map: &Map) {
        let mapsize: usize = (self.size_x * self.size_y) as usize;
        let mut open_list: VecDeque<((usize, usize), f32)> = VecDeque::with_capacity(mapsize);

        if let Some(pos) = map.starting_point {
            open_list.push_back(((pos.x, pos.y), 0.0));
            let idx = self.xy_idx(pos.x, pos.y);
            self.tiles[idx] = 0.0;
        }

        while let Some(((x, y), depth)) = open_list.pop_front() {
            let exits = map.get_available_exits(x, y);
            for (x, y, add_depth) in exits {
                let idx = self.xy_idx(x, y);
                let new_depth = depth + add_depth;
                let prev_depth = self.tiles[idx];
                if new_depth >= prev_depth { continue; }
                if new_depth >= self.max_depth { continue; }
                self.tiles[idx] = new_depth;
                open_list.push_back(((x, y), new_depth));
            }
        }
    }

    fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y * self.size_x ) + x 
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::map::Map;

    #[test]
    fn test_culling() {
        let map_str = "
        ##########
        # #      #
        ##########
        ";
        let mut map = Map::from_string(map_str);
        map.starting_point = Some(Point::new(8, 1));
        let dm = DijkstraMap::new(&map);

        println!("{:?}", &dm.tiles.iter().map(|&v| if v == f32::MAX {9.0} else {v}).collect::<Vec<f32>>());

        assert_eq!(dm.size_x, 10);
        assert_eq!(dm.size_y, 3);
        for i in 0..10 {
            assert_eq!(dm.tiles[i], MAX);
            assert_eq!(dm.tiles[2*dm.size_x + i], MAX);
            let idx = dm.size_x + i;
            if i < 3 || i == 9 {
                assert_eq!(dm.tiles[idx], MAX);
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
        let mut map = Map::from_string(map_str);
        map.starting_point = Some(Point::new(2, 2));
        let dm = DijkstraMap::new(&map);
        let expected = [MAX, MAX, MAX, MAX,
                        MAX, 1.45, 1.0, MAX, 
                        MAX, 1.0, 0.0, MAX, 
                        MAX, MAX, MAX, MAX];

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
        let mut map = Map::from_string(map_str);
        map.starting_point = Some(Point::new(8, 2));
        let dm = DijkstraMap::new(&map);
        let expected = [MAX, MAX, MAX, MAX, MAX, MAX, MAX, MAX, MAX, MAX, 
                        MAX, 7.45, 6.45, 5.45, 4.45, 3.45, 2.45, 1.45, 1.0, MAX, 
                        MAX, 7.9, 6.9, MAX, 4.0, 3.0, 2.0, 1.0, 0.0, MAX, 
                        MAX, MAX, MAX, MAX, MAX, MAX, MAX, MAX, MAX, MAX];

        for (v, e) in dm.tiles.iter().zip(expected.iter()) {
            assert!(f32::abs(v - e) <= 0.01);
        }
    }
}