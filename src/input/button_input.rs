use crate::{prelude::*, costume_event::ui_event, output::print_to_console};
use std::mem;

pub struct ButtonInputPlugin;

impl Plugin for ButtonInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,(
                    handle_menu_buttons, 
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