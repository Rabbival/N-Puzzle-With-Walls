use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum LoaderScreenAction {
    ChangeScreen(ScreenChangeRequestType),
    WarnBeforeDeletion(AreYouSureMessageType),
    GenerateBoard(Option<Entity>),
    JumpToChosenLayoutScreen(Option<usize>),
    ChooseLayoutInSlot(LoaderScreenSlot)
}