use crate::prelude::*;

#[derive (Event, Default)]
pub struct DespawnElementsWithTag(pub OnScreenTag);

#[derive (Event, Default)]
pub struct HideElementsWithTag(pub OnScreenTag);

pub struct ScreenUnloadingEventPlugin;

impl Plugin for ScreenUnloadingEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DespawnElementsWithTag>()
            .add_event::<HideElementsWithTag>()
            ;
    }
}