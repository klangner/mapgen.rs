//! Example generator usage:
//! ```
//! use fastrand::Rng;
//! use mapgen::{CaveMap, MapFilter};
//! use mapgen::cave::DrunkardsWalk;
//!
//! let mut rng = Rng::with_seed(100);
//! let gen = DrunkardsWalk::open_area();
//! let map = gen.modify_map(&mut rng, &CaveMap::new(80, 50));
//!
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//!

use fastrand::Rng;

use crate::MapFilter;
use crate::geometry::Vec2u;

use super::tile_map::Symmetry;
use super::CaveMap;

#[derive(PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode {
    StartingPoint,
    Random,
}

pub struct DrunkardsWalk {
    spawn_mode: DrunkSpawnMode,
    drunken_lifetime: i32,
    floor_percent: f32,
    brush_size: u32,
    symmetry: Symmetry,
}

impl MapFilter for DrunkardsWalk {
    fn modify_map(&self, rng: &mut Rng, map: &CaveMap) -> CaveMap {
        self.build(rng, map)
    }
}

impl DrunkardsWalk {
    pub fn new(
        spawn_mode: DrunkSpawnMode,
        drunken_lifetime: i32,
        floor_percent: f32,
        brush_size: u32,
        symmetry: Symmetry,
    ) -> Box<DrunkardsWalk> {
        Box::new(DrunkardsWalk {
            spawn_mode,
            drunken_lifetime,
            floor_percent,
            brush_size,
            symmetry,
        })
    }

    pub fn open_area() -> Box<DrunkardsWalk> {
        Self::new(DrunkSpawnMode::StartingPoint, 400, 0.5, 1, Symmetry::None)
    }

    pub fn open_halls() -> Box<DrunkardsWalk> {
        Self::new(DrunkSpawnMode::Random, 400, 0.5, 1, Symmetry::None)
    }

    pub fn winding_passages() -> Box<DrunkardsWalk> {
        Self::new(DrunkSpawnMode::Random, 400, 0.4, 1, Symmetry::None)
    }

    pub fn fat_passages() -> Box<DrunkardsWalk> {
        Self::new(DrunkSpawnMode::Random, 400, 0.4, 2, Symmetry::None)
    }

    pub fn fearful_symmetry() -> Box<DrunkardsWalk> {
        Self::new(DrunkSpawnMode::Random, 400, 0.4, 1, Symmetry::Both)
    }

    fn build(&self, rng: &mut Rng, map: &CaveMap) -> CaveMap {
        let mut new_map = map.clone();
        // Set a central starting point
        let starting_position = Vec2u::new(new_map.width / 2, new_map.height / 2);
        new_map.set_walkable(starting_position.x, starting_position.y, true);

        let total_tiles = new_map.width * new_map.height;
        let desired_floor_tiles = (self.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = new_map.walkable_layer.tiles.iter().filter(|&&a| a).count();
        let mut digger_count = 0;
        while floor_tile_count < desired_floor_tiles {
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
                        drunk_x = rng.choice(1..(new_map.width+1) - 3).unwrap() + 1;
                        drunk_y = rng.choice(1..(new_map.height+1) - 3).unwrap() + 1;
                    }
                }
            }
            let mut drunk_life = self.drunken_lifetime;

            while drunk_life > 0 {
                new_map.set_walkable(drunk_x, drunk_y, false);
                new_map.paint(self.symmetry, self.brush_size, drunk_x, drunk_y);

                let stagger_direction = rng.choice(1..5).unwrap();
                match stagger_direction {
                    1 => {
                        if drunk_x > 1 {
                            drunk_x -= 1;
                        }
                    }
                    2 => {
                        if drunk_x < new_map.width - 2 {
                            drunk_x += 1;
                        }
                    }
                    3 => {
                        if drunk_y > 1 {
                            drunk_y -= 1;
                        }
                    }
                    _ => {
                        if drunk_y < new_map.height - 2 {
                            drunk_y += 1;
                        }
                    }
                }

                drunk_life -= 1;
            }

            digger_count += 1;
            floor_tile_count = new_map.walkable_layer.tiles.iter().filter(|&&a| a).count();
        }

        new_map
    }
}
