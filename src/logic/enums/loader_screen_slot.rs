use bevy::prelude::Component;
use enum_iterator::Sequence;

pub const SAVED_LAYOUTS_PER_SCREEN: usize = 4;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash, Sequence)]
pub enum LoaderScreenSlot {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

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
    
    pub fn to_layout_offset(&self) -> usize { *self as usize }
    
    pub fn to_camera_order(&self) -> isize { *self as isize }
}