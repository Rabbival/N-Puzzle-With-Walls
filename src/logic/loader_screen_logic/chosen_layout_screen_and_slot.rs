use crate::prelude::*;

#[derive(Resource, Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct ChosenLayoutScreenAndSlot(pub Option<LayoutLoaderScreenAndSlot>);