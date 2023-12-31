use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum EternalButtonAction{
    EndGame,
    ToggleMenu
}