#![allow(clippy::type_complexity)]
mod app;
mod output;
mod input;
mod logic;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::input::input_handler::*;
    pub use crate::output::{
        camera::*,
        asset_loader::*,
        error_handler::*,
        graphics::*
    };
    pub use crate::logic::{
        board::*,
        board_manager::*,
        basic_direction::*,
        tile::*
    };
}
