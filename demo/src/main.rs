// Draw map on the screen

mod input;

use input::{Action, Input};
use macroquad::prelude::*;
use mapgen::{layer::WalkableLayer, MapBuilder, MazeBuilder};


// Settings
const WINDOW_WIDTH: usize = 1280;
const WINDOW_HEIGHT: usize = 768;

const TILE_SIZE: f32 = 32.;


#[derive(Debug)]
struct MapView {
    camera: Camera2D,
}

impl MapView {
    pub fn new() -> Self {
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32));
        camera.zoom = Vec2::new(2.0 / WINDOW_WIDTH as f32, 2.0 / WINDOW_HEIGHT as f32);
        Self {
            camera 
        }
    }

    pub fn zoom_in(&mut self, _dt: f32) {
        self.camera.zoom = Vec2::new(self.camera.zoom.x * 1.1,  self.camera.zoom.y * 1.1);
    }

    pub fn zoom_out(&mut self, _dt: f32) {
        self.camera.zoom = Vec2::new(self.camera.zoom.x * 0.9,  self.camera.zoom.y * 0.9);
    }

    pub fn move_right(&mut self, dt: f32) {
        self.camera.target.x += 100.0 * dt;
    }

    pub fn move_left(&mut self, dt: f32) {
        self.camera.target.x -= 100.0 * dt;
    }

    pub fn move_down(&mut self, dt: f32) {
        self.camera.target.y += 100.0 * dt;
    }

    pub fn move_up(&mut self, dt: f32) {
        self.camera.target.y -= 100.0 * dt;
    }

    fn draw(&self, map: &WalkableLayer) {
        set_camera(&self.camera);

        clear_background(LIGHTGRAY);
        for x in 0..map.width {
            for y in 0..map.height {
                let color = if map.is_blocked(x, y) { DARKGRAY } else { WHITE };
                draw_rectangle(
                    x as f32 * TILE_SIZE, 
                    y as f32 * TILE_SIZE, 
                    TILE_SIZE, 
                    TILE_SIZE, 
                    color);
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
    let mut map_view = MapView::new();
    let map = MapBuilder::new(100, 80)
        .with(MazeBuilder::new())
        .build();  

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

        // Update world (nothing there yet)
        // Draw world
        map_view.draw(&map.walkable_layer);
        
        next_frame().await
    }
}