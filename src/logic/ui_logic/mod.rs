use crate::prelude::*;

pub mod menu_ui_logic;

pub struct UiLogicPlugin;

impl Plugin for UiLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                MenuUiLogicPlugin
            )
            ;
    }
}