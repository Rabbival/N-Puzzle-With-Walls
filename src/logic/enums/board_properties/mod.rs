use crate::prelude::*;

pub mod board_size;
pub mod generation_method;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BoardProperties{
    pub size: BoardSize,
    pub wall_count: u8,
    pub empty_count: u8,
    pub generation_method: BoardGenerationMethod,
}


pub struct BoardPropertiesPlugin;

impl Plugin for BoardPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BoardProperties>()
            ;
    }
}