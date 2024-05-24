use crate::prelude::*;

#[derive(Event)]
pub struct LoaderScreenActionEvent {
    pub action: LoaderScreenAction,
}

#[derive(Event)]
pub struct LoaderSlotSetEvent{
    pub layout_entity: Entity,
    pub slot_to_set: LoaderScreenSlot
}

pub struct LoaderUiEventPlugin;

impl Plugin for LoaderUiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LoaderScreenActionEvent>()
            .add_event::<LoaderSlotSetEvent>();
    }
}
