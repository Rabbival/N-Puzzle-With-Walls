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
pub struct SetAppliedTagForProperty{
    pub give_tag_to_variant: MenuButtonAction
}

#[derive(Event)]
pub struct ShowGenerationError(pub BoardGenerationError);

pub struct MenuUiEventPlugin;

impl Plugin for MenuUiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MenuButtonPressed>()
            .add_event::<ApplyButtonPressed>()
            .add_event::<ShowGenerationError>()
            .add_event::<SetAppliedTagForProperty>();
    }
}