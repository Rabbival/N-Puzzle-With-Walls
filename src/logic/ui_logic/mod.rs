use crate::prelude::*;

pub mod eternal_ui_logic;
pub mod menu_ui_logic;
pub mod victory_ui_logic;
pub mod loader_ui_logic;
pub mod messages_logic;
pub mod active_loader_slot_updater;

pub struct UiLogicPlugin;

impl Plugin for UiLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MenuUiLogicPlugin, 
            EternalUiLogicPlugin, 
            VictoryUiLogicPlugin, 
            LoaderUiLogicPlugin,
            MessagesGraphicsPlugin,
            ActiveLoaderSlotUpdaterPlugin
        ))
            .add_systems(
                Update,
                toggle_button.in_set(InputSystemSets::PostMainChanges),
            );
    }
}

fn toggle_button(
    mut event_reader: EventReader<ToggleButton>,
    mut query: Query<(&mut BackgroundColor, Option<&SelectedOptionTag>), With<Button>>,
    mut commands: Commands,
) {
    for toggle_request in event_reader.read() {
        let entity_to_toggle = toggle_request.entity;
        if let Ok((mut button_color, optional_selected_tag)) = query.get_mut(entity_to_toggle) {
            if optional_selected_tag.is_none() {
                set_color_to_pressed(&mut button_color);
                commands
                    .entity(toggle_request.entity)
                    .insert(SelectedOptionTag);
            } else {
                set_color_to_normal(&mut button_color);
                commands
                    .entity(entity_to_toggle)
                    .remove::<SelectedOptionTag>();
            }
        }
    }
}