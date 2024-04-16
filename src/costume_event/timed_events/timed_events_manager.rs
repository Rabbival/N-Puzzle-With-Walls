use crate::prelude::*;


pub struct TimedEventsManagerPlugin;

impl Plugin for TimedEventsManagerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, update_all_timed_events);
    }
}

fn spawn_timed_events(
    mut commands: Commands
){
    //commands.spawn()
}

fn update_all_timed_events<T: Event + Clone>(
    mut event_writer: EventWriter<T>,
    mut timed_events_query: Query<(&mut TimedEvent<T>, Entity)>,
    time: Time,
    mut commands: Commands
){
    for (mut timed_event, timed_event_entity) in timed_events_query.iter_mut(){
        timed_event.time_until_call -= time.delta().as_secs_f32();
        if timed_event.time_until_call <= 0.0 {
            commands.get_entity(timed_event_entity).unwrap().despawn();
            event_writer.send(timed_event.fire_once_done.clone())
        }
    }
}