use crate::prelude::*;


#[derive(Event)]
pub struct LayoutSaveAttemptOutcomeEvent(pub SaveAttemptOutcome);

pub struct SystemEventPlugin;

impl Plugin for SystemEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LayoutSaveAttemptOutcomeEvent>();
    }
}