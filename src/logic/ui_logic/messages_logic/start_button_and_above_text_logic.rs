use crate::output::graphics::ui::RED_TEXT_COLOR;
use crate::prelude::*;


pub struct TextAboveStartButtonLogicPlugin;

impl Plugin for TextAboveStartButtonLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    listen_for_apply_button_press,
                    (
                        listen_for_alert_dismissal,
                        alert_player_of_reached_bounds.before(update_wall_count_unapplied),
                        alert_player_of_unsaved_changes.after(update_wall_count_unapplied),
                    ).chain(),
                    show_board_couldnt_be_generated,
                    update_main_button_text_to_show_functionality
                )
                    .run_if(in_state(AppState::Menu)),
            )
            .add_systems(
                OnExit(AppState::Menu),
                reset_texts_above_start_button
                    .in_set(StateChangeSystemSets::StateChangeListening)
            );
    }
}

fn reset_texts_above_start_button(
    mut lower_text_above_start_button_query: Query<&mut Text, (With<LowerTextAboveStartButton>, Without<UpperTextAboveStartButton>)>,
    mut upper_text_above_start_button_query: Query<&mut Text, (With<UpperTextAboveStartButton>, Without<LowerTextAboveStartButton>)>,
){
    set_text_section_value_and_color(
        &mut lower_text_above_start_button_query.single_mut().sections[0],
        None,
        Some(TextAboveStartButtonType::NoText.to_string())
    );
    set_text_section_value_and_color(
        &mut upper_text_above_start_button_query.single_mut().sections[0],
        None,
        Some(TextAboveStartButtonType::NoText.to_string())
    );
}

fn listen_for_apply_button_press(
    mut event_reader: EventReader<ApplyButtonPressed>,
    mut lower_text_above_start_button_query: Query<&mut Text, With<LowerTextAboveStartButton>>
){
    for _apply_button_press in event_reader.read() {
        set_text_section_value_and_color(
            &mut lower_text_above_start_button_query.single_mut().sections[0],
            None,
            Some(TextAboveStartButtonType::NoText.to_string())
        );
    }
}

fn alert_player_of_unsaved_changes(
    mut event_reader: EventReader<MenuButtonPressed>,
    mut lower_text_above_start_button_query: Query<&mut Text, With<LowerTextAboveStartButton>>,
    planned_board_properties_query: Query<&BoardProperties, With<PlannedBoardProperties>>,
    unapplied_menu_wall_count: Res<UnappliedMenuWallCount>
) {
    for menu_button_press in event_reader.read() {
        if let MenuButtonAction::ChangeWallTilesCount(_) = menu_button_press.action{
            let applied_to_plan_wall_count = planned_board_properties_query.single().wall_count;
            let new_lower_text_above_start_button_value =
                if unapplied_menu_wall_count.0 == applied_to_plan_wall_count{
                    TextAboveStartButtonType::NoText.to_string()
                }else{
                    TextAboveStartButtonType::UnappliedChanges.to_string()
                };
            set_text_section_value_and_color(
                &mut lower_text_above_start_button_query.single_mut().sections[0],
                None,
                Some(new_lower_text_above_start_button_value)
            );
        }
    }
}

fn listen_for_alert_dismissal(
    mut event_reader: EventReader<DismissIrrelevantAlerts>,
    mut upper_text_above_start_button_query: Query<&mut Text, With<UpperTextAboveStartButton>>,
){
    for _ in event_reader.read() {
        let mut upper_text_above_start_button = upper_text_above_start_button_query.single_mut();
        if *upper_text_above_start_button.sections[0].value != TextAboveStartButtonType::NoText.to_string()
            && ! upper_text_above_start_button.is_changed()
        {
            set_text_section_value_and_color(
                &mut upper_text_above_start_button.sections[0],
                None,
                Some(TextAboveStartButtonType::NoText.to_string())
            );
        }
    }
}

pub fn alert_player_of_reached_bounds(
    mut button_event_reader: EventReader<MenuButtonPressed>,
    planned_board_prop_query: Query<
        &BoardProperties,
        (
            With<PlannedBoardProperties>,
            Without<AppliedBoardProperties>,
        ),
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
    mut upper_text_above_start_button_query: Query<&mut Text, With<UpperTextAboveStartButton>>,
) {
    for button_event in button_event_reader.read() {
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action {
            alert_player_of_reached_bounds_inner(
                &wall_count_action,
                planned_board_prop_query.single(),
                &mut unapplied_menu_wall_count,
                &mut upper_text_above_start_button_query
            );
        }
    }
}

fn alert_player_of_reached_bounds_inner(
    wall_count_action: &WallTilesChange,
    planned_board_prop: &BoardProperties,
    unapplied_menu_wall_count: &mut UnappliedMenuWallCount,
    upper_text_above_start_button_query: &mut Query<&mut Text, With<UpperTextAboveStartButton>>,
) {
    match wall_count_action {
        WallTilesChange::Increase | WallTilesChange::Decrease => {
            let mut should_change_text_value = false;
            if let WallTilesChange::Increase = wall_count_action {
                if unapplied_menu_wall_count.0 >= planned_board_prop.size.wall_count_upper_bound() {
                    should_change_text_value = true;
                }
            } else if unapplied_menu_wall_count.0 == 0 {
                should_change_text_value = true;
            }
            if should_change_text_value {
                set_text_section_value_and_color(
                    &mut upper_text_above_start_button_query.single_mut().sections[0],
                    None,
                    Some(TextAboveStartButtonType::MenuError(MenuError::CantGoBeyondTileCountBounds(*wall_count_action)).to_string())
                );
            }
        }
        _ => {}
    }
}


fn update_main_button_text_to_show_functionality(
    mut button_event_reader: EventReader<MenuButtonPressed>,
    mut generation_text_query: Query<&mut Text, With<BoardGenerationTextTag>>,
) {
    for button_event in button_event_reader.read() {
        if let MenuButtonAction::ChangeGenerationMethod(generation_method)
            = button_event.action
        {
            set_text_section_value_and_color(
                &mut generation_text_query.single_mut().sections[0],
                None,
                Some(generation_method.to_generation_button_text())
            );
        }
    }
}

fn show_board_couldnt_be_generated(
    mut event_reader: EventReader<ShowGenerationError>,
    mut main_button_text_query: Query<
        &mut Text, 
        (With<BoardGenerationTextTag>, Without<UpperTextAboveStartButton>)
    >,
    mut upper_text_above_start_button_query: Query<
        &mut Text,
        (With<UpperTextAboveStartButton>, Without<BoardGenerationTextTag>)
    >,
) {
    for _ in event_reader.read() {
        set_text_section_value_and_color(
            &mut upper_text_above_start_button_query.single_mut().sections[0],
            None,
            Some(TextAboveStartButtonType::CouldntGenerateBoard.to_string())
        );
        set_text_section_value_and_color(
            &mut main_button_text_query.single_mut().sections[0],
            Some(RED_TEXT_COLOR), 
            None
        );
    }
}