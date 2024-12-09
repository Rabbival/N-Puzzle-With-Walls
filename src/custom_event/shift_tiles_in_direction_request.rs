use crate::prelude::*;

#[derive(Debug, Event)]
pub struct ShiftTilesInDirectionRequest {
    pub direction_to_shift_from: BasicDirection,
    pub empty_tile_index: usize,
    pub steps_count: usize,
}

pub struct ShiftTilesInDirectionRequestPlugin;

impl Plugin for ShiftTilesInDirectionRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShiftTilesInDirectionRequest>();
    }
}

impl ShiftTilesInDirectionRequest {
    pub fn new(keycode: &KeyCode) -> Option<Self> {
        if let Some(direction) = BasicDirection::from_keycode(keycode) {
            if let Some(empty_tile_index) = Self::empty_tile_index_from_keycode(keycode) {
                return Some(Self {
                    direction_to_shift_from: direction,
                    empty_tile_index,
                    steps_count: 1,
                });
            }
        }
        None
    }

    fn empty_tile_index_from_keycode(keycode: &KeyCode) -> Option<usize> {
        match keycode {
            KeyCode::KeyW | KeyCode::KeyD | KeyCode::KeyS | KeyCode::KeyA => Some(1),
            KeyCode::ArrowUp | KeyCode::ArrowRight | KeyCode::ArrowDown | KeyCode::ArrowLeft => {
                Some(0)
            }
            _ => None,
        }
    }
}
