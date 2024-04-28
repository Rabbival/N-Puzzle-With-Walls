use crate::prelude::*;

#[derive(Event, Default)]
pub struct BuildNewBoard {
    pub build_new_solved_board: bool,
}

#[derive(Event, Default)]
pub struct SpawnTileInLocation {
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
