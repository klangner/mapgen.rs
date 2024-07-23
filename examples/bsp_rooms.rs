use mapgen::rooms::BspRooms;
use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let system_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Can't access system time");
    let mut rng = StdRng::seed_from_u64(system_time.as_millis() as u64);
    let bsp = BspRooms::default();
    let map = bsp.generate_rooms(80, 50, &mut rng);
    println!("{:}", &map);
}
