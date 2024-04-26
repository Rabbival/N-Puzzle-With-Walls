pub mod displayed_loader_screen;
pub mod loader_screen_layout_text_tag;
pub mod layout_loader_screen_and_slot;
pub mod chosen_layout_screen_and_slot;

use crate::prelude::*;

pub struct LoaderScreenLogicPlugin;

impl Plugin for LoaderScreenLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DisplayedLoaderScreenPlugin);
    }
}