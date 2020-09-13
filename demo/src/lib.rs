use wasm_bindgen::prelude::*;
use rand::prelude::*;
use js_sys::Date;
use mapgen::dungeon::{
    MapBuilder,
    map::Map,
    cellular_automata::CellularAutomataGen,
    starting_point::{AreaStartingPosition, XStart, YStart},
    cull_unreachable::CullUnreachable,
    distant_exit::DistantExit,
};


#[wasm_bindgen]
pub struct World {
    map: Map,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32, height: u32) -> World {
        let seed = Date::new_0().get_time() as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        let map = MapBuilder::new(Box::new(CellularAutomataGen::new(width as usize, height as usize)))
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .with(CullUnreachable::new())
            .with(DistantExit::new())
            .build_map_with_rng(&mut rng);
        World { map }
    }

    pub fn width(&self) -> u32 {
        self.map.width as u32
    }

    pub fn height(&self) -> u32 {
        self.map.height as u32
    }
}


// Called when the wasm module is instantiated
// #[wasm_bindgen(start)]
// pub fn main() -> Result<(), JsValue> {
//     Ok(())
// }