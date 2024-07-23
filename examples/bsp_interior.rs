use rand::prelude::*;
use mapgen::{
    poi::{AreaStartingPosition, DistantExit, XStart, YStart}, 
    rooms::BspInterior};


fn main() {
    let mut rng = StdRng::seed_from_u64(907647352);
    let bsp = BspInterior::default();
    let map = bsp.generate_rooms(20, 10, &mut rng);
    let starting_point = AreaStartingPosition::find(XStart::LEFT, YStart::TOP, &map.walkable_layer);
    let exit_point = DistantExit::find(&starting_point, &map.walkable_layer);

    println!("{:}", &map);
    println!("Start: {:?}, exit: {:?}", starting_point, exit_point);
}