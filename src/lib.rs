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
        board_set_event::*,
        move_tile_event::*,
        screen_changing_event::*,
        ui_event::*,
        ui_spawn_event::*,
        app_event::*,
    };
    pub use crate::input::{
        InputPlugin,
        mouse_input_handler::*,
        keyboard_input_handler::*,
        button_input::*,
        move_request::*,
    };
    pub use crate::output::{
        graphics::{
            GraphicsPlugin,
            tile_graphics::*,
            ui_graphics::*,
            menu_spawner::*,
            eternal_buttons_spawner::*,
        },
        camera::*,
        asset_loader::*,
        error_handler::*,
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
            board_entities_spawner::*,
        },
        board_props::{
            BoardPropsPlugins,
            board_properties::*,
            update_board_properties::*,
        },
        data_structure::{
            grid_related::{
                grid::*,
                grid_location::*,
                grid_tree::*,
            },
            indexed_value::*,
            util_functions::*,
        },
        enums::{
            basic_direction::*,
            tile_type::*,
            eternal_button_action::*,
            board_property_enums::{
                board_size::*,
                generation_method::*,
                wall_tiles_change::*,
                menu_button_action::*,
            },
        },
        states::{
            StatePlugin,
            game_state::*,
        },
        ui_logic::{
            UiLogicPlugin,
            menu_ui_logic::*,
            eternal_ui_logic::*
        },
        board_manager::*,
        tile_dictionary::*,
        tile_type_board::*,
        tile::*,
    };
    pub use crate::bundles::tile_bundle::*;
}
