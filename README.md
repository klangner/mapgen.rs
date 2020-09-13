# Game Map Generator

[![Build Status](https://travis-ci.org/klangner/mapgen.rs.svg?branch=master)](https://travis-ci.org/klangner/mapgen.rs)
[![Crates.io](https://img.shields.io/crates/v/mapgen.svg)](https://crates.io/crates/mapgen) 
[![mapgen.rs](https://docs.rs/mapgen/badge.svg)](https://docs.rs/mapgen/)

Generate procedural maps for games. [Try it in the browser](https://klangner.github.io/mapgen.rs/) using WebAssembly.


## Features

### Dungeons

  * Map generators
    * [ ] BSP Interior
    * [ ] BSP Room 
    * [x] Cellular automata
    * [ ] Diffusion-Limited Aggregation (DLA)
    * [ ] Drunkard's walk
    * [ ] Maze
    * [ ] Prefabs
    * [ ] Voronoi hive
    * [ ] Wave Function Collapse
  * Map modifiers (filters)
    * [x] Area exit point
    * [x] Area starting point
    * [x] Cellular automata
    * [x] Cull unreachable areas
    * [ ] Voronoi spawning


## Usage

Add dependency to your project
```
mapgen = "0.1"
```

Using single map generator:

```rust
use rand::prelude::*;
use mapgen::dungeon::{
    MapGenerator,
    cellular_automata::CellularAutomataGen
};

let mut rng = StdRng::seed_from_u64(100);
let gen = CellularAutomataGen::new(80, 50);
let map = gen.generate_map(&mut rng)
```

Use MapBuilder for chaining map generator and modifiers

```rust
use mapgen::dungeon::{
    MapBuilder,
    map::{Map, Point, TileType},
    cellular_automata::CellularAutomataGen,
    starting_point::{AreaStartingPosition, XStart, YStart},
    cull_unreachable::CullUnreachable,
};

let map = MapBuilder::new(Box::new(CellularAutomataGen::new(80, 50)))
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .with(CullUnreachable::new())
            .build_map();
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
