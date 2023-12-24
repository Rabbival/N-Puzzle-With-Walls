use crate::{prelude::*, costume_event::{ui_event, board_set_event}, output::{print_to_console, graphics::menu_graphics}};
use std::mem;

pub struct UpdateBoardPropertiesPlugin;

impl Plugin for UpdateBoardPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                        general_update_planned_board_properties,
                        update_wall_count,
                        set_applied_props, 
                        begin_generation_and_game
                    )
                    .chain()
                    .run_if(in_state(GameState::Menu))
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
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<ApplyButtonTag>)
    >,
    mut commands: Commands,
){
    for button_event in button_event_listener.read(){
        let mut planned_board_prop = planned_board_prop_query.single_mut();
        let menu_button_action = button_event.action;
        let pressed_button_entity = button_event.entity;
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

        for (
            previous_button, 
            mut previous_color, 
            menu_button_action_of_chosen
        ) in currently_chosen.iter_mut(){
            if mem::discriminant(&menu_button_action) == mem::discriminant(menu_button_action_of_chosen){
                menu_graphics::set_color_to_normal(&mut previous_color);
                commands.entity(previous_button).remove::<SelectedOptionTag>();
                commands.entity(pressed_button_entity).insert(SelectedOptionTag);
            }  
        }
    }
}

fn update_wall_count(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut planned_board_prop_query: Query<
        &mut BoardProperties, 
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
    mut commands: Commands,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action{
            let mut planned_board_prop = planned_board_prop_query.single_mut();
            let (apply_button, mut apply_button_color) = apply_button_query.single_mut();
            match wall_count_action{
                WallTilesChange::Apply=> {
                    planned_board_prop.wall_count = unapplied_menu_wall_count.0;
                    commands.entity(button_event.entity).insert(SelectedOptionTag);
                },
                WallTilesChange::Increase | WallTilesChange::Decrease=> {
                    menu_graphics::set_color_to_normal(&mut apply_button_color);
                    commands.entity(apply_button).remove::<SelectedOptionTag>();

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


fn set_applied_props(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<ApplyButtonTag>)
    >,
    mut currently_applied: Query<
        Entity,
        (With<AppliedOptionTag>, Without<SelectedOptionTag>)
    >,
    mut applied_board_prop_query: Query<
        &mut BoardProperties, 
        (With<AppliedBoardProperties>, Without<PlannedBoardProperties>)
    >,
    mut planned_board_prop_query: Query<
        &mut BoardProperties, 
        (With<PlannedBoardProperties>, Without<AppliedBoardProperties>)
    >,
    mut commands: Commands,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::GenerateBoard = button_event.action{
            let planned_board_prop = planned_board_prop_query.single_mut();
            let mut applied_props = applied_board_prop_query.single_mut();
            *applied_props = *planned_board_prop;

            // remove applied from previous settings
            for previously_applied in currently_applied.iter_mut(){
                commands.entity(previously_applied).remove::<AppliedOptionTag>();
            }
            // insert applied to the new settings
            for (previous_button, _ , _ ) in currently_chosen.iter_mut(){
                commands.entity(previous_button).insert(AppliedOptionTag);
            }
        }
    }
}

fn begin_generation_and_game(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut spawn_board_event_writer: EventWriter<board_set_event::SpawnBoardWithNewSettings>,
    mut game_state: ResMut<NextState<GameState>>,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::GenerateBoard = button_event.action{
            spawn_board_event_writer.send(board_set_event::SpawnBoardWithNewSettings);
            game_state.set(GameState::Game);
            print_to_console::game_log(GameLog::NewBoardGenerated);
        }
    }
}