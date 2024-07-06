use crate::output::graphics::ui::{GREEN_TEXT_COLOR, NORMAL_TEXT_COLOR, RED_TEXT_COLOR};
use crate::prelude::*;

const GAME_SCREEN_TEXT_SHORT_TIME: f32 = 2.0;
const GAME_SCREEN_TEXT_LONG_TIME: f32 = 8.0;


#[derive(Component)]
pub struct GameScreenTextResetTimer(pub Timer);

pub struct GameScreenTextLogicPlugin;

impl Plugin for GameScreenTextLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    show_walls_layout_save_attempt_outcome,
                    tick_text_reset_timer,
                    reset_text_above_save_button_when_timer_is_done,
                    show_board_couldnt_be_generated,
                    set_choose_empty_message_visibility.run_if(resource_changed::<MultipleEmptyTilesChoiceManager>),
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
    mut text_above_save_button_query: Query<&mut Text, (With<GameScreenTextType>, Without<SaveWallsLayoutTextTag>)>,
    mut save_button_text_query: Query<&mut Text, (With<SaveWallsLayoutTextTag>, Without<GameScreenTextType>)>,
){
    reset_text_above_save_button_inner(
        &mut text_above_save_button_query.single_mut().sections[0],
        &mut save_button_text_query.single_mut().sections[0],
    );
}

fn reset_text_above_save_button_when_timer_is_done(
    mut text_reset_event_reader: EventReader<ResetTextAboveSaveButton>,
    mut text_above_save_button_query: Query<&mut Text, (With<GameScreenTextType>, Without<SaveWallsLayoutTextTag>)>,
    mut save_button_text_query: Query<&mut Text, (With<SaveWallsLayoutTextTag>, Without<GameScreenTextType>)>,
){
    for _timer_reset in text_reset_event_reader.read(){
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
        Some(GameScreenTextType::NoText.to_string())
    );
    set_text_section_value_and_color(
        save_button_text_section,
        Some(NORMAL_TEXT_COLOR),
        None
    );
}

fn show_walls_layout_save_attempt_outcome(
    mut event_reader: EventReader<LayoutSaveAttemptOutcomeEvent>,
    mut text_above_save_button_query: Query<&mut Text, (With<GameScreenTextType>, Without<SaveWallsLayoutTextTag>)>,
    mut save_button_text_query: Query<&mut Text, (With<SaveWallsLayoutTextTag>, Without<GameScreenTextType>)>,
    mut commands: Commands,
){
    for layout_attempt_outcome in event_reader.read(){
        let save_button_text_new_color;
        let text_above_button_new_color;
        let save_button_reset_timer;
        
        let text_above_save_button_section = 
            &mut text_above_save_button_query.single_mut().sections[0];
        let text_above_button_new_value = 
            GameScreenTextType::from_save_attempt_outcome(layout_attempt_outcome.0.clone()).to_string();
        let save_button_text_section = 
            &mut save_button_text_query.single_mut().sections[0];
        let save_button_text_new_value = None;
        
        if let SaveAttemptOutcome::LayoutSavedSuccessfully(_) = layout_attempt_outcome.0 {
            text_above_button_new_color = Some(GREEN_TEXT_COLOR);
            save_button_text_new_color = Some(GREEN_TEXT_COLOR);
            save_button_reset_timer = 
                Timer::from_seconds(GAME_SCREEN_TEXT_SHORT_TIME, TimerMode::Once);
        }else{
            text_above_button_new_color = Some(RED_TEXT_COLOR);
            save_button_text_new_color = Some(RED_TEXT_COLOR);
            save_button_reset_timer =
                Timer::from_seconds(GAME_SCREEN_TEXT_LONG_TIME, TimerMode::Once);
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
        commands.spawn(GameScreenTextResetTimer(save_button_reset_timer));
    }
}

fn show_board_couldnt_be_generated(
    mut event_reader: EventReader<ShowGenerationError>,
    mut game_screen_text_query: Query<&mut Text, With<GameScreenTextType>>,
    mut commands: Commands,
) {
    for _ in event_reader.read() {
        set_text_section_value_and_color(
            &mut game_screen_text_query.single_mut().sections[0],
            Some(RED_TEXT_COLOR),
            Some(GameScreenTextType::CouldntGenerateBoard.to_string())
        );
        commands.spawn(GameScreenTextResetTimer(
            Timer::from_seconds(GAME_SCREEN_TEXT_LONG_TIME, TimerMode::Once))
        );
    }
}

fn set_choose_empty_message_visibility(
    multiple_empty_tiles_choice_manager: Res<MultipleEmptyTilesChoiceManager>,
    mut game_screen_text_query: Query<&mut Text, With<GameScreenTextType>>,
){
    if multiple_empty_tiles_choice_manager.choice_pending {
        set_text_section_value_and_color(
            &mut game_screen_text_query.single_mut().sections[0],
            Some(GRAY_TEXT_COLOR),
            Some(GameScreenTextType::EmptyTileChoicePending.to_string())
        );
    } else{
        set_text_section_value_and_color(
            &mut game_screen_text_query.single_mut().sections[0],
            None,
            Some(GameScreenTextType::NoText.to_string())
        );
    }
}

fn tick_text_reset_timer(
    mut text_reset_event_writer: EventWriter<ResetTextAboveSaveButton>,
    time: Res<Time>,
    mut save_button_timer_query: Query<(&mut GameScreenTextResetTimer, Entity)>,
    mut commands: Commands
){
    let query_length = save_button_timer_query.iter().len();
    for (mut timer_ref, timer_entity) in &mut save_button_timer_query{
        timer_ref.0.tick(time.delta());
        if timer_ref.0.just_finished(){
            commands.entity(timer_entity).despawn();
            if query_length <= 1 {
                text_reset_event_writer.send(ResetTextAboveSaveButton);
            }
        }
    }
}