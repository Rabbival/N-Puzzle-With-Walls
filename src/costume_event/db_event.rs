use crate::prelude::*;

#[derive(Event)]
pub struct SaveToDB(pub DomainBoard);

#[derive(Event)]
pub struct LoadFromDB(pub DomainBoardIndex);

pub struct DataBaseEventPlugin;

impl Plugin for DataBaseEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveToDB>()
            .add_event::<LoadFromDB>();
    }
}
