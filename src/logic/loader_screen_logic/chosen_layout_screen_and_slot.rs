use crate::prelude::*;

#[derive(Resource, Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct ChosenLayoutScreenAndSlot(pub Option<LayoutLoaderScreenAndSlot>);

pub struct ChosenLayoutScreenAndSlotPlugin;

impl Plugin for ChosenLayoutScreenAndSlotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChosenLayoutScreenAndSlot>();
    }
}

//TODO: make it a None if the layout is deleted 