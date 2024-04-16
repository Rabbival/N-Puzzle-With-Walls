use crate::prelude::*;

pub mod timed_events_manager;
pub mod timed_event;

pub struct TimedEventsPlugin;

impl Plugin for TimedEventsPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(
            TimedEventsManagerPlugin
        );
    }
}