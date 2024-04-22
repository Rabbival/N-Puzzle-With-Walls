use crate::prelude::*;

#[derive(Event)]
pub struct MenuButtonPressed {
    pub entity: Entity,
    pub action: MenuButtonAction,
}

#[derive(Event)]
pub struct ApplyButtonPressed {
    pub action: MenuButtonAction,
}

#[derive(Event)]
pub struct VictoryButtonPressed {
    pub action: VictoryButtonAction,
}

#[derive(Event)]
pub struct SaveWallsLayoutButtonPressed;

#[derive(Event)]
pub struct ShowGenerationError(pub BoardGenerationError);

#[derive(Event)]
pub struct ResetButtonTextColor;

#[derive(Event)]
pub struct ResetTextAboveSaveButton;

#[derive(Event)]
pub struct ToggleButton {
    pub entity: Entity,
}

#[derive(Event)]
pub struct LoaderScreenArrowPressed {
    pub action: ScreenChangeArrowsAction,
}


pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuButtonPressed>()
            .add_event::<ApplyButtonPressed>()
            .add_event::<VictoryButtonPressed>()
            .add_event::<ShowGenerationError>()
            .add_event::<SaveWallsLayoutButtonPressed>()
            .add_event::<ResetButtonTextColor>()
            .add_event::<ToggleButton>()
            .add_event::<ResetTextAboveSaveButton>()
            .add_event::<LoaderScreenArrowPressed>();
    }
}
