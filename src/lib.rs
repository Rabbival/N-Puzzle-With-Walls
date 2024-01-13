#![allow(clippy::type_complexity)]
mod app;
mod bundles;
mod costume_event;
mod input;
mod logic;
mod output;
mod screen_setup;
mod system_sets;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::bundles::tile_bundle::*;
    pub use crate::costume_event::{
        app_event::*, board_set_event::*, move_tile_event::*, screen_changing_event::*,
        ui_event::*, ui_spawn_event::*, game_event::*, EventPlugins,
    };
    pub use crate::input::{
        button_input::*, keyboard_input_handler::*, mouse_input_handler::*, move_request::*,
        InputPlugin,
    };
    pub use crate::logic::{
        board_building::{
            board_builder::*, board_entities_spawner::*, brute_force_builder::*,
            permutation_builder::*, solved_board_builder::*, BoardBuildingPlugins,
        },
        board_manager::*,
        board_props::{board_properties::*, update_board_properties::*, BoardPropsPlugins},
        data_structure::{
            grid_related::{grid::*, grid_location::*, grid_traveller::*, grid_tree::*},
            indexed_value::*,
            util_functions::*,
        },
        enums::{
            basic_direction::*,
            board_property_enums::{
                board_size::*, generation_method::*, grid_traveller_type::*, menu_button_action::*,
                wall_tiles_change::*,
            },
            eternal_button_action::*,
            game_button_action::*,
            tile_type::*,
        },
        states::{app_state::*, game_state::*, StatePlugin},
        tile::*,
        tile_dictionary::*,
        tile_type_board::*,
        ui_logic::{eternal_ui_logic::*, menu_ui_logic::*, UiLogicPlugin},
        BoardPlugins,
    };
    pub use crate::output::{
        asset_loader::*,
        camera::*,
        error_handler::*,
        graphics::{
            eternal_buttons_spawner::*, menu_spawner::*, tile_graphics::*, ui_graphics::*, messages_graphics::*,
            GraphicsPlugin, OnOwnScreenVisibility,
        },
        print_to_console::*,
    };
    pub use crate::screen_setup::*;
    pub use crate::system_sets::*;
}
