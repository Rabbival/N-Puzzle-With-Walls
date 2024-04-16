use crate::prelude::*;


#[derive(Resource, Debug, Clone, Default, PartialEq, Eq)]
pub struct CurrentBoardWallLocations(pub Vec<GridLocation>);

pub struct CurrentBoardWallLocationsPlugin;

impl Plugin for CurrentBoardWallLocationsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoardWallLocations>();
    }
}