use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum PopUpMessageButtonAction {
    Cancel,
    Confirm,
}
