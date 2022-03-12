//! Apply noise to the map.
//! Each cell will be set to Floor with the given probabilty.
//! 
//! Example usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{Map, MapFilter, NoData};
//! use mapgen::filter::NoiseGenerator;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = NoiseGenerator::<NoData>::uniform();
//! let map = gen.modify_map(&mut rng, &Map::new(80, 50));
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use std::marker::PhantomData;

use rand::prelude::*;
use crate::MapFilter;
use crate::{BuilderData, Map, Tile};


/// Map noise generator
pub struct NoiseGenerator<D: BuilderData> {
    prob: f32,
    phantom: PhantomData<D>,
}

impl<D: BuilderData> MapFilter<D> for NoiseGenerator<D> {
    fn modify_map(&self, rng: &mut StdRng, map: &Map<D>)  -> Map<D> {
        self.build(map, rng)
    }
}

impl<D: BuilderData> NoiseGenerator<D> {
    /// Create noise with custom probability
    pub fn new(prob: f32) -> Box<NoiseGenerator<D>> {
        Box::new(NoiseGenerator {
            prob,
            phantom: PhantomData,
        })
    }

    /// Create uniform noise (Probablity 0.5)
    pub fn uniform() -> Box<NoiseGenerator<D>> {
        Box::new(NoiseGenerator {
            prob: 0.5,
            phantom: PhantomData,
        })
    }

    /// Generate map
    fn build(&self, map: &Map<D>, rng: &mut StdRng) -> Map<D> {
        let mut new_map = map.clone();
        let p = (self.prob * 100.0) as u32;
        for y in 1..new_map.height-1 {
            for x in 1..new_map.width-1 {
                let roll = rng.next_u32() % 100;
                if roll > p { new_map.set_tile(x, y, Tile::floor()) } 
                else { new_map.set_tile(x, y, Tile::wall()) }
            }
        }

        new_map
    }

}
