use crate::prelude::*;
#[derive(Component)]
pub struct TextAboveSaveButton;

pub struct TextAboveSaveButtonPlugin;

impl Plugin for TextAboveSaveButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    im_a_dummy_function_to_check_the_text
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                OnExit(AppState::Game),
                reset_text_above_save_button
                    .in_set(StateChangeSystemSets::StateChangeListening)
            );
    }
}

fn reset_text_above_save_button(
    mut text_above_save_button_query: Query<&mut Text, With<TextAboveSaveButton>>
){
    let text_above_save_button = &mut text_above_save_button_query.single_mut().sections[0].value;
    *text_above_save_button = TextAboveSaveButtonType::NoText.to_string();
}

fn im_a_dummy_function_to_check_the_text(
    mut event_listener: EventReader<SaveWallsLayoutButtonPressed>,
    mut text_above_save_button_query: Query<&mut Text, With<TextAboveSaveButton>>
) {
    for _ in event_listener.read() {
        let text_above_save_button = &mut text_above_save_button_query.single_mut().sections[0].value;
        *text_above_save_button = TextAboveSaveButtonType::WallLayoutAlreadyExistsInMemory.to_string();
    }
}
