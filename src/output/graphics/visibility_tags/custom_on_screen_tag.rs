use crate::prelude::*;

#[derive(Component, Default, PartialEq, Eq, Debug)]
pub struct CustomOnScreenTag {
    pub screen: AppState,
    pub on_own_screen_visibility: Option<Visibility>
}

pub fn simple_on_screen_tag(screen: AppState) -> CustomOnScreenTag {
    CustomOnScreenTag {
        screen,
        on_own_screen_visibility: None
    }
}