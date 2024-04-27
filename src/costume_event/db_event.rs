use crate::prelude::*;

#[derive(Event)]
pub struct SaveToDB(pub DomainBoard);

#[derive(Event)]
pub struct RemoveFromDB(pub SavedLayoutIndex);

#[derive(Event)]
pub struct ClearDB;

#[derive(Event)]
pub struct SuccessSavingToDB(pub SavedLayoutIndex);

#[derive(Event)]
pub struct SuccessRemovingFromDB(pub SavedLayoutIndex);

#[derive(Event)]
pub struct SuccessClearingDB;

pub struct DataBaseEventPlugin;

impl Plugin for DataBaseEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveToDB>()
            .add_event::<ClearDB>()
            .add_event::<RemoveFromDB>()
            .add_event::<SuccessSavingToDB>()
            .add_event::<SuccessRemovingFromDB>()
            .add_event::<SuccessClearingDB>()
        ;
    }
}
