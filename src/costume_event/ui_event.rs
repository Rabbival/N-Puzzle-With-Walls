use crate::prelude::*;

#[derive (Event)]
pub struct ButtonPressed{
    pub entity: Entity,
    pub action: MenuButtonAction
}

#[derive (Event)]
pub struct ApplyButtonPressed{
    pub action: MenuButtonAction
}

#[derive (Event)]
pub struct ShowGenerationError(pub BoardGenerationError);

#[derive (Event)]
pub struct ResetButtonTextColor;


pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ButtonPressed>()
            .add_event::<ApplyButtonPressed>()
            .add_event::<ShowGenerationError>()
            .add_event::<ResetButtonTextColor>()
            ;
    }
}