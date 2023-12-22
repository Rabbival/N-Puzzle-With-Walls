use crate::prelude::*;

pub mod board_size;
pub mod generation_method;
pub mod wall_tiles_change;
pub mod menu_button_action;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BoardProperties{
    pub size: BoardSize,
    pub wall_count: u8,
    pub empty_count: u8,
    pub generation_method: BoardGenerationMethod,
}

/// info that's not yet been applied to the next board's properties
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UnappliedToBoardProperties{
    pub wall_count: u8
}


pub struct BoardPropertiesPlugin;

impl Plugin for BoardPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BoardProperties>()
            ;
    }
}