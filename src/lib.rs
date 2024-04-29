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
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::bundles::{tile_bundle::*, saved_layout_bundle::*};
    pub use crate::costume_event::{
        app_event::*, board_set_event::*, db_event::*, EventPlugin,
        move_tile_event::*, screen_changing_event::*, system_event::*,
        ui_event::{
            game_ui_event::*,
            loader_ui_event::*,
            menu_ui_event::*,
            DismissIrrelevantAlerts,
            ToggleButton,
            ui_spawn_event::*,
            UiEventPlugin,
            SetEntityVisibility,
            AreYouSureMessageButtonEvent,
        }
    };
    pub use crate::data_base::{
        data_base_manager::*,
        DataBasePlugin,
        domain_board::*,
        saved_layout_index::*,
        domain_board_name::*,
    };
    pub use crate::input::{
        button_input::*, InputPlugin, keyboard_input_handler::*, mouse_input_handler::*,
        move_request::*,
    };
    pub use crate::logic::{
        board_building::{
            board_builder::*, board_entities_spawner::*, BoardBuildingPlugin,
            brute_force_builder::*, permutation_builder::*, solved_board_builder::*,
            wall_placement_validator::*,
        },
        board_manager::*,
        board_props::{
            board_properties::*, BoardPropsPlugin, current_board_wall_locations::*,
            update_board_properties::*,
        },
        BoardPlugin,
        data_structure::{
            grid_related::{grid::*, grid_cycle_checker::*, grid_location::*, grid_traveller::*, grid_tree::*},
            indexed_value::*,
            linked_list::*,
            util_functions::*,
        },
        enums::{
            basic_direction::*,
            board_quality::*,
            board_property_enums::{
                board_size::*, generation_method::*, grid_traveller_type::*, menu_button_action::*,
                wall_tiles_change::*,
            },
            loader_screen_slot::*,
            system_enum::{
                folder_to_access::*,
                save_attempt_outcome::*,
            },
            tile_type::*,
            ui_enum::{
                are_you_sure_message_type::*,
                button_actions::{
                    eternal_button_action::*,
                    victory_button_action::*,
                    are_you_sure_message_button_action::*,
                },
                text_above_save_button_type::*,
                text_above_start_button_type::*,
                screen_change_request_type::*,
                loader_screen_action::*,
            },
        },
        loader_screen_logic::{
            displayed_loader_screen::*, layout_loader_screen_and_slot::*,
            loader_screen_layout_text_tag::*, LoaderScreenLogicPlugin,
            chosen_layout_screen_and_slot::*, game_starter_from_loader::*,
        },
        states::{app_state::*, game_state::*, StatePlugin},
        tile::*,
        tile_board::*,
        tile_dictionary::*,
        ui_logic::{eternal_ui_logic::*, loader_ui_logic::*, menu_ui_logic::*, UiLogicPlugin, victory_ui_logic::*, 
        messages_logic::{
            MessagesGraphicsPlugin,
            are_you_sure_message_logic::*,
            save_button_and_above_text_logic::*,
            start_button_and_above_text_logic::*,
        }},
    };
    pub use crate::output::{
        console::{
            costume_error::{
                board_generation_error::*,
                data_struct_error::{
                    DataStructError,
                    grid_error::*,
                    grid_tree_error::*,
                    tile_board_error::*,
                },
                entity_related_costume_error::*,
                error_handler::*,
                menu_error::*,
                MismatchError,
                system_access_error::*,
                tile_move_error::*,
                data_base_error::*,
            },
            costume_print::{
                BevyPrintType,
                game_log::*,
                print_display_deriver_vec,
                solution_printer::*,
                system_log::*,
            }
        },
        graphics::{
            camera::*,
            GraphicsPlugin,
            tile_graphics::*,
            ui::{
                build_node_bundle_with_full_percentage_style,
                button_and_text_styles::*, ButtonText,
                eternal_buttons_spawner::*,
                loader_screen_spawner::*, menu_spawner::*,
                messages_spawners::{
                    are_you_sure_message_spawner::*,
                    MessagesSpawnersPlugin,
                    victory_message::*,
                },
                save_walls_layout_button_spawner::*,
                set_color_to_imaged_normal,
                set_color_to_imaged_pressed,
                set_color_to_normal,
                set_color_to_pressed,
                set_text_section_value_and_color,
                UiGraphicsPlugin,
                ImagedButtonTag
            },
            visibility_tags::{
                custom_on_screen_tag::*,
                multiple_on_screen_tags::*,
            },
        },
    };
    pub use crate::system::{
        asset_loader::*,
        board_layout_to_ron_file::*,
        ron_loader::*,
        system_access::*,
        text_file_system_access::*,
    };
    pub use crate::screen_setup::*;
    pub use crate::system_sets::*;
}
