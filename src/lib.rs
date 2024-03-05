#![allow(clippy::type_complexity)]
mod app;
mod bundles;
mod costume_event;
mod input;
mod logic;
mod output;
mod screen_setup;
mod system_sets;
mod data_base;
mod system;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::bundles::tile_bundle::*;
    pub use crate::costume_event::{
        app_event::*, board_set_event::*, db_event::*, EventPlugins,
        game_event::*, move_tile_event::*, screen_changing_event::*, ui_event::*, ui_spawn_event::*,
    };
    pub use crate::data_base::{
        data_base_manager::*,
        DataBasePlugins,
        domain_board::*,
        domain_board_index::*,
    };
    pub use crate::input::{
        button_input::*, InputPlugin, keyboard_input_handler::*, mouse_input_handler::*,
        move_request::*,
    };
    pub use crate::logic::{
        board_building::{
            board_builder::*, board_entities_spawner::*, BoardBuildingPlugins,
            brute_force_builder::*, permutation_builder::*, solved_board_builder::*,
            wall_placement_validator::*,
        },
        board_manager::*,
        board_props::{board_properties::*, BoardPropsPlugins, update_board_properties::*},
        BoardPlugins,
        data_structure::{
            grid_related::{grid::*, grid_cycle_checker::*, grid_location::*, grid_traveller::*, grid_tree::*},
            indexed_value::*,
            linked_list::*,
            util_functions::*,
        },
        enums::{
            basic_direction::*,
            board_property_enums::{
                board_size::*, generation_method::*, grid_traveller_type::*, menu_button_action::*,
                wall_tiles_change::*,
            },
            eternal_button_action::*,
            folder_to_access::*,
            game_button_action::*,
            tile_type::*,
        },
        states::{app_state::*, game_state::*, StatePlugin},
        tile::*,
        tile_board::*,
        tile_dictionary::*,
        ui_logic::{eternal_ui_logic::*, menu_ui_logic::*, UiLogicPlugin, victory_ui_logic::*},
    };
    pub use crate::output::{
        console::{
            costume_error::{
                MismatchError,
                error_handler::*,
                board_generation_error::*,
                menu_error::*,
                tile_move_error::*,
                entity_related_costume_error::*,
                system_access_error::*,
                data_struct_error::{
                    DataStructError,
                    grid_tree_error::*,
                    tile_board_error::*,
                    grid_error::*,
                },
            },
            costume_print::{
                BevyPrintType,
                print_display_deriver_vec,
                game_log::*,
                solution_printer::*,
                system_log::*,
            }
        },
        graphics::{
            ui::{
                eternal_buttons_spawner::*, menu_graphics::*, menu_spawner::*, messages_graphics::*,
                UiGraphicsPlugin, 
                build_node_bundle_with_full_percentage_style,
                set_color_to_normal,
                set_color_to_pressed
            },
            tile_graphics::*, 
            camera::*,
            GraphicsPlugin,
            OnOwnScreenVisibility, 
        },
    };
    pub use crate::system::{
        asset_loader::*,
        ron_loader::*,
        text_file_system_access::*,
        system_access::*,
    };
    pub use crate::screen_setup::*;
    pub use crate::system_sets::*;
}
