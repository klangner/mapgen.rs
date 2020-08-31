pub mod map;
pub mod cellular_automata;

use rand::prelude::*;
use map::Map;


pub trait MapGenerator {
    fn generate_map(&self, rng: &mut StdRng) -> Map;
}

pub trait MapModifier {
    fn modify_map(&self, rng: &mut StdRng, map: &Map) -> Map;
}

pub struct MapBuilder {
    generator: Box<dyn MapGenerator>,
    modifiers: Vec<Box<dyn MapModifier>>,
    rng: StdRng,
}

impl MapBuilder {
    pub fn new(generator : Box<dyn MapGenerator>) -> MapBuilder {
        MapBuilder { 
            generator, 
            modifiers: Vec::new(),
            rng: StdRng::seed_from_u64(0)
        }
    }

    pub fn with(&mut self, modifier : Box<dyn MapModifier>) {
        self.modifiers.push(modifier);
    }

    pub fn build_map(&mut self) -> Map {
        let mut map = self.generator.generate_map(&mut self.rng);
        
        // Build additional layers in turn
        for modifier in self.modifiers.iter() {
            modifier.modify_map(&mut self.rng, &mut map);
        }

        map
    }
}

