use crate::prelude::*;

#[derive(Event)]
pub struct SaveToDB(pub DomainBoard);

#[derive(Event)]
pub struct RemoveFromDB(pub DomainBoardNameWithoutPostfix);

#[derive(Event)]
pub struct ClearDB;

pub struct DataBaseEventPlugin;

impl Plugin for DataBaseEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveToDB>()
            .add_event::<ClearDB>()
            .add_event::<RemoveFromDB>();
    }
}
