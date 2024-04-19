use crate::output::graphics::ui::{GREEN_TEXT_COLOR, NORMAL_TEXT_COLOR, RED_TEXT_COLOR};
use crate::prelude::*;

const TIME_UNTIL_TEXT_ABOVE_SAVE_GOES_BACK_AFTER_SUCCESS: f32 = 2.0;
const TIME_UNTIL_TEXT_ABOVE_SAVE_GOES_BACK_AFTER_FAILURE: f32 = 8.0;

#[derive(Component)]
pub struct TextAboveSaveButton;

#[derive(Component)]
pub struct SaveWallsLayoutTextResetTimer(pub Timer);

pub struct TextAboveSaveButtonPlugin;

impl Plugin for TextAboveSaveButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    show_walls_layout_save_attempt_outcome,
                    tick_text_reset_timer,
                    reset_text_above_save_button_when_timer_is_done
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                OnExit(AppState::Game),
                reset_text_above_save_button_when_exiting_game_screen
                    .in_set(StateChangeSystemSets::StateChangeListening)
            );
    }
}

fn reset_text_above_save_button_when_exiting_game_screen(
    mut text_above_save_button_query: Query<&mut Text, (With<TextAboveSaveButton>, Without<SaveWallsLayoutTextTag>)>,
    mut save_button_text_query: Query<&mut Text, (With<SaveWallsLayoutTextTag>, Without<TextAboveSaveButton>)>,
){
    reset_text_above_save_button_inner(
        &mut text_above_save_button_query.single_mut().sections[0],
        &mut save_button_text_query.single_mut().sections[0],
    );
}

fn reset_text_above_save_button_when_timer_is_done(
    mut text_reset_event_listener: EventReader<ResetTextAboveSaveButton>,
    mut text_above_save_button_query: Query<&mut Text, (With<TextAboveSaveButton>, Without<SaveWallsLayoutTextTag>)>,
    mut save_button_text_query: Query<&mut Text, (With<SaveWallsLayoutTextTag>, Without<TextAboveSaveButton>)>,
){
    for _timer_reset in text_reset_event_listener.read(){
        reset_text_above_save_button_inner(
            &mut text_above_save_button_query.single_mut().sections[0],
            &mut save_button_text_query.single_mut().sections[0],
        );
    }
}

fn reset_text_above_save_button_inner(
    text_above_save_button_section: &mut TextSection,
    save_button_text_section: &mut TextSection,
){
    set_text_section_value_and_color(
        text_above_save_button_section,
        Some(RED_TEXT_COLOR),
        Some(TextAboveSaveButtonType::NoText.to_string())
    );
    set_text_section_value_and_color(
        save_button_text_section,
        Some(NORMAL_TEXT_COLOR),
        None
    );
}

fn show_walls_layout_save_attempt_outcome(
    mut event_listener: EventReader<LayoutSaveAttemptOutcomeEvent>,
    mut text_above_save_button_query: Query<&mut Text, (With<TextAboveSaveButton>, Without<SaveWallsLayoutTextTag>)>,
    mut save_button_text_query: Query<&mut Text, (With<SaveWallsLayoutTextTag>, Without<TextAboveSaveButton>)>,
    mut commands: Commands,
){
    for layout_attempt_outcome in event_listener.read(){
        let save_button_text_new_color;
        let text_above_button_new_color;
        let save_button_reset_timer;
        
        let text_above_save_button_section = 
            &mut text_above_save_button_query.single_mut().sections[0];
        let text_above_button_new_value = 
            TextAboveSaveButtonType::from_save_attempt_outcome(layout_attempt_outcome.0).to_string();
        let save_button_text_section = 
            &mut save_button_text_query.single_mut().sections[0];
        let save_button_text_new_value = None;
        
        if let SaveAttemptOutcome::LayoutSavedSuccessfully = layout_attempt_outcome.0 {
            text_above_button_new_color = Some(GREEN_TEXT_COLOR);
            save_button_text_new_color = Some(GREEN_TEXT_COLOR);
            save_button_reset_timer = 
                Timer::from_seconds(TIME_UNTIL_TEXT_ABOVE_SAVE_GOES_BACK_AFTER_SUCCESS, TimerMode::Once);
        }else{
            text_above_button_new_color = None;
            save_button_text_new_color = Some(RED_TEXT_COLOR);
            save_button_reset_timer =
                Timer::from_seconds(TIME_UNTIL_TEXT_ABOVE_SAVE_GOES_BACK_AFTER_FAILURE, TimerMode::Once);
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
        commands.spawn(SaveWallsLayoutTextResetTimer(save_button_reset_timer));
    }
}

fn tick_text_reset_timer(
    mut text_reset_event_writer: EventWriter<ResetTextAboveSaveButton>,
    time: Res<Time>,
    mut save_button_timer_query: Query<(&mut SaveWallsLayoutTextResetTimer, Entity)>,
    mut commands: Commands
){
    let query_length = save_button_timer_query.iter().len();
    for (mut timer_ref, timer_entity) in save_button_timer_query.iter_mut(){
        timer_ref.0.tick(time.delta());
        if timer_ref.0.just_finished(){
            commands.entity(timer_entity).despawn();
            if query_length <= 1 {
                text_reset_event_writer.send(ResetTextAboveSaveButton);
            }
        }
    }
}