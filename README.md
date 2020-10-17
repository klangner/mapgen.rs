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
  * [ ] Maze
  * [x] Noise generator
  * [ ] Prefabs
  * [x] Room corridors nearest
  * [x] Simple rooms
  * [ ] Voronoi hive
  * [ ] Wave Function Collapse


## Usage

Add dependency to your project
```
mapgen = "0.4"
```

Using single map generator:

```rust
use rand::prelude::*;
use mapgen::{Map, MapFilter};
use mapgen::filter::CellularAutomata;

let mut rng = StdRng::seed_from_u64(100);
let gen = CellularAutomata::new();
let map = gen.modify_map(&mut rng, &Map::new(80, 50));
```

Use MapBuilder for chaining map generator and modifiers

```rust
use mapgen::{
    MapBuilder,
    filter::{
        NoiseGenerator, 
        CellularAutomata,
        AreaStartingPosition,
        XStart, 
        YStart,
    },
};

let map = MapBuilder::new(80, 50)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .build();
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
