use crate::prelude::*;

pub mod tile_graphics;
pub mod ui_graphics;


pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TileGraphicsPlugin,
                UiGraphicsPlugin,
            ))
            ;
    }
}