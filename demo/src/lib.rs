use wasm_bindgen::prelude::*;
use web_sys;
use rand::prelude::*;
use mapgen::{Map, MapBuilder, TileType, geometry::Point};
use mapgen::filter::*;
use mapgen::metric;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Floor = 0,
    Wall = 1,
}

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    tiles: Vec<Cell>,
    map: Map,
}

#[wasm_bindgen]
pub struct Position {
    col: usize,
    row: usize,
}


#[wasm_bindgen]
impl World {
    
    fn new(width: u32, height: u32, map: Map) -> World {
        let tiles = (0..map.tiles.len())
            .map(|i| if map.tiles[i] == TileType::Floor {Cell::Floor} else {Cell::Wall})
            .collect();
        World { width, height, tiles, map }
    }

    pub fn new_cellular_automata(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("Cellular Automata with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(NoiseGenerator::uniform())
            .with(CellularAutomata::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .with(CullUnreachable::new())
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        World::print_map_metrics(&map);
        World::new(width, height, map)
    }

    pub fn new_simple_rooms(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("Simple Rooms with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(SimpleRooms::new())
            .with(NearestCorridors::new())
            .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        World::print_map_metrics(&map);
        World::new(width, height, map)
    }

    pub fn new_bsp_rooms(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("BSP Rooms with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(BspRooms::new())
            .with(NearestCorridors::new())
            .with(AreaStartingPosition::new(XStart::LEFT, YStart::BOTTOM))
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        World::print_map_metrics(&map);
        World::new(width, height, map)
    }

    pub fn new_bsp_interior(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("BSP Interior with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(BspInterior::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .with(CullUnreachable::new())
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        World::print_map_metrics(&map);
        World::new(width, height, map)
    }

    pub fn new_drunkard(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("Drunkard with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(DrunkardsWalk::open_halls())
            .with(AreaStartingPosition::new(XStart::RIGHT, YStart::BOTTOM))
            .with(CullUnreachable::new())
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        World::print_map_metrics(&map);
        World::new(width, height, map)
    }

    pub fn new_maze(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("Maze with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(MazeBuilder::new())
            .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        World::print_map_metrics(&map);
        World::new(width, height, map)
    }

    pub fn new_voronoi(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("Voronoi Hive with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(width as usize, height as usize)
            .with(VoronoiHive::new())
            .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
            .with(DistantExit::new())
            .build_with_rng(&mut rng);
        World::print_map_metrics(&map);
        World::new(width, height, map)
    }

    pub fn new_random(width: u32, height: u32, seed: u32) -> World {
        let mut rng = rand::thread_rng();
        let px = rng.gen::<f32>();
        if px < 1.0/6.0 {
            World::new_cellular_automata(width, height, seed)
        } else if px < 2.0/6.0 {
            World::new_simple_rooms(width, height, seed)
        } else if px < 3.0/6.0 {
            World::new_drunkard(width, height, seed)
        } else if px < 4.0/6.0 {
            World::new_bsp_rooms(width, height, seed)
        } else if px < 5.0/6.0 {
            World::new_bsp_rooms(width, height, seed)
        } else {
            World::new_maze(width, height, seed)
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tiles(&self) -> *const Cell {
        self.tiles.as_ptr()
    }

    pub fn player_pos(&self) -> Position {
        let p = self.map.starting_point.unwrap_or(Point::new(0, 0));
        Position { col: p.x, row: p.y }
    }

    pub fn exit_pos(&self) -> Position {
        let p = self.map.exit_point.unwrap_or(Point::new(0, 0));
        Position { col: p.x, row: p.y }
    }

    fn print_map_info(info: String) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let div = document.get_element_by_id("map-info").expect("Need div with id: map-info");
        div.set_inner_html(&info);
    }
    
    fn print_map_metrics(map: &Map) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let div = document.get_element_by_id("map-metrics").expect("Need div with id: map-metrics");
        let density = metric::density(map);
        let path_length = metric::path_length(map);
        let info = format!("Metrics: density: {}, path length: {}", density, path_length);
        div.set_inner_html(&info);
    }
}

#[wasm_bindgen]
impl Position {
    pub fn new(col: usize, row: usize) -> Position {
        Position { col, row }
    }

    pub fn col(&self) -> usize {
        self.col
    }
    
    pub fn row(&self) -> usize {
        self.row
    }
}