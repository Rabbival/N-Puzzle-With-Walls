#![allow(clippy::type_complexity)]
mod app;
mod screen_setup;
mod system_sets;
mod costume_event;
mod output;
mod input;
mod logic;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::screen_setup::*;
    pub use crate::system_sets::*;
    pub use crate::costume_event::{
        EventPlugin,
        reset_event::ResetEventPlugin,
        move_tile_event::MoveTileEventPlugin
    };
    pub use crate::input::{
        InputPlugin,
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
