use crate::{prelude::*, costume_event::reset_event};

pub const NORMAL_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
pub const HOVERED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
pub const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

/// Which option is currently selected
#[derive(Component)]
pub struct SelectedOptionTag;

#[derive(Component, Debug)]
pub enum MenuButtonAction{
    ChangeSize(BoardSize),
    ChangeWallTilesCount(u8),
    ChangeEmptyCount(u8),
    ChangeGenerationMethod,
    GenerateBoard
}


pub struct MenuGraphicsPlugin;

impl Plugin for MenuGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,(
                update_button_color,
                menu_action
                ).run_if(in_state(GameState::Menu)),
            );
    }
}


fn update_button_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOptionTag>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_action(
    mut input_event_writer: EventWriter<reset_event::ResetBoardLogic>,
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    mut currently_chosen: Query<(Entity, &mut BackgroundColor, &MenuButtonAction), With<SelectedOptionTag>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut board_size_res: ResMut<BoardSize>,
    mut commands: Commands
) {
    for (
        interaction, 
        menu_button_action, 
        mut entity
    ) 
    in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_button_action{
                MenuButtonAction::ChangeSize(board_size)=> {
                    *board_size_res = *board_size;

                    for (
                        previous_button, 
                        mut previous_color, 
                        menu_button_action_of_chosen
                    ) in currently_chosen.iter_mut(){
                        if let MenuButtonAction::ChangeSize(_) = menu_button_action_of_chosen {
                            *previous_color = NORMAL_BUTTON.into();
                            commands.entity(previous_button).remove::<SelectedOptionTag>();
                            commands.entity(entity).insert(SelectedOptionTag);
                        }  
                    }
                },
                MenuButtonAction::ChangeWallTilesCount(walls_count)=> {

                },
                MenuButtonAction::ChangeEmptyCount(empty_count)=> {

                },
                MenuButtonAction::ChangeGenerationMethod=> {

                },
                MenuButtonAction::GenerateBoard=>{
                    input_event_writer.send(reset_event::ResetBoardLogic{reroll_solved: true});
                    game_state.set(GameState::Game);
                }
            };

            game_log(GameLog::BoardSettingsChanged(menu_button_action));
        }
    }
}
