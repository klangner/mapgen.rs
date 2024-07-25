// Draw map on the screen

mod input;
mod generator;
mod settings;

use generator::*;
use input::{Action, Input};
use settings::*;
use macroquad::prelude::*;
use mapgen::layer::WalkableLayer;


#[derive(Debug)]
struct MapView {
    camera: Camera2D,
    move_speed: f32,
    texture: Texture2D,
}

impl MapView {
    pub fn new(texture: Texture2D) -> Self {
        // let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32));
        let width = (MAP_WIDTH * TILE_SIZE) as f32;
        let height = (MAP_HEIGHT * TILE_SIZE) as f32;
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., width, height));
        camera.zoom = Vec2::new(2.0 / width, 2.0 / height);
        camera.target = Vec2::new((MAP_WIDTH * TILE_SIZE) as f32 / 2., (MAP_HEIGHT * TILE_SIZE) as f32 / 2.);

        Self {
            camera,
            move_speed: 400., 
            texture,
        }
    }

    pub fn zoom_in(&mut self, _dt: f32) {
        self.camera.zoom = Vec2::new(self.camera.zoom.x * 1.05,  self.camera.zoom.y * 1.05);
    }

    pub fn zoom_out(&mut self, _dt: f32) {
        self.camera.zoom = Vec2::new(self.camera.zoom.x * 0.95,  self.camera.zoom.y * 0.95);
    }

    pub fn move_right(&mut self, dt: f32) {
        self.camera.target.x += self.move_speed * dt;
    }

    pub fn move_left(&mut self, dt: f32) {
        self.camera.target.x -= self.move_speed * dt;
    }

    pub fn move_down(&mut self, dt: f32) {
        self.camera.target.y += self.move_speed * dt;
    }

    pub fn move_up(&mut self, dt: f32) {
        self.camera.target.y -= self.move_speed * dt;
    }

    fn draw(&self, map: &WalkableLayer) {
        set_camera(&self.camera);

        clear_background(LIGHTGRAY);
        for x in 0..map.width {
            for y in 0..map.height {
                let (frame_x, frame_y) = if map.is_walkable(x, y) { (0,11) } else { (0,1) };
                let source = Rect::new(
                    (frame_x * TILE_SIZE) as f32,
                    (frame_y * TILE_SIZE) as f32, 
                    TILE_SIZE as f32, 
                    TILE_SIZE as f32);
                let params = DrawTextureParams {
                    source: Some(source),
                    ..Default::default()
                };
                draw_texture_ex(
                    &self.texture, 
                    (x * TILE_SIZE) as f32, 
                    (y * TILE_SIZE) as f32, 
                    WHITE, 
                    params);
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Map viewer".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let input: Input = Input::new();
    let mut map = MapGenerator::new();
    let tileset = load_texture("assets/tiles.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);
    let mut map_view = MapView::new(tileset);

    loop {
        let dt = get_frame_time();

        // Process input aka Controller
        #[cfg(not(target_arch = "wasm32"))]
        if input.is_action_pressed(Action::Quit) {
            break;
        }

        if input.is_action_pressed(Action::ZoomIn) {
            map_view.zoom_in(dt);
        }
        if input.is_action_pressed(Action::ZoomOut) {
            map_view.zoom_out(dt);
        }
        if input.is_action_pressed(Action::Left) {
            map_view.move_left(dt);
        }
        if input.is_action_pressed(Action::Right) {
            map_view.move_right(dt);
        }
        if input.is_action_pressed(Action::Up) {
            map_view.move_up(dt);
        }
        if input.is_action_pressed(Action::Down) {
            map_view.move_down(dt);
        }

        // // Update world 
        map.process_actions();
        // // Draw world
        map_view.draw(&map.tileset);
        
        next_frame().await
    }
}