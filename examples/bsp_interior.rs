use fastrand::Rng;
use mapgen::{
    poi::{AreaStartingPosition, DistantExit, XStart, YStart},
    rooms::{BspInterior, NearestCorridors},
};

fn main() {
    let mut rng = Rng::with_seed(907647352);
    let bsp = BspInterior::default();
    let rooms = bsp.generate(20, 10, &mut rng);
    let corridors = NearestCorridors::new();
    let map = corridors.generate(&rooms);
    let starting_point = AreaStartingPosition::find(XStart::LEFT, YStart::TOP, &map.walkable_layer);
    let exit_point = DistantExit::find(&starting_point, &map.walkable_layer);

    println!("{:}", &map);
    println!("Start: {:?}, exit: {:?}", starting_point, exit_point);
}
