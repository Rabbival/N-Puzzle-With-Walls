use crate::prelude::*;

#[derive(Component, Default, PartialEq, Eq, Debug)]
pub struct CustomOnScreenTag {
    pub screen: AppState,
    pub on_own_screen_visibility: Option<Visibility>
}