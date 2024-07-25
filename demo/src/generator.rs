use fastrand::Rng;
// use macroquad::time::get_time;
use super::settings::*;
use macroquad::{input::{is_key_pressed, KeyCode}, time::get_time};
use mapgen::{layer::WalkableLayer, rooms::*, cave::*, MapBuilder, MazeBuilder};


pub struct MapGenerator {
    pub tileset: WalkableLayer,
}

impl MapGenerator {
    pub fn new() -> Self {
        Self { tileset: Self::maze() }
    }
    
    pub fn bsp_interior() -> WalkableLayer {
        let mut rng = Rng::with_seed((get_time() * 1000.) as u64);
        let bsp = BspInterior::default();
        let rooms = bsp.generate(MAP_WIDTH, MAP_HEIGHT, &mut rng);
        let corridors = NearestCorridors::default();
        let map = corridors.generate(&rooms);
        map.walkable_layer
    }
    
    pub fn bsp_room() -> WalkableLayer {
        let mut rng = Rng::with_seed((get_time() * 1000.) as u64);
        let bsp = BspRooms::default();
        let rooms = bsp.generate(MAP_WIDTH, MAP_HEIGHT, &mut rng);
        let corridors = NearestCorridors::default();
        let map = corridors.generate(&rooms);
        map.walkable_layer
    }
    
    pub fn maze() -> WalkableLayer {
        let mut rng = Rng::with_seed((get_time() * 1000.) as u64);
        MapBuilder::new(MAP_WIDTH, MAP_HEIGHT)
            .with(MazeBuilder::new())
            .build_with_rng(&mut rng)
            .walkable_layer
    }
    
    pub fn cellular_automata() -> WalkableLayer {
        let mut rng = Rng::with_seed((get_time() * 1000.) as u64);
        MapBuilder::new(MAP_WIDTH, MAP_HEIGHT)
            .with(NoiseGenerator::uniform())
            .with(CellularAutomata::new())
            .build_with_rng(&mut rng)
            .walkable_layer
    }

    pub fn drunkar_walk() -> WalkableLayer {
        let mut rng = Rng::with_seed((get_time() * 1000.) as u64);
        MapBuilder::new(MAP_WIDTH, MAP_HEIGHT)
            .with(DrunkardsWalk::open_halls())
            .build_with_rng(&mut rng)
            .walkable_layer
    }

    pub fn simple_rooms() -> WalkableLayer {
        let mut rng = Rng::with_seed((get_time() * 1000.) as u64);
        let bsp = SimpleRooms::default();
        let rooms = bsp.generate(MAP_WIDTH, MAP_HEIGHT, &mut rng);
        let corridors = NearestCorridors::default();
        let map = corridors.generate(&rooms);
        map.walkable_layer
    }
    
    pub fn voronoi() -> WalkableLayer {
        let mut rng = Rng::with_seed((get_time() * 1000.) as u64);
        MapBuilder::new(MAP_WIDTH, MAP_HEIGHT)
            .with(VoronoiHive::new())
            .build_with_rng(&mut rng)
            .walkable_layer
    }
    
    pub fn process_actions(&mut self) {
        if is_key_pressed(KeyCode::Key1) {
            self.tileset = Self::bsp_interior();
        } else if is_key_pressed(KeyCode::Key2) {
            self.tileset = Self::bsp_room();
        } else if is_key_pressed(KeyCode::Key3) {
            self.tileset = Self::cellular_automata();
        } else if is_key_pressed(KeyCode::Key4) {
            self.tileset = Self::drunkar_walk();
        } else if is_key_pressed(KeyCode::Key5) {
            self.tileset = Self::maze();
        } else if is_key_pressed(KeyCode::Key6) {
            self.tileset = Self::simple_rooms();
        } else if is_key_pressed(KeyCode::Key7) {
            self.tileset = Self::voronoi();
        }
    }
}