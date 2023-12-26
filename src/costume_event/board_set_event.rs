use crate::prelude::*;


#[derive (Event, Default)]
pub struct BuildNewBoard{
    pub reroll_solved: bool
}

/// contains the current and previous values
#[derive (Event, Default)]
pub struct SpawnTileInLocation{
    pub tiletype: TileType,
    pub location: GridLocation
}

pub struct ResetEventPlugin;

impl Plugin for ResetEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildNewBoard>()
            .add_event::<SpawnTileInLocation>()
            ;
    }
}