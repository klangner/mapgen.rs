use mapgen::cave::{CellularAutomata, NoiseGenerator};
use mapgen::{poi::*, MapBuilder};

fn main() {
    let map = MapBuilder::new(20, 20)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .build(100);

    let starting_point =
        AreaStartingPosition::find(XStart::CENTER, YStart::CENTER, &map.walkable_layer);
    let walkables = CullUnreachable::remove_walkable_tiles(&starting_point, &map.walkable_layer);

    println!("{:}", &walkables);
}
