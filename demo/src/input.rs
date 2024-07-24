use std::collections::HashMap;

use macroquad::input::{is_key_down, KeyCode};


#[derive(Eq, Hash, PartialEq)]
pub enum Action {
    Quit,
    Left,
    Right,
    Up,
    Down,
    ZoomIn,
    ZoomOut,
}

pub struct Input {
    actions: HashMap<Action, Vec<KeyCode>>,
}

impl Input {
    pub fn new() -> Self {
        let actions: HashMap<Action, Vec<KeyCode>> = vec![
            (Action::Quit, vec![KeyCode::Q, KeyCode::Escape]),
            (Action::Left, vec![KeyCode::A, KeyCode::Left]),
            (Action::Right, vec![KeyCode::D, KeyCode::Right]),
            (Action::Up, vec![KeyCode::W, KeyCode::Up]),
            (Action::Down, vec![KeyCode::S, KeyCode::Down]),
            (Action::ZoomIn, vec![KeyCode::RightBracket]),
            (Action::ZoomOut, vec![KeyCode::LeftBracket]),
        ].into_iter().collect();

        Input { actions }
    }    

    // Check if action was executed
    pub fn is_action_pressed(&self, action: Action) -> bool {
        if let Some(key_codes) = self.actions.get(&action) {
            key_codes.iter().any(|&k| is_key_down(k))
        } else {
            false
        }

    }
}