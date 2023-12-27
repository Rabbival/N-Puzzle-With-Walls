use crate::{prelude::*, costume_event::{ui_event, board_set_event}, output::print_to_console};

pub struct UpdateBoardPropertiesPlugin;

impl Plugin for UpdateBoardPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                        general_update_planned_board_properties,
                        update_wall_count,
                        set_applied_props_and_begin_generation
                    )
                    .chain()//chained because count needs size and generation needs everything set
                    .run_if(in_state(GameState::Menu))
                    .in_set(InputSystemSets::InputHandling)
            )
            ;
    }
}


/// for the planned board properties updates that don't require special treatment
fn general_update_planned_board_properties(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut planned_board_prop_query: Query<
        &mut BoardProperties, 
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
){
    for button_event in button_event_listener.read(){
        let mut planned_board_prop = planned_board_prop_query.single_mut();
        let menu_button_action = button_event.action;
        match menu_button_action{
            MenuButtonAction::ChangeSize(new_board_size)=> {
                planned_board_prop.size = new_board_size;
                if unapplied_menu_wall_count.0 > new_board_size.wall_count_upper_bound(){
                    unapplied_menu_wall_count.0 = new_board_size.wall_count_upper_bound();
                }
            },
            MenuButtonAction::ChangeEmptyTilesCount(new_empty_count)=> {
                planned_board_prop.empty_count = new_empty_count;
            },
            MenuButtonAction::ChangeGenerationMethod(generation_method)=> {
                planned_board_prop.generation_method = generation_method;
            },
            _=> continue,
        }
    }
}

fn update_wall_count(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut planned_board_prop_query: Query<
        &mut BoardProperties, 
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action{
            let mut planned_board_prop = planned_board_prop_query.single_mut();
            match wall_count_action{
                WallTilesChange::Apply=> {
                    planned_board_prop.wall_count = unapplied_menu_wall_count.0;
                },
                WallTilesChange::Increase | WallTilesChange::Decrease=> {
                    if let WallTilesChange::Increase = wall_count_action{
                        if unapplied_menu_wall_count.0 < planned_board_prop.size.wall_count_upper_bound(){
                            unapplied_menu_wall_count.0 += 1;
                        }else{
                            print_to_console::print_menu_error(
                                MenuError::CantGoBeyondTileCountBounds(wall_count_action)
                            );
                        }
                    }else{
                        if unapplied_menu_wall_count.0 > 0{
                            unapplied_menu_wall_count.0 -= 1;
                        }else{
                            print_to_console::print_menu_error(
                                MenuError::CantGoBeyondTileCountBounds(wall_count_action)
                            );
                        }
                    }
                }
            }
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
