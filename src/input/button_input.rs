use crate::{prelude::*, costume_event::board_set_event, output::{print_to_console, graphics::menu_graphics}};
use std::mem;

pub struct ButtonInputPlugin;

impl Plugin for ButtonInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,(
                    handle_menu_buttons, 
                    set_chosen_options_to_fit_current_props
                )
                .run_if(in_state(GameState::Menu)),
            );
    }
}


fn handle_menu_buttons(
    mut input_event_writer: EventWriter<board_set_event::SpawnBoardWithNewSettings>,
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<ApplyButtonTag>)
    >,
    mut currently_applied: Query<
        Entity,
        (With<AppliedOptionTag>, Without<SelectedOptionTag>)
    >,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut applied_board_prop_query: Query<
        &mut BoardProperties, 
        (With<AppliedBoardProperties>, Without<PlannedBoardProperties>)
    >,
    mut planned_board_prop_query: Query<
        &mut BoardProperties, 
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
    mut commands: Commands
) {
    for (
        interaction, 
        menu_button_action, 
        entity
    ) 
    in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let mut planned_board_prop = planned_board_prop_query.single_mut();
            match menu_button_action{
                MenuButtonAction::ChangeSize(new_board_size)=> {
                    planned_board_prop.size = *new_board_size;
                },
                MenuButtonAction::ChangeEmptyTilesCount(new_empty_count)=> {
                    planned_board_prop.empty_count = *new_empty_count;
                },
                MenuButtonAction::ChangeGenerationMethod(generation_method)=> {
                    planned_board_prop.generation_method = *generation_method;
                },
                MenuButtonAction::GenerateBoard=>{
                    input_event_writer.send(board_set_event::SpawnBoardWithNewSettings);
                    game_state.set(GameState::Game);
                },
                MenuButtonAction::ChangeWallTilesCount(wall_count_action)=> {
                    match wall_count_action{
                        WallTilesChange::Apply=> {
                            planned_board_prop.wall_count = unapplied_menu_wall_count.0;
                        },
                        WallTilesChange::Increase=> {
                            if unapplied_menu_wall_count.0 < planned_board_prop.size.wall_count_upper_bound(){
                                unapplied_menu_wall_count.0 += 1;
                            }else{
                                print_to_console::print_menu_error(
                                    MenuError::CantGoBeyondTileCountBounds(*wall_count_action)
                                );
                                break;
                            }
                        },
                        WallTilesChange::Decrease=> {
                            if unapplied_menu_wall_count.0 > 0{
                                unapplied_menu_wall_count.0 -= 1;
                            }else{
                                print_to_console::print_menu_error(
                                    MenuError::CantGoBeyondTileCountBounds(*wall_count_action)
                                );
                                break;
                            }
                        }
                    }
                }
            };

            
            match menu_button_action{
                MenuButtonAction::ChangeWallTilesCount(pending_change) => {
                    let (apply_button, mut apply_button_color) = apply_button_query.single_mut();
                    match pending_change{
                        WallTilesChange::Apply=> {
                            //doesn't change a thing if it's already chosen
                            commands.entity(entity).insert(SelectedOptionTag);
                        },
                        WallTilesChange::Increase | WallTilesChange::Decrease=> {
                            menu_graphics::set_color_to_normal(&mut apply_button_color);
                            commands.entity(apply_button).remove::<SelectedOptionTag>();
                            break; // shouldn't be printed to the console as no changes were applied
                        }
                    }
                },
                MenuButtonAction::GenerateBoard => {
                    // remove applied from previous settings
                    for previously_applied in currently_applied.iter_mut(){
                        commands.entity(previously_applied).remove::<AppliedOptionTag>();
                    }
                    // insert applied to the new settings
                    for (previous_button, _ , _ ) in currently_chosen.iter_mut(){
                        commands.entity(previous_button).insert(AppliedOptionTag);
                    }

                    // update applied props
                    let mut applied_props = applied_board_prop_query.single_mut();
                    *applied_props = *planned_board_prop;
                },
                _ => {
                    for (
                        previous_button, 
                        mut previous_color, 
                        menu_button_action_of_chosen
                    ) in currently_chosen.iter_mut(){
                        if mem::discriminant(menu_button_action) == mem::discriminant(menu_button_action_of_chosen){
                            menu_graphics::set_color_to_normal(&mut previous_color);
                            commands.entity(previous_button).remove::<SelectedOptionTag>();
                            commands.entity(entity).insert(SelectedOptionTag);
                        }  
                    }
                }
            }

            print_to_console::game_log(GameLog::BoardSettingsChanged(menu_button_action));
        }
    }
}

fn set_chosen_options_to_fit_current_props(
    mut event_listener: EventReader<SetMenuElementsToFitCurrent>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<AppliedOptionTag>)
    >,
    mut currently_applied: Query<
        (Entity, &mut BackgroundColor) , 
        (With<AppliedOptionTag>, Without<SelectedOptionTag>)
    >,
    mut commands: Commands
){
    for _event in event_listener.read(){
        // remove from previously chosen and not applied
        for (
            chosen_not_applied, 
            mut not_applied_button_color, 
            _
        ) in currently_chosen.iter_mut(){
            menu_graphics::set_color_to_normal(&mut not_applied_button_color);
            commands.entity(chosen_not_applied).remove::<SelectedOptionTag>();
        }

        // put the chosen mark in the currently applied ones
        for (
            should_be_marked_chosen,
            mut should_be_marked_button_color, 
        ) in currently_applied.iter_mut(){
            menu_graphics::set_color_to_pressed(&mut should_be_marked_button_color);
            commands.entity(should_be_marked_chosen).insert(SelectedOptionTag);
        }
    }
}