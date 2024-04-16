use crate::output::graphics::ui::{GREEN_TEXT_COLOR, NORMAL_TEXT_COLOR, RED_TEXT_COLOR};
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
                    show_walls_layout_saved_successfully_message
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
    mut text_above_save_button_query: Query<&mut Text, (With<TextAboveSaveButton>, Without<SaveWallsLayoutTextTag>)>,
    mut save_button_text_query: Query<&mut Text, (With<SaveWallsLayoutTextTag>, Without<TextAboveSaveButton>)>
){
    let text_above_save_button_section = &mut text_above_save_button_query.single_mut().sections[0];
    set_text_section_value_and_color(
        text_above_save_button_section,
        Some(RED_TEXT_COLOR),
        Some(TextAboveSaveButtonType::NoText.to_string())
    );
    let text_above_save_button_section = &mut save_button_text_query.single_mut().sections[0];
    set_text_section_value_and_color(
        text_above_save_button_section,
        Some(NORMAL_TEXT_COLOR),
        None
    );
}

fn show_walls_layout_saved_successfully_message(
    mut event_listener: EventReader<LayoutSaveAttemptOutcomeEvent>,
    mut text_above_save_button_query: Query<&mut Text, (With<TextAboveSaveButton>, Without<SaveWallsLayoutTextTag>)>,
    mut save_button_text_query: Query<&mut Text, (With<SaveWallsLayoutTextTag>, Without<TextAboveSaveButton>)>
){
    for layout_attempt_outcome in event_listener.read(){
        let text_above_save_button_section = 
            &mut text_above_save_button_query.single_mut().sections[0];
        let text_above_button_new_value = 
            TextAboveSaveButtonType::from_save_attempt_outcome(layout_attempt_outcome.0).to_string(); 
        let mut text_above_button_new_color = None;
        let save_button_text_section = 
            &mut save_button_text_query.single_mut().sections[0];
        let save_button_text_new_value = None;
        let mut save_button_text_new_color = Some(RED_TEXT_COLOR);
        if let SaveAttemptOutcome::LayoutSavedSuccessfully = layout_attempt_outcome.0 {
            text_above_button_new_color = Some(GREEN_TEXT_COLOR);
            save_button_text_new_color = Some(GREEN_TEXT_COLOR);
        };
        set_text_section_value_and_color(
            text_above_save_button_section,
            text_above_button_new_color,
            Some(text_above_button_new_value)
        );
        set_text_section_value_and_color(
            save_button_text_section,
            save_button_text_new_color,
            save_button_text_new_value
        );
    }
}