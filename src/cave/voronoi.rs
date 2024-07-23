//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{CaveMap, MapFilter};
//! use mapgen::cave::VoronoiHive;
//!
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = VoronoiHive::new();
//! let map = gen.modify_map(&mut rng, &CaveMap::new(80, 50));
//!
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//!

use crate::MapFilter;
use crate::{geometry::Vec2u, random::Rng};
use rand::prelude::*;

use super::CaveMap;

pub struct VoronoiHive {
    n_seeds: u32,
}

impl MapFilter for VoronoiHive {
    fn modify_map(&self, rng: &mut StdRng, map: &CaveMap) -> CaveMap {
        self.build(rng, map)
    }
}

impl VoronoiHive {
    pub fn new() -> Box<VoronoiHive> {
        Box::new(VoronoiHive { n_seeds: 64 })
    }

    fn build(&self, rng: &mut StdRng, map: &CaveMap) -> CaveMap {
        let mut new_map = map.clone();
        let seeds = self.generate_seeds(rng, map.width, map.height);

        let mut voronoi_distance = vec![(0, 0.0f32); self.n_seeds as usize];
        let mut voronoi_membership: Vec<i32> = vec![0; (map.width * map.height) as usize];
        for (i, vid) in voronoi_membership.iter_mut().enumerate() {
            for (seed, pos) in seeds.iter().enumerate() {
                let distance = pos.distance_to(&map.idx_point(i));
                voronoi_distance[seed] = (seed, distance);
            }

            voronoi_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            *vid = voronoi_distance[0].0 as i32;
        }

        for y in 1..new_map.height - 1 {
            for x in 1..new_map.width - 1 {
                let mut neighbors = 0;
                let my_idx = new_map.xy_idx(x, y);
                let my_seed = voronoi_membership[my_idx];
                if voronoi_membership[new_map.xy_idx(x - 1, y)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[new_map.xy_idx(x + 1, y)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[new_map.xy_idx(x, y - 1)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[new_map.xy_idx(x, y + 1)] != my_seed {
                    neighbors += 1;
                }

                if neighbors < 2 {
                    new_map.set_walkable(x, y, true);
                }
            }
        }

        new_map
    }

    /// Generate random seeds
    fn generate_seeds(&self, rng: &mut StdRng, width: u32, height: u32) -> Vec<Vec2u> {
        let mut seeds: Vec<Vec2u> = Vec::new();

        while (seeds.len() as u32) < self.n_seeds {
            let vx = rng.roll_dice(1, width - 1);
            let vy = rng.roll_dice(1, height - 1);
            let candidate = Vec2u::new(vx, vy);
            if !seeds.contains(&candidate) {
                seeds.push(candidate);
            }
        }

        seeds
    }
}
