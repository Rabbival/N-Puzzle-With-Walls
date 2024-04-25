use enum_iterator::Sequence;

pub mod error_handler;
pub mod menu_error;
pub mod board_generation_error;
pub mod data_struct_error;
pub mod tile_move_error;
pub mod entity_related_costume_error;
pub mod system_access_error;
pub mod board_loading_error;


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct MismatchError {
    pub expected: String,
    pub found: String
}