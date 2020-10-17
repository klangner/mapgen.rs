use rand::prelude::*;
use mapgen::*;


fn main() {
    let mut rng = StdRng::seed_from_u64(907647352);
    let gen = BspInterior::new();
    let map = gen.modify_map(&mut rng, &Map::new(20, 10));
    println!("{:}", &map);
}