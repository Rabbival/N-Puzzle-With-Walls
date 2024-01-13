use crate::{costume_event::ui_event, output::graphics::ui_graphics, prelude::*};

pub mod eternal_ui_logic;
pub mod menu_ui_logic;

pub struct UiLogicPlugin;

impl Plugin for UiLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuUiLogicPlugin, EternalUiLogicPlugin))
            .add_systems(
                Update,
                toggle_button.in_set(InputSystemSets::PostMainChanges),
            );
    }
}

fn toggle_button(
    mut event_listener: EventReader<ui_event::ToggleButton>,
    mut query: Query<(&mut BackgroundColor, Option<&SelectedOptionTag>), With<Button>>,
    mut commands: Commands,
) {
    for toggle_request in event_listener.read() {
        let entity_to_toggle = toggle_request.entity;
        if let Ok((mut button_color, optional_selected_tag)) = query.get_mut(entity_to_toggle) {
            if optional_selected_tag.is_none() {
                ui_graphics::set_color_to_pressed(&mut button_color);
                commands
                    .entity(toggle_request.entity)
                    .insert(SelectedOptionTag);
            } else {
                ui_graphics::set_color_to_normal(&mut button_color);
                commands
                    .entity(entity_to_toggle)
                    .remove::<SelectedOptionTag>();
            }
        }
    }
}
