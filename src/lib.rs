#![allow(clippy::type_complexity)]
mod app;
mod camera;
mod board;
mod input_handling;
mod board_manager;
mod asset_loader;

pub mod prelude {
    pub use bevy::reflect::TypeUuid;
    pub use bevy::{prelude::*, utils::HashMap};

    pub use crate::app::*;
    pub use crate::camera::*;
    pub use crate::board::*;
    pub use crate::input_handling::*;
    pub use crate::board_manager::*;
    pub use crate::asset_loader::*;
}
