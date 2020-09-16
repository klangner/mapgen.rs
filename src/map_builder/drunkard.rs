//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::map_builder::{
//!     MapGenerator,
//!     drunkard::DrunkardsWalkGen
//! };
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = DrunkardsWalkGen::open_area();
//! let map = gen.generate_map(80, 50, &mut rng);
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use super::MapGenerator;
use crate::{
    map::{Map, Symmetry, TileType},
    geometry::Point,
    random::Rng
};


#[derive(PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode { StartingPoint, Random }

pub struct DrunkardsWalkGen {
    spawn_mode : DrunkSpawnMode,
    drunken_lifetime : i32,
    floor_percent: f32,
    brush_size: usize,
    symmetry: Symmetry
}

impl MapGenerator for DrunkardsWalkGen {
    fn generate_map(&self, width: usize, height: usize, rng: &mut StdRng) -> Map {
        self.build(rng, width, height)
    }
}

impl DrunkardsWalkGen {
    pub fn new( spawn_mode: DrunkSpawnMode, 
                drunken_lifetime: i32, 
                floor_percent: f32,
                brush_size: usize, 
                symmetry: Symmetry) -> Box<DrunkardsWalkGen>
    {
        Box::new(DrunkardsWalkGen{
            spawn_mode,
            drunken_lifetime,
            floor_percent,
            brush_size,
            symmetry
        })
    }

    pub fn open_area() -> Box<DrunkardsWalkGen> {
        DrunkardsWalkGen::new(DrunkSpawnMode::StartingPoint, 400, 0.5, 1, Symmetry::None)
    }

    pub fn open_halls() -> Box<DrunkardsWalkGen> {
        DrunkardsWalkGen::new(DrunkSpawnMode::Random, 400, 0.5, 1, Symmetry::None)
    }

    pub fn winding_passages() -> Box<DrunkardsWalkGen> {
        DrunkardsWalkGen::new(DrunkSpawnMode::Random, 400, 0.4, 1, Symmetry::None)
    }

    pub fn fat_passages() -> Box<DrunkardsWalkGen> {
        DrunkardsWalkGen::new(DrunkSpawnMode::Random, 400, 0.4, 2, Symmetry::None)
    }

    pub fn fearful_symmetry() -> Box<DrunkardsWalkGen> {
        DrunkardsWalkGen::new(DrunkSpawnMode::Random, 400, 0.4, 1, Symmetry::Both)
    }
    
    fn build(&self, rng: &mut StdRng, width: usize, height: usize) -> Map {
        let mut map = Map::new(width, height);
        // Set a central starting point
        let starting_position = Point::new( map.width / 2, map.height / 2 );
        map.set_tile(starting_position.x, starting_position.y, TileType::Floor);

        let total_tiles = map.width * map.height;
        let desired_floor_tiles = (self.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = map.tiles.iter().filter(|a| **a == TileType::Floor).count();
        let mut digger_count = 0;
        while floor_tile_count  < desired_floor_tiles {
            let mut drunk_x;
            let mut drunk_y;
            match self.spawn_mode {
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = starting_position.x;
                    drunk_y = starting_position.y;
                }
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        drunk_x = starting_position.x;
                        drunk_y = starting_position.y;
                    } else {
                        drunk_x = rng.roll_dice(1, map.width - 3) + 1;
                        drunk_y = rng.roll_dice(1, map.height - 3) + 1;
                    }
                }
            }
            let mut drunk_life = self.drunken_lifetime;

            while drunk_life > 0 {
                map.set_tile(drunk_x, drunk_y, TileType::Wall); 
                map.paint(self.symmetry, self.brush_size, drunk_x, drunk_y);
                // map.exit_point = Some(Point::new(drunk_x, drunk_y));

                let stagger_direction = rng.roll_dice(1, 4);
                match stagger_direction {
                    1 => { if drunk_x > 2 { drunk_x -= 1; } }
                    2 => { if drunk_x < map.width-2 { drunk_x += 1; } }
                    3 => { if drunk_y > 2 { drunk_y -=1; } }
                    _ => { if drunk_y < map.height-2 { drunk_y += 1; } }
                }

                drunk_life -= 1;
            }

            digger_count += 1;
            // for t in map.tiles.iter_mut() {
            //     if *t == TileType::DownStairs {
            //         *t = TileType::Floor;
            //     }
            // }
            floor_tile_count = map.tiles.iter().filter(|a| **a == TileType::Floor).count();
        }

        map
    }
}