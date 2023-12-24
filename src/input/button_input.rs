use crate::{prelude::*, costume_event::ui_event, output::{print_to_console, graphics::menu_graphics}};
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
    mut button_event_writer: EventWriter<ui_event::ButtonPressed>,
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, Entity),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (
        interaction, 
        menu_button_action, 
        entity
    ) 
    in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            button_event_writer.send(ui_event::ButtonPressed{
                entity,
                action: *menu_button_action
            });

            if let MenuButtonAction::GenerateBoard = menu_button_action{
                continue;
            }
            if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = menu_button_action{
                if mem::discriminant(wall_count_action) != mem::discriminant(&WallTilesChange::Apply){
                    continue;
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