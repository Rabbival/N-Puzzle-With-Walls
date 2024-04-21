pub mod saved_layouts_screen;
pub mod saved_layouts_screens_manager;
pub mod loader_screen_layout_text_tag;

use crate::prelude::*;

pub struct LoaderScreenLogicPlugin;

impl Plugin for LoaderScreenLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SavedLayoutsScreensManagerPlugin
        ));
    }
}