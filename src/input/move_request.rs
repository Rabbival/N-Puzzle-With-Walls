use crate::prelude::*;

#[derive(Debug, Default)]
pub struct MoveRequest {
    pub move_neighbor_from_direction: Option<BasicDirection>,
    pub empty_tile_index: Option<usize>,
}

impl MoveRequest {
    pub fn new(keycode: &KeyCode) -> Self {
        Self {
            move_neighbor_from_direction: BasicDirection::from_keycode(
                keycode,
            ),
            empty_tile_index: Self::empty_tile_index_from_keycode(keycode),
        }
    }

    fn empty_tile_index_from_keycode(keycode: &KeyCode) -> Option<usize> {
        match keycode {
            KeyCode::KeyW | KeyCode::KeyD | KeyCode::KeyS | KeyCode::KeyA => Some(1),
            KeyCode::ArrowUp | KeyCode::ArrowRight | KeyCode::ArrowDown | KeyCode::ArrowLeft => Some(0),
            _ => None,
        }
    }
}
