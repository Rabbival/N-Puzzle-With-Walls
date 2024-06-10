use crate::prelude::*;

#[derive(Event, Debug)]
pub struct UpdateTileLocationGraphics {
    pub tile: Tile,
    pub new_location: GridLocation,
}

#[derive(Event)]
pub struct SwitchTilesLogic {
    pub move_neighbor_from_direction: BasicDirection,
    pub empty_tile_index: usize,
}

#[derive(Event)]
pub struct SetMultipleEmptyTilesChoiceManager {
    pub new_config: MultipleEmptyTilesChoiceManager
}

#[derive(Event)]
pub struct CheckIfBoardIsSolved;

pub struct MoveTileEventPlugin;

impl Plugin for MoveTileEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateTileLocationGraphics>()
            .add_event::<SwitchTilesLogic>()
            .add_event::<CheckIfBoardIsSolved>()
            .add_event::<SetMultipleEmptyTilesChoiceManager>();
    }
}
