use crate::prelude::*;


#[derive (Event, Default)]
pub struct BuildNewBoard{
    pub reroll_solved: bool
}

/// contains the current and previous values
#[derive (Event, Default)]
pub struct SpawnTileInLocation{
    pub indexed_tiletype: IndexedValue<TileType>,
    pub location: Vec3
}

#[derive (Event)]
pub struct SetCameraAccordingToNewSettings{
    pub new_grid_side_length: u8
}


pub struct ResetEventPlugin;

impl Plugin for ResetEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildNewBoard>()
            .add_event::<SpawnTileInLocation>()
            .add_event::<SetCameraAccordingToNewSettings>()
            ;
    }
}