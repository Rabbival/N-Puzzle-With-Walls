use bevy::prelude::Component;
use enum_iterator::Sequence;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Sequence, )]
pub enum LoaderScreenSlot {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

#[derive(Component)]
pub struct LoaderScreenSlotTag(pub LoaderScreenSlot);