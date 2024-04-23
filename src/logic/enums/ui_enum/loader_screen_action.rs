use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum LoaderScreenAction {
    ChangeScreen(ScreenChangeRequestType),
}