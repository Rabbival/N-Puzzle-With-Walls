use crate::prelude::*;

#[derive(Event, Default)]
pub struct SetMenuElementsToFitCurrent;

pub struct ScreenChangingEventPlugin;

impl Plugin for ScreenChangingEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetMenuElementsToFitCurrent>();
    }
}
