use crate::logic::enums::board_building_request::BoardBuildingRequest;
use crate::prelude::*;

#[derive(Event)]
pub struct BuildNewBoard(pub BoardBuildingRequest);

#[derive(Event, Default)]
pub struct SpawnTileInLocation {
    pub optional_loader_slot: Option<LoaderScreenSlot>,
    pub tile: Tile,
    pub location: Vec3,
}

pub struct ResetEventPlugin;

impl Plugin for ResetEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildNewBoard>()
            .add_event::<SpawnTileInLocation>();
    }
}
