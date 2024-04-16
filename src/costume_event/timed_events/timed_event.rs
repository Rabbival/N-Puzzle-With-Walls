use crate::prelude::*;

#[derive(Component)]
pub struct TimedEvent<T: Event>{
    pub time_until_call: f32,
    pub fire_once_done: T
}