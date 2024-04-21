use bevy::prelude::Component;
use enum_iterator::Sequence;

pub const SAVED_LAYOUTS_PER_SCREEN: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Sequence)]
pub enum LoaderScreenSlot {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

#[derive(Component)]
pub struct LoaderScreenSlotTag(pub LoaderScreenSlot);

impl LoaderScreenSlot{
    pub fn try_from_layout_offset(offset: usize) -> Option<Self>{
        match offset{
            0 => Some(LoaderScreenSlot::TopLeft),
            1 => Some(LoaderScreenSlot::TopRight),
            2 => Some(LoaderScreenSlot::BottomLeft),
            3 => Some(LoaderScreenSlot::BottomRight),
            _ => None
        }
    }
    
    pub fn to_layout_offset(&self) -> usize {
        match self{
            LoaderScreenSlot::TopLeft => 0,
            LoaderScreenSlot::TopRight => 1,
            LoaderScreenSlot::BottomLeft => 2,
            LoaderScreenSlot::BottomRight => 3
        }
    }
}