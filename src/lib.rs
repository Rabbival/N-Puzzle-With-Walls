#![allow(clippy::type_complexity)]
mod app;
mod camera;
mod board;
mod input_handler;
mod board_manager;
mod asset_loader;
mod graphics;
mod basic_direction;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::camera::*;
    pub use crate::board::*;
    pub use crate::input_handler::*;
    pub use crate::board_manager::*;
    pub use crate::asset_loader::*;
    pub use crate::graphics::*;
    pub use crate::basic_direction::*;
}
