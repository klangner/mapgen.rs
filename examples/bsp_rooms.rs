use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;
use mapgen::*;


fn main() {
    let system_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Can't access system time");
    let mut rng = StdRng::seed_from_u64(system_time.as_millis() as u64);
    let gen = BspRooms::<NoData>::new();
    let map = gen.modify_map(&mut rng, &Map::new(80, 50));
    println!("{:}", &map);
}