use crate::output::graphics::ui::RED_TEXT_COLOR;
use crate::prelude::*;
#[derive(Component)]
pub struct TextAboveStartButton;

pub struct TextAboveStartButtonPlugin;

impl Plugin for TextAboveStartButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    listen_for_apply_button_press,
                    (
                        alert_player_of_unsaved_changes,
                        alert_player_of_reached_bounds
                    ).after(update_wall_count_unapplied),
                    show_board_couldnt_be_generated,
                    update_main_button_text_to_show_functionality
                )
                    .run_if(in_state(AppState::Menu)),
            )
            .add_systems(
                OnExit(AppState::Menu),
                reset_text_above_start_button
                    .in_set(StateChangeSystemSets::StateChangeListening)
            );
    }
}

fn reset_text_above_start_button(
    mut text_above_start_button_query: Query<&mut Text, With<TextAboveStartButton>>
){
    let text_above_start_button = &mut text_above_start_button_query.single_mut().sections[0].value;
    *text_above_start_button = TextAboveStartButtonType::NoText.to_string();
}

fn listen_for_apply_button_press(
    mut event_listener: EventReader<ApplyButtonPressed>,
    mut text_above_start_button_query: Query<&mut Text, With<TextAboveStartButton>>
){
    for _apply_button_press in event_listener.read() {
        let text_above_start_button = &mut text_above_start_button_query.single_mut().sections[0].value;
        *text_above_start_button = TextAboveStartButtonType::NoText.to_string();
    }
}

fn alert_player_of_unsaved_changes(
    mut event_listener: EventReader<MenuButtonPressed>,
    mut text_above_start_button_query: Query<&mut Text, With<TextAboveStartButton>>,
    planned_board_properties_query: Query<&BoardProperties, With<PlannedBoardProperties>>,
    unapplied_menu_wall_count: Res<UnappliedMenuWallCount>
) {
    for menu_button_press in event_listener.read() {
        if let MenuButtonAction::ChangeWallTilesCount(_) = menu_button_press.action{
            let text_above_start_button = &mut text_above_start_button_query.single_mut().sections[0].value;
            let applied_to_plan_wall_count = planned_board_properties_query.single().wall_count;
            if unapplied_menu_wall_count.0 == applied_to_plan_wall_count{
                *text_above_start_button = TextAboveStartButtonType::NoText.to_string();
            }else{
                *text_above_start_button = TextAboveStartButtonType::UnappliedChanges.to_string();
            }
        }
    }
}

pub fn alert_player_of_reached_bounds(
    mut button_event_listener: EventReader<MenuButtonPressed>,
    planned_board_prop_query: Query<
        &BoardProperties,
        (
            With<PlannedBoardProperties>,
            Without<AppliedBoardProperties>,
        ),
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
    mut text_above_start_button_query: Query<&mut Text, With<TextAboveStartButton>>,
) {
    for button_event in button_event_listener.read() {
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action {
            alert_player_of_reached_bounds_inner(
                &wall_count_action,
                planned_board_prop_query.single(),
                &mut unapplied_menu_wall_count,
                &mut text_above_start_button_query.single_mut().sections[0].value
            );
        }
    }
}

fn alert_player_of_reached_bounds_inner(
    wall_count_action: &WallTilesChange,
    planned_board_prop: &BoardProperties,
    unapplied_menu_wall_count: &mut UnappliedMenuWallCount,
    text_above_start_button: &mut String
) {
    match wall_count_action {
        WallTilesChange::Increase | WallTilesChange::Decrease => {
            if let WallTilesChange::Increase = wall_count_action {
                if unapplied_menu_wall_count.0 >= planned_board_prop.size.wall_count_upper_bound() {
                    *text_above_start_button = TextAboveStartButtonType::MenuError(MenuError::CantGoBeyondTileCountBounds(*wall_count_action)).to_string();
                }
            } else if unapplied_menu_wall_count.0 <= 0 {
                *text_above_start_button = TextAboveStartButtonType::MenuError(MenuError::CantGoBeyondTileCountBounds(*wall_count_action)).to_string();
            }
        }
        _ => {}
    }
}


fn update_main_button_text_to_show_functionality(
    mut button_event_listener: EventReader<MenuButtonPressed>,
    mut generation_text_query: Query<&mut Text, With<BoardGenerationTextTag>>,
) {
    for button_event in button_event_listener.read() {
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
    mut event_listener: EventReader<ShowGenerationError>,
    mut main_button_text_query: Query<
        &mut Text, 
        (With<BoardGenerationTextTag>, Without<TextAboveStartButton>)
    >,
    mut text_above_start_button_query: Query<
        &mut Text,
        (With<TextAboveStartButton>, Without<BoardGenerationTextTag>)
    >,
) {
    for _ in event_listener.read() {
        set_text_section_value_and_color(
            &mut text_above_start_button_query.single_mut().sections[0],
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