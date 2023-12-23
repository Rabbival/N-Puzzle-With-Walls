use crate::prelude::*;


pub mod board_properties;
pub mod update_board_properties;

pub struct BoardPropsPlugins;

impl Plugin for BoardPropsPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BoardPropertiesPlugin,
                UpdateBoardPropertiesPlugin
            ))
            ;
    }
}