use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum AreYouSureMessageButtonAction {
    Cancel,
    Confirm,
}
