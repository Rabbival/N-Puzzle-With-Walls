use crate::logic::enums::board_building_request::BoardBuildingRequest;
use crate::prelude::*;

pub struct UpdateBoardPropertiesPlugin;

impl Plugin for UpdateBoardPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu),
                 set_planned_props_to_fit_current
            )
            .add_systems(
            Update,
            (
                (
                    general_update_planned_board_properties,
                    update_wall_count_unapplied,
                    set_applied_props_and_exit_menu,
                )
                    .in_set(InputSystemSets::InputHandling),
                apply_wall_count_to_planned_props.in_set(InputSystemSets::PostMainChanges),
            )
                .run_if(in_state(AppState::Menu)),
        );
    }
}

/// for the planned board properties updates that don't require special treatment
fn general_update_planned_board_properties(
    mut button_event_reader: EventReader<MenuButtonPressed>,
    mut button_event_writer_for_apply: EventWriter<ApplyButtonPressed>,
    mut planned_board_prop_query: Query<
        &mut BoardProperties,
        (
            With<PlannedBoardProperties>,
            Without<AppliedBoardProperties>,
        ),
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
) {
    for button_event in button_event_reader.read() {
        general_update_planned_board_properties_inner(
            &mut button_event_writer_for_apply,
            &button_event.action,
            &mut planned_board_prop_query.single_mut(),
            &mut unapplied_menu_wall_count,
        );
    }
}

fn general_update_planned_board_properties_inner(
    button_event_writer_for_apply: &mut EventWriter<ApplyButtonPressed>,
    menu_button_action: &MenuButtonAction,
    planned_board_prop: &mut BoardProperties,
    unapplied_menu_wall_count: &mut UnappliedMenuWallCount,
) {
    match menu_button_action {
        MenuButtonAction::ChangeSize(new_board_size) => {
            planned_board_prop.size = *new_board_size;
            if unapplied_menu_wall_count.0 > new_board_size.wall_count_upper_bound() {
                unapplied_menu_wall_count.0 = new_board_size.wall_count_upper_bound();
                //press the apply button to force apply
                button_event_writer_for_apply.send(ApplyButtonPressed {
                    action: MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Apply),
                });
            }
        }
        MenuButtonAction::ChangeEmptyTilesCount(new_empty_count) => {
            planned_board_prop.empty_count = *new_empty_count;
        }
        MenuButtonAction::ChangeSpanningTreeGeneration(traveller_type) => {
            planned_board_prop.tree_traveller_type = *traveller_type;
        }
        MenuButtonAction::ChangeGenerationMethod(generation_method) => {
            planned_board_prop.generation_method = *generation_method;
        }
        MenuButtonAction::ChangeBoardDifficulty(board_difficulty) => {
            planned_board_prop.board_difficulty = *board_difficulty;
        }
        _ => {}
    }
}

pub fn update_wall_count_unapplied(
    mut button_event_reader: EventReader<MenuButtonPressed>,
    planned_board_prop_query: Query<
        &BoardProperties,
        (
            With<PlannedBoardProperties>,
            Without<AppliedBoardProperties>,
        ),
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
) {
    for button_event in button_event_reader.read() {
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action {
            update_wall_count_unapplied_inner(
                &wall_count_action,
                planned_board_prop_query.single(),
                &mut unapplied_menu_wall_count,
            );
        }
    }
}

fn update_wall_count_unapplied_inner(
    wall_count_action: &WallTilesChange,
    planned_board_prop: &BoardProperties,
    unapplied_menu_wall_count: &mut UnappliedMenuWallCount,
) {
    match wall_count_action {
        WallTilesChange::Increase | WallTilesChange::Decrease => {
            if let WallTilesChange::Increase = wall_count_action {
                if unapplied_menu_wall_count.0 < planned_board_prop.size.wall_count_upper_bound() {
                    unapplied_menu_wall_count.0 += 1;
                } else {
                    print_menu_error(MenuError::CantGoBeyondTileCountBounds(
                        *wall_count_action,
                    ));
                }
            } else if unapplied_menu_wall_count.0 > 0 {
                unapplied_menu_wall_count.0 -= 1;
            } else {
                print_menu_error(MenuError::CantGoBeyondTileCountBounds(
                    *wall_count_action,
                ));
            }
        }
        _ => {}
    }
}

fn apply_wall_count_to_planned_props(
    mut apply_button_event_reader: EventReader<ApplyButtonPressed>,
    mut planned_board_prop_query: Query<
        &mut BoardProperties,
        (
            With<PlannedBoardProperties>,
            Without<AppliedBoardProperties>,
        ),
    >,
    unapplied_menu_wall_count: Res<UnappliedMenuWallCount>,
) {
    for apply_button_event in apply_button_event_reader.read() {
        if let MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Apply) =
            apply_button_event.action
        {
            let mut planned_board_prop = planned_board_prop_query.single_mut();
            planned_board_prop.wall_count = unapplied_menu_wall_count.0;
            game_log(GameLog::WallCountSet(planned_board_prop.wall_count));
        }
    }
}

pub fn set_applied_props_and_exit_menu(
    mut button_event_reader: EventReader<MenuButtonPressed>,
    mut menu_toggle_event_writer: EventWriter<ToggleMenu>,
    mut spawn_board_event_writer: EventWriter<BuildNewBoard>,
    mut applied_board_prop_query: Query<
        &mut BoardProperties,
        (
            With<AppliedBoardProperties>,
            Without<PlannedBoardProperties>,
        ),
    >,
    mut planned_board_prop_query: Query<
        &mut BoardProperties,
        (
            With<PlannedBoardProperties>,
            Without<AppliedBoardProperties>,
        ),
    >,
) {
    for button_event in button_event_reader.read() {
        if let MenuButtonAction::MainButtonPressed = button_event.action {
            let planned_board_prop = planned_board_prop_query.single_mut();
            let mut applied_props = applied_board_prop_query.single_mut();
            *applied_props = *planned_board_prop;
            
            match applied_props.generation_method{
                BoardGenerationMethod::Auto => {
                    spawn_board_event_writer.send(BuildNewBoard(BoardBuildingRequest::CreateANewBoardFromNothing));
                },
                // only board generation can fail (and force us to stay in the menu screen)
                _ => {
                    menu_toggle_event_writer.send(ToggleMenu{
                        out_of_menu_into: Some(applied_props.generation_method.to_app_state())
                    });
                }
            }
        }
    }
}

/// sets the one that appears in the menu to fit the current configuration
fn set_planned_props_to_fit_current(
    mut event_writer: EventWriter<SetMenuElementsToFitCurrent>, 
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
    applied_board_prop_query: Query<
        &BoardProperties,
        (
            With<AppliedBoardProperties>,
            Without<PlannedBoardProperties>,
        ),
    >,
    mut planned_board_prop_query: Query<
        &mut BoardProperties,
        (
            With<PlannedBoardProperties>,
            Without<AppliedBoardProperties>,
        ),
    >,
) {
    let current_props = applied_board_prop_query.single();
    let mut planned_props = planned_board_prop_query.single_mut();
    unapplied_menu_wall_count.0 = current_props.wall_count;
    *planned_props = *current_props;
    event_writer.send(SetMenuElementsToFitCurrent);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECREASE_REQUEST: WallTilesChange = WallTilesChange::Decrease;
    const INCREASE_REQUEST: WallTilesChange = WallTilesChange::Increase;

    #[test]
    fn check_wall_count_cant_go_down_from_zero() {
        assert!(try_going_down_from_zero(
            &mut UnappliedMenuWallCount(0),
            &mut BoardProperties::default()
        ));
        assert!(try_going_up_from_zero(
            &mut UnappliedMenuWallCount(0),
            &mut BoardProperties::default()
        ))
    }

    fn try_going_down_from_zero(
        current_unapplied_wall_count: &mut UnappliedMenuWallCount,
        planned_board_prop: &mut BoardProperties,
    ) -> bool {
        update_wall_count_unapplied_inner(
            &DECREASE_REQUEST,
            planned_board_prop,
            current_unapplied_wall_count,
        );
        current_unapplied_wall_count.0 == 0
    }

    fn try_going_up_from_zero(
        current_unapplied_wall_count: &mut UnappliedMenuWallCount,
        planned_board_prop: &mut BoardProperties,
    ) -> bool {
        update_wall_count_unapplied_inner(
            &INCREASE_REQUEST,
            planned_board_prop,
            current_unapplied_wall_count,
        );
        current_unapplied_wall_count.0 > 0
    }

    #[test]
    fn check_wall_count_cant_go_up_from_max_for_size() {
        assert!(try_going_up_from_max_for_size(
            &mut UnappliedMenuWallCount(BoardProperties::default().size.wall_count_upper_bound()),
            &mut BoardProperties::default()
        ));
        assert!(try_going_down_from_max_for_size(
            &mut UnappliedMenuWallCount(BoardProperties::default().size.wall_count_upper_bound()),
            &mut BoardProperties::default()
        ));
        assert!(try_going_up_from_max_after_size_change(
            BoardSize::Small,
            BoardSize::Large
        ))
    }

    fn try_going_up_from_max_for_size(
        current_unapplied_wall_count: &mut UnappliedMenuWallCount,
        planned_board_prop: &mut BoardProperties,
    ) -> bool {
        update_wall_count_unapplied_inner(
            &INCREASE_REQUEST,
            planned_board_prop,
            current_unapplied_wall_count,
        );
        current_unapplied_wall_count.0 == planned_board_prop.size.wall_count_upper_bound()
    }

    fn try_going_down_from_max_for_size(
        current_unapplied_wall_count: &mut UnappliedMenuWallCount,
        planned_board_prop: &mut BoardProperties,
    ) -> bool {
        update_wall_count_unapplied_inner(
            &DECREASE_REQUEST,
            planned_board_prop,
            current_unapplied_wall_count,
        );
        current_unapplied_wall_count.0 < planned_board_prop.size.wall_count_upper_bound()
    }

    fn try_going_up_from_max_after_size_change(
        smaller_board_size: BoardSize,
        bigger_board_size: BoardSize,
    ) -> bool {
        let mut planned_board_prop = BoardProperties{
            size: smaller_board_size,
            ..default()
        };
        let smaller_size_wall_count_upper_bound = planned_board_prop.size.wall_count_upper_bound();
        let mut current_unapplied_wall_count =
            UnappliedMenuWallCount(smaller_size_wall_count_upper_bound);

        update_wall_count_unapplied_inner(
            &INCREASE_REQUEST,
            &planned_board_prop,
            &mut current_unapplied_wall_count,
        );
        let first_check = current_unapplied_wall_count.0 == smaller_size_wall_count_upper_bound;

        planned_board_prop.size = bigger_board_size;
        update_wall_count_unapplied_inner(
            &INCREASE_REQUEST,
            &planned_board_prop,
            &mut current_unapplied_wall_count,
        );
        let second_check = current_unapplied_wall_count.0 > smaller_size_wall_count_upper_bound;
        first_check && second_check
    }

    #[test]
    fn check_wall_count_becoming_smaller_size_max_when_above_it() {
        let mut app = App::new();
        app.add_event::<ApplyButtonPressed>().add_systems(
            Update,
            check_wall_count_becoming_smaller_size_max_when_above_it_inner,
        );
        app.update();
    }

    fn check_wall_count_becoming_smaller_size_max_when_above_it_inner(
        mut button_event_writer_for_apply: EventWriter<ApplyButtonPressed>,
    ) {
        assert!(goes_down_to_new_max_if_larger(
            BoardSize::Small,
            BoardSize::Large,
            &mut button_event_writer_for_apply,
        ))
    }

    fn goes_down_to_new_max_if_larger(
        smaller_board_size: BoardSize,
        bigger_board_size: BoardSize,
        button_event_writer_for_apply: &mut EventWriter<ApplyButtonPressed>,
    ) -> bool {
        let mut planned_board_prop = BoardProperties {
            size: bigger_board_size,
            ..default()
        };
        let bigger_size_wall_count_upper_bound = planned_board_prop.size.wall_count_upper_bound();
        let mut current_unapplied_wall_count =
            UnappliedMenuWallCount(bigger_size_wall_count_upper_bound);

        general_update_planned_board_properties_inner(
            button_event_writer_for_apply,
            &MenuButtonAction::ChangeSize(smaller_board_size),
            &mut planned_board_prop,
            &mut current_unapplied_wall_count,
        );

        current_unapplied_wall_count.0 == smaller_board_size.wall_count_upper_bound()
    }
}
