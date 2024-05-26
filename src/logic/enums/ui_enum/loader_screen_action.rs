use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum LoaderScreenAction {
    ChangeScreen(ScreenChangeRequestType),
    WarnBeforeDeletion(PopUpMessageType),
    GenerateBoard(Option<Entity>),
    JumpToChosenLayoutScreen(Option<usize>, BoardDifficulty),
    ChooseLayoutInSlot(LoaderScreenSlot)
}