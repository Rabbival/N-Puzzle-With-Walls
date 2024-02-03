use crate::prelude::*;

#[derive(Event, Default)]
pub struct BuildNewBoard {
    pub reroll_solved: bool,
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
