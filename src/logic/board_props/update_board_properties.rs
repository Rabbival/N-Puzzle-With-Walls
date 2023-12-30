use crate::{prelude::*, costume_event::{ui_event, board_set_event}, output::print_to_console};

pub struct UpdateBoardPropertiesPlugin;

impl Plugin for UpdateBoardPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ((
                            general_update_planned_board_properties,
                            update_wall_count_unapplied,
                            set_applied_props_and_begin_generation
                        )
                        .chain()
                        .in_set(InputSystemSets::InputHandling),
                        apply_wall_count_to_planned_props.in_set(InputSystemSets::PostMainChanges)
                    )
                    .run_if(in_state(GameState::Menu))
                    
            )
            ;
    }
}


/// for the planned board properties updates that don't require special treatment
fn general_update_planned_board_properties(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut button_event_writer_for_apply: EventWriter<ui_event::ApplyButtonPressed>,
    mut planned_board_prop_query: Query<
        &mut BoardProperties, 
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
){
    for button_event in button_event_listener.read(){
        general_update_planned_board_properties_inner(
            &mut button_event_writer_for_apply,
            &button_event.action,
            &mut planned_board_prop_query.single_mut(),
            &mut unapplied_menu_wall_count,
        );
    }
}

fn general_update_planned_board_properties_inner(
    button_event_writer_for_apply: &mut EventWriter<ui_event::ApplyButtonPressed>,
    menu_button_action: &MenuButtonAction,
    planned_board_prop: &mut BoardProperties,
    unapplied_menu_wall_count: &mut UnappliedMenuWallCount,
){
    match menu_button_action{
        MenuButtonAction::ChangeSize(new_board_size)=> {
            planned_board_prop.size = *new_board_size;
            if unapplied_menu_wall_count.0 > new_board_size.wall_count_upper_bound(){
                unapplied_menu_wall_count.0 = new_board_size.wall_count_upper_bound();
                //press the apply button to force apply
                button_event_writer_for_apply.send(ui_event::ApplyButtonPressed{
                    action: MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Apply)
                })
            }
        },
        MenuButtonAction::ChangeEmptyTilesCount(new_empty_count)=> {
            planned_board_prop.empty_count = *new_empty_count;
        },
        MenuButtonAction::ChangeGenerationMethod(generation_method)=> {
            planned_board_prop.generation_method = *generation_method;
        },
        _=> {},
    }
}

fn update_wall_count_unapplied(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    planned_board_prop_query: Query<
        &BoardProperties, 
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action{
            update_wall_count_unapplied_inner(
                &wall_count_action,
                planned_board_prop_query.single(),
                &mut unapplied_menu_wall_count
            );
        }      
    }
}

fn update_wall_count_unapplied_inner(
    wall_count_action: &WallTilesChange,
    planned_board_prop: &BoardProperties,
    unapplied_menu_wall_count: &mut UnappliedMenuWallCount,
){
    match wall_count_action{
        WallTilesChange::Increase | WallTilesChange::Decrease=> {
            if let WallTilesChange::Increase = wall_count_action{
                if unapplied_menu_wall_count.0 < planned_board_prop.size.wall_count_upper_bound(){
                    unapplied_menu_wall_count.0 += 1;
                }else{
                    print_to_console::print_menu_error(
                        MenuError::CantGoBeyondTileCountBounds(*wall_count_action)
                    );
                }
            }else{
                if unapplied_menu_wall_count.0 > 0{
                    unapplied_menu_wall_count.0 -= 1;
                }else{
                    print_to_console::print_menu_error(
                        MenuError::CantGoBeyondTileCountBounds(*wall_count_action)
                    );
                }
            }
        }
        _ => {}
    }
}

fn apply_wall_count_to_planned_props(
    mut apply_button_event_listener: EventReader<ui_event::ApplyButtonPressed>,
    mut planned_board_prop_query: Query<
        &mut BoardProperties, 
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    unapplied_menu_wall_count: Res<UnappliedMenuWallCount>,
){
    for apply_button_event in apply_button_event_listener.read(){
        if let MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Apply) 
            = apply_button_event.action
        {
            let mut planned_board_prop = planned_board_prop_query.single_mut();
            planned_board_prop.wall_count = unapplied_menu_wall_count.0;
            print_to_console::game_log(GameLog::WallCountSet(planned_board_prop.wall_count));
        }      
    }
}


fn set_applied_props_and_begin_generation(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut spawn_board_event_writer: EventWriter<board_set_event::BuildNewBoard>,
    mut camera_adjustmant_event_writer: EventWriter<SetCameraAccordingToNewSettings>,
    mut applied_board_prop_query: Query<
        &mut BoardProperties, 
        (With<AppliedBoardProperties>, Without<PlannedBoardProperties>)
    >,
    mut planned_board_prop_query: Query<
        &mut BoardProperties,
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    mut game_state: ResMut<NextState<GameState>>,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::GenerateBoard = button_event.action{
            let planned_board_prop = planned_board_prop_query.single_mut();
            let mut applied_props = applied_board_prop_query.single_mut();
            
            spawn_board_event_writer.send(board_set_event::BuildNewBoard{
                reroll_solved: true
            });
            camera_adjustmant_event_writer.send(board_set_event::SetCameraAccordingToNewSettings{
                new_grid_side_length: planned_board_prop.size.to_grid_side_length()
            });
            *applied_props = *planned_board_prop;

            game_state.set(GameState::Game);
            print_to_console::game_log(GameLog::NewBoardGenerated);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    const DECREASE_REQUEST: WallTilesChange = WallTilesChange::Decrease;
    const INCREASE_REQUEST: WallTilesChange = WallTilesChange::Increase;

    #[test]
    fn check_wall_count_cant_go_down_from_zero(){
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
        planned_board_prop: &mut BoardProperties
    )-> bool {
        update_wall_count_unapplied_inner(
            &DECREASE_REQUEST,
            planned_board_prop,
            current_unapplied_wall_count,
        );
        current_unapplied_wall_count.0 == 0
    }

    fn try_going_up_from_zero(
        current_unapplied_wall_count: &mut UnappliedMenuWallCount,
        planned_board_prop: &mut BoardProperties
    )-> bool {
        update_wall_count_unapplied_inner(
            &INCREASE_REQUEST,
            planned_board_prop,
            current_unapplied_wall_count,
        );
        current_unapplied_wall_count.0 > 0
    }


    #[test]
    fn check_wall_count_cant_go_up_from_max_for_size(){
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
        planned_board_prop: &mut BoardProperties
    )-> bool {
        update_wall_count_unapplied_inner(
            &INCREASE_REQUEST,
            planned_board_prop,
            current_unapplied_wall_count,
        );
        current_unapplied_wall_count.0 == planned_board_prop.size.wall_count_upper_bound()
    }

    fn try_going_down_from_max_for_size(
        current_unapplied_wall_count: &mut UnappliedMenuWallCount,
        planned_board_prop: &mut BoardProperties
    )-> bool {
        update_wall_count_unapplied_inner(
            &DECREASE_REQUEST,
            planned_board_prop,
            current_unapplied_wall_count,
        );
        current_unapplied_wall_count.0 < planned_board_prop.size.wall_count_upper_bound()
    }

    fn try_going_up_from_max_after_size_change(
        smaller_board_size: BoardSize,
        bigger_board_size: BoardSize
    )-> bool {
        let mut planned_board_prop = BoardProperties::default();
        planned_board_prop.size = smaller_board_size;
        let smaller_size_wall_count_upper_bound = planned_board_prop.size.wall_count_upper_bound();
        let mut current_unapplied_wall_count 
            = UnappliedMenuWallCount(smaller_size_wall_count_upper_bound);

            update_wall_count_unapplied_inner(
            &INCREASE_REQUEST,
            &mut planned_board_prop,
            &mut current_unapplied_wall_count,
        );
        let first_check = current_unapplied_wall_count.0 == smaller_size_wall_count_upper_bound;

        planned_board_prop.size = bigger_board_size;
        update_wall_count_unapplied_inner(
            &INCREASE_REQUEST,
            &mut planned_board_prop,
            &mut current_unapplied_wall_count,
        );
        let second_check = current_unapplied_wall_count.0 > smaller_size_wall_count_upper_bound;
        first_check && second_check
    }


    #[test]
    fn check_wall_count_becoming_smaller_size_max_when_above_it(){
        let mut app = App::new();
        app
            .add_event::<ui_event::ApplyButtonPressed>()
            .add_systems(Update, check_wall_count_becoming_smaller_size_max_when_above_it_inner)
        ;
        app.update();
    }

    fn check_wall_count_becoming_smaller_size_max_when_above_it_inner(
        mut button_event_writer_for_apply: EventWriter<ui_event::ApplyButtonPressed>
    ){
        assert!(goes_down_to_new_max_if_larger(
            BoardSize::Small,
            BoardSize::Large,
            &mut button_event_writer_for_apply,
        ))
    }

    fn goes_down_to_new_max_if_larger(
        smaller_board_size: BoardSize,
        bigger_board_size: BoardSize,
        button_event_writer_for_apply: &mut EventWriter<ui_event::ApplyButtonPressed>,
    ) -> bool {
        let mut planned_board_prop = BoardProperties::default();
        planned_board_prop.size = bigger_board_size;
        let bigger_size_wall_count_upper_bound = planned_board_prop.size.wall_count_upper_bound();
        let mut current_unapplied_wall_count 
            = UnappliedMenuWallCount(bigger_size_wall_count_upper_bound);
        
        general_update_planned_board_properties_inner(
            button_event_writer_for_apply,
            &MenuButtonAction::ChangeSize(smaller_board_size),
            &mut planned_board_prop,
            &mut current_unapplied_wall_count,
        );

        current_unapplied_wall_count.0 == smaller_board_size.wall_count_upper_bound()
    }
}