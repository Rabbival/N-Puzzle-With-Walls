#![allow(clippy::type_complexity)]
mod app;
mod output;
mod input;
mod logic;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::input::{
        mouse_input_handler::*,
        keyboard_input_handler::*
    };
    pub use crate::output::{
        camera::*,
        asset_loader::*,
        error_handler::*,
        graphics::*,
        print_to_console::*,
    };
    pub use crate::logic::{
        grid::*,
        board_manager::*,
        basic_direction::*,
        tile_type::*,
        grid_location::*,
        tile_dictionary::*,
        tile_type_board::*
    };
}
