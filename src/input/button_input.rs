use crate::{prelude::*, costume_event::ui_event, output::print_to_console};

pub struct ButtonInputPlugin;

impl Plugin for ButtonInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,(
                    handle_menu_buttons, 
                )
                .run_if(in_state(GameState::Menu))
                .in_set(InputSystemSets::InputListening)
            )
            ;
    }
}


fn handle_menu_buttons(
    mut button_event_writer: EventWriter<ui_event::ButtonPressed>,
    mut apply_button_event_writer: EventWriter<ui_event::ApplyButtonPressed>,
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, Entity, Option<&ApplyButtonTag>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (
        interaction, 
        menu_button_action, 
        entity,
        optional_apply_button_tag
    ) 
    in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(_) = optional_apply_button_tag{
                apply_button_event_writer.send(ui_event::ApplyButtonPressed{
                    action: *menu_button_action
                });
            }else{
                button_event_writer.send(ui_event::ButtonPressed{
                    entity,
                    action: *menu_button_action
                });
            }

            match menu_button_action{
                MenuButtonAction::GenerateBoard | MenuButtonAction::ChangeWallTilesCount(_) => {},
                _ => {
                    print_to_console::game_log(GameLog::BoardSettingsChanged(menu_button_action));
                }
            }
        }
    }
}