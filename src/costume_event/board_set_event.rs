use bevy::prelude::*;

use crate::prelude::{TileType, NewAndFormer};


#[derive (Event, Default)]
pub struct BuildNewBoard{
    pub reroll_solved: bool
}

/// contains the current and previous values
#[derive (Event, Default)]
pub struct SpawnOrDispawnTiles{
    pub max_tiletype: NewAndFormer<TileType>,
    pub empty_count: NewAndFormer<u8>,
    pub wall_count: NewAndFormer<u8>
}

pub struct ResetEventPlugin;

impl Plugin for ResetEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildNewBoard>()
            .add_event::<SpawnOrDispawnTiles>()
            ;
    }
}