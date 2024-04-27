use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum LoaderScreenAction {
    ChangeScreen(ScreenChangeRequestType),
    WarnBeforeDeletion(AreYouSureMessageType),
    GenerateBoard(Option<SavedLayoutIndex>),
    JumpToChosenLayoutPage(Option<usize>),
    ChooseLayoutInSlot(LoaderScreenSlot)
}