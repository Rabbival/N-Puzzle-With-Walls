use crate::{prelude::*, logic::enums::basic_direction};

#[derive (Event, Default)]
pub struct SwitchTilesGraphics{
    pub first_grid_location: GridLocation, 
    pub second_grid_location: GridLocation
}

#[derive (Event)]
pub struct SwitchTilesLogic{
    pub move_neighbor_from_direction: basic_direction::BasicDirection,
    pub empty_tile_index: usize
}

pub struct MoveTileEventPlugin;

impl Plugin for MoveTileEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SwitchTilesGraphics>()
            .add_event::<SwitchTilesLogic>()
            ;
    }
}