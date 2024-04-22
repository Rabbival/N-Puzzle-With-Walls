use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy)]
pub enum ScreenChangeArrowsAction {
    Next,
    Previous,
}