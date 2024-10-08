use crate::prelude::*;

#[derive(Event)]
pub struct PopUpMessageButtonEvent {
    pub action: PopUpMessageButtonAction
}

#[derive(Event)]
pub struct KeyboardKeyTypedEvent{
    pub keycode: KeyCode,
    pub shift_pressed: bool
}

#[derive(Event)]
pub struct SetConfirmAllowed(pub bool);

pub struct PopUpMessageEventPlugin;

impl Plugin for PopUpMessageEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PopUpMessageButtonEvent>()
            .add_event::<KeyboardKeyTypedEvent>()
            .add_event::<SetConfirmAllowed>();
    }
}