use wasm_bindgen::prelude::*;
use web_sys;
use rand::prelude::*;
use mapgen::dungeon::{
    MapBuilder,
    map::TileType,
    cellular_automata::CellularAutomataGen,
    simple_rooms::SimpleRoomsGen,
    starting_point::{AreaStartingPosition, XStart, YStart},
    cull_unreachable::CullUnreachable,
    distant_exit::DistantExit,
    rooms_corridors_nearest::NearestCorridors,
};


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
}

#[wasm_bindgen]
impl World {

    pub fn new_cellular_automata(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("Cellular Automata with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(CellularAutomataGen::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .with(CullUnreachable::new())
            .with(DistantExit::new())
            .build_map_with_rng(width as usize, height as usize, &mut rng);
        let tiles = (0..map.tiles.len())
            .map(|i| if map.tiles[i] == TileType::Floor {Cell::Floor} else {Cell::Wall})
            .collect();
        World { 
            width,
            height,
            tiles }
    }

    pub fn new_simple_rooms(width: u32, height: u32, seed: u32) -> World {
        World::print_map_info(format!("Simple Rooms with the seed: {}", seed));
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let map = MapBuilder::new(SimpleRoomsGen::new())
            .with(NearestCorridors::new())
            .build_map_with_rng(width as usize, height as usize, &mut rng);
        let tiles = (0..map.tiles.len())
            .map(|i| if map.tiles[i] == TileType::Floor {Cell::Floor} else {Cell::Wall})
            .collect();
        World { 
            width,
            height,
            tiles }
    }

    pub fn new_random(width: u32, height: u32, seed: u32) -> World {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.5 {
            World::new_cellular_automata(width, height, seed)
        } else {
            World::new_simple_rooms(width, height, seed)
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

    fn print_map_info(info: String) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let div = document.get_element_by_id("map-info").expect("Need div with id: map-info");
        div.set_inner_html(&info);
    }
}