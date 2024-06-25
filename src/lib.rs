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

#[macro_use]
mod my_macros;


pub mod prelude {
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::bundles::{saved_layout_bundle::*, tile_bundle::*};
    pub use crate::costume_event::{
        app_event::*, board_set_event::*, db_event::*, EventPlugin,
        move_tile_event::*, screen_changing_event::*, system_event::*,
        ui_event::{
            DismissIrrelevantAlerts,
            game_ui_event::*,
            loader_ui_event::*,
            menu_ui_event::*,
            pop_up_message_event::*,
            SetEntityVisibility,
            ToggleButton,
            ui_spawn_event::*,
            UiEventPlugin,
        }
    };
    pub use crate::data_base::{
        data_base_request_handler::*,
        data_base_startup::*,
        DataBaseManager,
        DataBasePlugin,
        domain_board::*,
        domain_board_name::*,
        newborn_domain_board_name::*,
        saved_layout_index::*,
    };
    pub use crate::input::{
        button_input::*, InputPlugin, keyboard_input_handler::*, mouse_input_handler::*,
        move_request::*,
    };
    pub use crate::logic::{
        board_building::{
            board_entities_spawner::*, BoardBuildingPlugin, brute_force_builder::*,
            game_board_builder::*, solved_board_builder::*,
            wall_placement_validator::*,
        },
        board_manager::*,
        board_props::{
            board_properties::*, BoardPropsPlugin,
            unapplied_menu_wall_count::*, update_board_properties::*,
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
            board_property_enums::{
                board_difficulty::*, board_size::*, generation_method::*, grid_traveller_type::*,
                menu_button_action::*, wall_tiles_change::*,
            },
            board_quality::*,
            loader_screen_slot::*,
            system_enum::{
                folder_to_access::*,
                save_attempt_outcome::*,
                system_file_type::*,
            },
            tile_type::*,
            ui_enum::{
                button_actions::{
                    eternal_button_action::*,
                    pop_up_message_button_action::*,
                    victory_button_action::*,
                },
                game_screen_text::*,
                loader_screen_action::*,
                pop_up_message_type::*,
                screen_change_request_type::*,
                text_above_pop_up_buttons_type::*,
                text_above_start_button_type::*,
            },
        },
        loader_screen_logic::{
            chosen_layout_location::*, displayed_loader_screen::*, game_starter_from_loader::*,
            layout_loader_screen_and_slot::*, loader_screen_layout_text_tag::*,
            loader_slot_ownership_tag::*, LoaderScreenLogicPlugin, screen_slot_and_difficulty::*,
        },
        multiple_empty_tiles_choice_manager::*,
        states::{app_state::*, game_state::*, StatePlugin},
        empty_tile_arrow::*,
        tile::*,
        tile_board::*,
        tile_dictionary::*,
        ui_logic::{active_loader_slot_updater::*, eternal_ui_logic::*, loader_ui_logic::*, menu_ui_logic::*, messages_logic::{
            game_screen_text_logic::*,
            MessagesGraphicsPlugin,
            pop_up_message_logic::*,
            start_button_and_above_text_logic::*,
        }, UiLogicPlugin,
                   victory_ui_logic::*},
    };
    pub use crate::logic::enums::found_empty_neighbors::*;
    pub use crate::output::{
        console::{
            costume_error::{
                board_generation_error::*,
                data_base_error::*,
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
            },
            costume_print::{
                BevyPrintType,
                game_log::*,
                print_display_deriver_vec,
                solution_printer::*,
                system_log::*,
            }
        },
        game_session_log,
        graphics::{
            camera::*,
            GraphicsPlugin,
            tile_board_graphics::{
                TileGraphicsPlugin,
                board_spawning_tile_graphics::*, 
                tile_addons_spawner::*,
                in_game_tile_graphics::*,
                in_game_arrows_graphics::*,
            },
            ui::{
                build_node_bundle_with_full_percentage_style,
                button_and_text_styles::*,
                ButtonText,
                eternal_buttons_spawner::*, GRAY_TEXT_COLOR,
                hide_by_chosen_generation_method::*,
                ImagedButtonTag, INDIGO_TEXT_COLOR,
                loader_screen_spawner::*,
                menu_spawner::*,
                messages::{
                    MessagesSpawnersPlugin,
                    pop_up_message::*,
                    victory_message::*,
                },
                save_walls_layout_button_spawner::*,
                set_color_to_imaged_normal,
                set_color_to_imaged_pressed,
                set_color_to_normal,
                set_color_to_pressed,
                set_text_section_value_and_color,
                UiGraphicsPlugin,
            },
            visibility_tags::{
                custom_on_screen_tag::*,
                multiple_on_screen_tags::*,
            },
        },
    };
    pub use crate::system::{
        asset_loader::*,
        ron_loader::*,
        system_access::*,
        system_file_name::*,
        text_file_system_access::*,
    };
    pub use crate::screen_setup::*;
    pub use crate::system_sets::*;
}