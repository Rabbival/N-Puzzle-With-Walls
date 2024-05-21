use crate::prelude::*;

#[derive(Component, Debug, Default)]
pub struct LoaderSlotOwnershipTag(pub Option<LoaderScreenSlot>);