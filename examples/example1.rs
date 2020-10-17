use mapgen::{
    MapBuilder,
    filter::{
        NoiseGenerator, 
        CellularAutomata,
        AreaStartingPosition,
        XStart, 
        YStart,
    },
};


fn main() {
    let map = MapBuilder::new(80, 50)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .build();  
    
        println!("{:}", &map);
}