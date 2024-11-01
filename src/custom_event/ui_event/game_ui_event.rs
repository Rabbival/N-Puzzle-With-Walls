use crate::prelude::*;

#[derive(Event)]
pub struct SaveLayoutButtonPressed;

#[derive(Event)]
pub struct ResetTextAboveSaveButton;

#[derive(Event)]
pub struct VictoryButtonPressed(pub VictoryButtonAction);

pub struct GameUiEventPlugin;

impl Plugin for GameUiEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VictoryButtonPressed>()
            .add_event::<SaveLayoutButtonPressed>()
            .add_event::<ResetTextAboveSaveButton>();
    }
}
