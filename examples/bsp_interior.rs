use rand::prelude::*;
use mapgen::rooms::BspInterior;


fn main() {
    let mut rng = StdRng::seed_from_u64(907647352);
    let bsp = BspInterior::default();
    let map = bsp.generate_rooms(20, 10, &mut rng);
    println!("{:}", &map);
}