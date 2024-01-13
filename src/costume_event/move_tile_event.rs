use crate::{logic::enums::basic_direction, prelude::*};

#[derive(Event)]
pub struct UpdateTileLocationGraphics {
    pub tile: Tile,
    pub new_location: GridLocation,
}

#[derive(Event)]
pub struct SwitchTilesLogic {
    pub move_neighbor_from_direction: basic_direction::BasicDirection,
    pub empty_tile_index: usize,
}

pub struct MoveTileEventPlugin;

impl Plugin for MoveTileEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateTileLocationGraphics>()
            .add_event::<SwitchTilesLogic>();
    }
}
