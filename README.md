# Game Map Generator

[![Build Status](https://travis-ci.org/klangner/mapgen.rs.svg?branch=master)](https://travis-ci.org/klangner/mapgen.rs)
[![Crates.io](https://img.shields.io/crates/v/mapgen.svg)](https://crates.io/crates/mapgen) 
[![mapgen.rs](https://docs.rs/mapgen/badge.svg)](https://docs.rs/mapgen/)

Generate procedural maps for games. [Try it in the browser](https://klangner.github.io/mapgen.rs/) using WebAssembly.


## Map filters

This library consists of different map filters which can be combined to create custom map generator.

### Implemented filters

  * [x] Area exit point
  * [x] Area starting point
  * [x] BSP Interior
  * [x] BSP Rooms
  * [x] Cellular automata
  * [x] Cull unreachable areas
  * [ ] Diffusion-Limited Aggregation (DLA)
  * [x] Drunkard's walk
  * [x] Maze
  * [x] Noise generator
  * [ ] Prefabs
  * [x] Room corridors nearest
  * [x] Simple rooms
  * [x] Voronoi hive
  * [ ] Wave Function Collapse


## Usage

Add dependency to your project
```
mapgen = "0.5"
```

Generate room based map
```rust
use rand::prelude::*;
use mapgen::{
    poi::{AreaStartingPosition, DistantExit, XStart, YStart}, 
    rooms::BspInterior};


fn main() {
    let mut rng = StdRng::seed_from_u64(907647352);
    let bsp = BspInterior::default();
    let map = bsp.generate_rooms(20, 10, &mut rng);
    let starting_point = AreaStartingPosition::find(XStart::LEFT, YStart::TOP, &map.walkable_layer);
    let exit_point = DistantExit::find(&starting_point, &map.walkable_layer);

    println!("{:}", &map);
    println!("Start: {:?}, exit: {:?}", starting_point, exit_point);
}
```

Using single dungeon generators:

```rust
use mapgen::dungeon::{CellularAutomata, NoiseGenerator};
use mapgen::{poi::*, MapBuilder};


fn main() {
    let map = MapBuilder::new(20, 20)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .build();  
    
    let starting_point = AreaStartingPosition::find(XStart::CENTER, YStart::CENTER, &map.walkable_layer);
    let walkables = CullUnreachable::remove_walkable_tiles(&starting_point, &map.walkable_layer);
    
    println!("{:}", &walkables);
}
```

For more information check the [doc](https://docs.rs/mapgen)


This library is based on the code from the [Roguelike tutorial](https://github.com/thebracket/rustrogueliketutorial).
I highly recommend it for learning how to write Roguelike in Rust.


# License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
