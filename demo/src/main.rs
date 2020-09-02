use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        math::{Point3, Vector3},
        Transform, TransformBundle,
    },
    ecs::Entity,
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        types::DefaultBackend,
        RenderFlat2D, RenderToWindow, RenderingBundle, Texture,
        palette::Srgba,
    },
    tiles::{MortonEncoder, RenderTiles2D, Tile, TileMap},
    utils::application_root_dir,
    window::ScreenDimensions,
    winit,
};
use mapgen::dungeon::{
    MapBuilder,
    map::{Map, Point, TileType},
    cellular_automata::CellularAutomataGen,
    starting_point::{AreaStartingPosition, XStart, YStart},
};


#[derive(Default, Clone)]
struct MapTiles ;

impl Tile for MapTiles {
    fn sprite(&self, p: Point3<u32>, world: &World) -> Option<usize> {
        let map = world.read_resource::<Map>();
        let player_pos = Point::new(p.x as usize, p.y as usize);
        if map.starting_point == Some(player_pos) {
            Some(64)
        } else if map.at(p.x as usize, p.y as usize) == TileType::Wall {
            Some(35)
        } else {
            Some(46)
        }
    }

    fn tint(&self, p: Point3<u32>, world: &World) -> Srgba {
        let map = world.read_resource::<Map>();
        let player_pos = Point::new(p.x as usize, p.y as usize);
        if map.starting_point == Some(player_pos) {
            Srgba::new(1.0, 1.0, 0.0, 1.0)
        } else {
            Srgba::new(1.0, 1.0, 1.0, 1.0)
        }
    }
}

fn load_tiles_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn init_camera(world: &mut World, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(camera)
        .named("camera")
        .build()
}

fn init_map(world: &mut World) {
    let map = MapBuilder::new(Box::new(CellularAutomataGen::new(80, 50)))
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .build_map();
    world.insert(map); 
}


struct PlayState;
impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let mut world = data.world;
        
        // Create map
        init_map(&mut world);

        let map_sprite_sheet_handle =
            load_tiles_sprite_sheet(world, "texture/cp437_20x20.png", "texture/cp437_20x20.ron");

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let _camera = init_camera(
            world,
            Transform::from(Vector3::new(-10.0, 10.0, 1.1)),
            Camera::standard_2d(width, height),
        );

        let tile_map = TileMap::<MapTiles, MortonEncoder>::new(
            Vector3::new(80, 50, 1),
            Vector3::new(20, 20, 1),
            Some(map_sprite_sheet_handle),
        );

        let _map_entity = world
            .create_entity()
            .with(tile_map)
            .with(Transform::default())
            .build();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("demo", log::LevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;
    let assets_directory = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file("config/input.ron")?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<MapTiles, MortonEncoder>::default()),
        )?;

    let mut game = Application::build(assets_directory, PlayState)?.build(game_data)?;
    game.run();
    Ok(())
}