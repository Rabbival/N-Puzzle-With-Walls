use crate::prelude::*;

#[derive(Event, Default)]
pub struct SaveToDB {
    pub board: DomainBoard,
    pub name: DomainBoardName,
    pub existing_boards_with_same_name_and_difficulty: Vec<SavedLayoutIndexInDifficultyVec>,
}

#[derive(Event)]
pub struct RemoveFromDB(pub SavedLayoutIndexInDifficultyVec);

#[derive(Event)]
pub struct ClearDB;

#[derive(Event)]
pub struct SuccessSavingToDB(pub SavedLayoutIndexInDifficultyVec);

#[derive(Event)]
pub struct SuccessRemovingFromDB(pub SavedLayoutIndexInDifficultyVec);

#[derive(Event)]
pub struct SuccessClearingDB;

#[derive(Event)]
pub struct SetNewbornDomainBoardNameToDefault;

#[derive(Event)]
pub struct UpdateNewbornDomainBoardName(pub DomainBoardName);

pub struct DataBaseEventPlugin;

impl Plugin for DataBaseEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveToDB>()
            .add_event::<ClearDB>()
            .add_event::<RemoveFromDB>()
            .add_event::<SuccessSavingToDB>()
            .add_event::<SuccessRemovingFromDB>()
            .add_event::<SuccessClearingDB>()
            .add_event::<SetNewbornDomainBoardNameToDefault>()
            .add_event::<UpdateNewbornDomainBoardName>();
    }
}
