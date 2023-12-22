use crate::{prelude::*, costume_event::reset_event, output::{print_to_console, graphics::menu_graphics}};
use std::mem;

pub struct ButtonInputPlugin;

impl Plugin for ButtonInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,(handle_menu_buttons).run_if(in_state(GameState::Menu)),
            );
    }
}


fn handle_menu_buttons(
    mut input_event_writer: EventWriter<reset_event::ResetBoardLogic>,
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<ApplyButtonTag>)
        >,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut board_prop_res: ResMut<BoardProperties>,
    mut unapplied_to_prop_res: ResMut<UnappliedToBoardProperties>,
    mut commands: Commands
) {
    for (
        interaction, 
        menu_button_action, 
        entity
    ) 
    in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_button_action{
                MenuButtonAction::ChangeSize(new_board_size)=> {
                    board_prop_res.size = *new_board_size;
                },
                MenuButtonAction::ChangeEmptyTilesCount(new_empty_count)=> {
                    board_prop_res.empty_count = *new_empty_count;
                },
                MenuButtonAction::ChangeGenerationMethod(generation_method)=> {
                    board_prop_res.generation_method = *generation_method;
                },
                MenuButtonAction::GenerateBoard=>{
                    input_event_writer.send(reset_event::ResetBoardLogic{reroll_solved: true});
                    game_state.set(GameState::Game);
                },
                MenuButtonAction::ChangeWallTilesCount(wall_count_action)=> {
                    match wall_count_action{
                        WallTilesChange::Apply=> {
                            board_prop_res.wall_count = unapplied_to_prop_res.wall_count;
                        },
                        WallTilesChange::Increase=> {
                            unapplied_to_prop_res.wall_count += 1;
                        },
                        WallTilesChange::Decrease=> {
                            unapplied_to_prop_res.wall_count -= 1;
                        }
                    }
                }
            };

            if let MenuButtonAction::ChangeWallTilesCount(pending_change) = menu_button_action{
                let (apply_button, mut apply_button_color) = apply_button_query.single_mut();
                match pending_change{
                    WallTilesChange::Apply=> {
                        //doesn't change a thing if it's already chosen
                        commands.entity(entity).insert(SelectedOptionTag);
                    },
                    WallTilesChange::Increase | WallTilesChange::Decrease=> {
                        menu_graphics::set_color_to_normal(&mut apply_button_color);
                        commands.entity(apply_button).remove::<SelectedOptionTag>();
                    }
                }
            } else {
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

            print_to_console::game_log(GameLog::BoardSettingsChanged(menu_button_action));
        }
    }
}
