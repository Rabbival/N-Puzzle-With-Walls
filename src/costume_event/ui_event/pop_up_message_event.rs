use crate::prelude::*;

#[derive(Event)]
pub struct PopUpMessageButtonEvent {
    pub action: PopUpMessageButtonAction
}

#[derive(Event)]
pub struct KeyboardKeyTypedEvent(pub KeyCode);

pub struct PopUpMessageEventPlugin;

impl Plugin for PopUpMessageEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PopUpMessageButtonEvent>()
            .add_event::<KeyboardKeyTypedEvent>();
    }
}