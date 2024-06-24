use crate::prelude::*;

pub struct InGameArrowsGraphicsPlugin;

impl Plugin for InGameArrowsGraphicsPlugin{
    fn build(&self, app: &mut App) {
        app;

        //TODO: call get_direct_neighbors_of_empty from GameBoard when the arrows spawn
        // (listen for SpawnTileAddons requests)
        // then again when UpdateTileLocationGraphics is sent
    }
}