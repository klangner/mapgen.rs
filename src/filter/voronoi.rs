//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{Map, MapFilter};
//! use mapgen::filter::VoronoiHive;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = VoronoiHive::new();
//! let map = gen.modify_map(&mut rng, &Map::new(80, 50));
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use crate::MapFilter;
use crate::{
    map::{Map, TileType},
    random::Rng,
    geometry::Point,
};


pub struct VoronoiHive {
    n_seeds: usize,
}


impl MapFilter for VoronoiHive {
    fn modify_map(&self, rng: &mut StdRng, map: &Map)  -> Map {
        self.build(rng, map)
    }
}

impl VoronoiHive {
    pub fn new() -> Box<VoronoiHive> {
        Box::new(VoronoiHive{
            n_seeds: 64,
        })
    }

    fn build(&self, rng: &mut StdRng, map: &Map) -> Map {
        let mut new_map = map.clone();
        let seeds = self.generate_seeds(rng, map.width, map.height);

        let mut voronoi_distance = vec![(0, 0.0f32) ; self.n_seeds];
        let mut voronoi_membership : Vec<i32> = vec![0 ; map.width as usize * map.height as usize];
        for (i, vid) in voronoi_membership.iter_mut().enumerate() {
            let x = i % map.width;
            let y = i / map.width;

            for (seed, pos) in seeds.iter().enumerate() {
                let distance = pos.distance_to(&Point::new(x, y));
                voronoi_distance[seed] = (seed, distance);
            }

            voronoi_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

            *vid = voronoi_distance[0].0 as i32;
        }

        for y in 1..new_map.height-1 {
            for x in 1..new_map.width-1 {
                let mut neighbors = 0;
                let my_idx = new_map.xy_idx(x, y);
                let my_seed = voronoi_membership[my_idx];
                if voronoi_membership[new_map.xy_idx(x-1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[new_map.xy_idx(x+1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[new_map.xy_idx(x, y-1)] != my_seed { neighbors += 1; }
                if voronoi_membership[new_map.xy_idx(x, y+1)] != my_seed { neighbors += 1; }

                if neighbors < 2 {
                    new_map.set_tile(x, y, TileType::Floor);
                }
            }
        }

        new_map
    }    
    
    /// Generate random seeds
    fn generate_seeds(&self, rng: &mut StdRng, width: usize, height: usize) -> Vec<Point> {
        let mut seeds: Vec<Point> = Vec::new();

        while seeds.len() < self.n_seeds {
            let vx = rng.roll_dice(1, width-1);
            let vy = rng.roll_dice(1, height-1);
            let candidate = Point::new(vx, vy);
            if !seeds.contains(&candidate) {
                seeds.push(candidate);
            }
        }

        seeds
    }

}