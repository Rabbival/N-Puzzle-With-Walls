use crate::{prelude::*, logic::enums::basic_direction};

#[derive(Debug, Default)]
pub struct MoveRequest{
    pub move_neighbor_from_direction: Option<basic_direction::BasicDirection>,
    pub empty_tile_index: Option<usize>
}

impl MoveRequest{
    pub fn new(keycode: &KeyCode) -> Self{
        Self{
            move_neighbor_from_direction: basic_direction::BasicDirection::opposite_from_keycode(keycode),
            empty_tile_index: Self::empty_tile_index_from_keycode(keycode)
        }
    }

    fn empty_tile_index_from_keycode(keycode: &KeyCode) -> Option<usize> {
        match keycode{
            KeyCode::W | KeyCode::D | KeyCode::S | KeyCode::A => Some(1),
            KeyCode::Up | KeyCode::Right | KeyCode::Down | KeyCode::Left => Some(0),
            _ => None
        }
    }
}