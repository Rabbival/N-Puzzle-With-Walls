#![allow(clippy::type_complexity)]
mod app;
mod screen_setup;
mod system_sets;
mod costume_event;
mod output;
mod input;
mod logic;
mod bundles;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::screen_setup::*;
    pub use crate::system_sets::*;
    pub use crate::costume_event::{
        EventPlugins,
        reset_event::*,
        move_tile_event::*,
        screen_unloading_event::*,
        ui_event::*,
    };
    pub use crate::input::{
        InputPlugin,
        mouse_input_handler::*,
        keyboard_input_handler::*,
        button_input::*,
    };
    pub use crate::output::{
        camera::*,
        asset_loader::*,
        error_handler::*,
        graphics::{
            GraphicsPlugin,
            tile_graphics::*,
            menu_graphics::*,
            menu_spawner::*,
        },
        print_to_console::*,
    };
    pub use crate::logic::{
        BoardPlugins,
        board_building::{
            BoardBuildingPlugins,
            solved_board_builder::*,
            board_builder::*,
            permutation_builder::*,
            brute_force_builder::*,
            board_properties::*,
        },
        states::{
            StatePlugin,
            game_state::*,
        },
        enums::{
            basic_direction::*,
            tile_type::*,
            board_property_enums::{
                board_size::*,
                generation_method::*,
                wall_tiles_change::*,
                menu_button_action::*,
            },
        },
        data_structure::{
            grid_related::{
                grid::*,
                grid_location::*,
            },
            indexed_value::*,
        },
        board_manager::*,
        tile_dictionary::*,
        tile_type_board::*,
    };
    pub use crate::bundles::tile_bundle::*;
}
