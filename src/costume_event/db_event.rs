use crate::prelude::*;

#[derive(Event)]
pub struct SaveToDB(pub DomainBoard);

//TODO: make use of, don't forget to add to plugin
#[derive(Event)]
pub struct RemoveFromDB(pub DomainBoardIndex);

#[derive(Event)]
pub struct ClearDB;

pub struct DataBaseEventPlugin;

impl Plugin for DataBaseEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveToDB>()
            .add_event::<ClearDB>();
    }
}
