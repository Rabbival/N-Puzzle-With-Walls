use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum LoaderScreenAction {
    ChangeScreen(ScreenChangeRequestType),
    WarnBeforeDeletion(AreYouSureMessageType),
    GenerateBoard(Option<Entity>),
    JumpToChosenLayoutScreen(Option<usize>),    //TODO: add difficulty too so that it'll also change to the correct one
    ChooseLayoutInSlot(LoaderScreenSlot)
}