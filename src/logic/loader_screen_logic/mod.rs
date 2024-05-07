pub mod displayed_loader_screen;
pub mod loader_screen_layout_text_tag;
pub mod layout_loader_screen_and_slot;
pub mod chosen_layout_properties;
pub mod game_starter_from_loader;
pub mod screen_slot_and_difficulty;

use crate::prelude::*;

pub struct LoaderScreenLogicPlugin;

impl Plugin for LoaderScreenLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DisplayedLoaderScreenPlugin,
            ChosenLayoutPropertiesPlugin,
            GameStarterFromLoaderPlugin
        ));
    }
}