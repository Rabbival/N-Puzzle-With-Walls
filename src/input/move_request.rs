use crate::logic::enums::basic_direction;

#[derive(Debug, Default)]
pub struct MoveRequest{
    pub move_neighbor_from_direction: Option<basic_direction::BasicDirection>,
    pub empty_tile_index: usize
}