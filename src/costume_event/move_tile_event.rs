use bevy::prelude::*;
use crate::prelude::*;

#[derive (Event, Default)]
pub struct SwitchTilesGraphics{
    pub first_grid_location: GridLocation, 
    pub second_grid_location: GridLocation
}

pub struct MoveTileEventPlugin;

impl Plugin for MoveTileEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SwitchTilesGraphics>()
            ;
    }
}