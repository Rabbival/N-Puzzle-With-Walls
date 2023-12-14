use bevy::prelude::*;
use crate::prelude::*;

pub mod reset_event;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ResetEventPlugin,
            ))
            ;
    }
}