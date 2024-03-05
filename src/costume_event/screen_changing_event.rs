use crate::prelude::*;

#[derive(Event, Default)]
pub struct DespawnElementsWithTag(pub CustomOnScreenTag);

#[derive(Event, Default)]
pub struct ToggleVisibilityForElementsWithTag(pub CustomOnScreenTag);

#[derive(Event, Default)]
pub struct SetPlannedPropertiesToFitCurrent;

#[derive(Event, Default)]
pub struct SetMenuElementsToFitCurrent;

pub struct ScreenChangingEventPlugin;

impl Plugin for ScreenChangingEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnElementsWithTag>()
            .add_event::<ToggleVisibilityForElementsWithTag>()
            .add_event::<SetPlannedPropertiesToFitCurrent>()
            .add_event::<SetMenuElementsToFitCurrent>();
    }
}
