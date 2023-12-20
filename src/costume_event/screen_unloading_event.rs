use crate::prelude::*;

#[derive (Event, Default)]
pub struct DespawnElementsTaggedWith(OnScreenTag);

pub struct ScreenUnloadingEventPlugin;

impl Plugin for ScreenUnloadingEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DespawnElementsTaggedWith>()
            ;
    }
}