use crate::{
    costume_event::{app_event, ui_event},
    output::print_to_console,
    prelude::*,
};

pub struct ButtonInputPlugin;

impl Plugin for ButtonInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_menu_buttons.run_if(in_state(AppState::Menu)),
                handle_game_buttons.run_if(in_state(AppState::Game)),
                handle_eternal_buttons,
            )
                .in_set(InputSystemSets::InputListening),
        );
    }
}

fn handle_eternal_buttons(
    mut end_game_event_writer: EventWriter<app_event::EndGame>,
    mut menu_toggle_event_writer: EventWriter<app_event::ToggleMenu>,
    interaction_query: Query<
        (&Interaction, &EternalButtonAction),
        (Changed<Interaction>, With<EternalButton>),
    >,
) {
    for (interaction, eternal_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match eternal_button_action {
                EternalButtonAction::EndGame => end_game_event_writer.send(app_event::EndGame),
                EternalButtonAction::ToggleMenu => {
                    menu_toggle_event_writer.send(app_event::ToggleMenu);
                }
            }
        }
    }
}

fn handle_menu_buttons(
    mut button_event_writer: EventWriter<ui_event::MenuButtonPressed>,
    mut apply_button_event_writer: EventWriter<ui_event::ApplyButtonPressed>,
    mut reset_button_text_color_event_writer: EventWriter<ui_event::ResetButtonTextColor>,
    interaction_query: Query<
        (
            &Interaction,
            &MenuButtonAction,
            Entity,
            Option<&ApplyButtonTag>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, menu_button_action, entity, optional_apply_button_tag) in
        interaction_query.iter()
    {
        if *interaction == Interaction::Pressed {
            if optional_apply_button_tag.is_some() {
                apply_button_event_writer.send(ui_event::ApplyButtonPressed {
                    action: *menu_button_action,
                });
            } else {
                button_event_writer.send(ui_event::MenuButtonPressed {
                    entity,
                    action: *menu_button_action,
                });
            }

            // if any button was pressed, turn back all the ui text that was turned red
            reset_button_text_color_event_writer.send(ui_event::ResetButtonTextColor);

            match menu_button_action {
                MenuButtonAction::GenerateBoard | MenuButtonAction::ChangeWallTilesCount(_) => {}
                _ => {
                    print_to_console::game_log(GameLog::BoardSettingsChanged(menu_button_action));
                }
            }
        }
    }
}

fn handle_game_buttons(
    mut button_event_writer: EventWriter<ui_event::GameButtonPressed>,
    mut reset_button_text_color_event_writer: EventWriter<ui_event::ResetButtonTextColor>,
    interaction_query: Query<
        (
            &Interaction,
            &GameButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, game_button_action) in
        interaction_query.iter()
    {
        if *interaction == Interaction::Pressed {
            button_event_writer.send(ui_event::GameButtonPressed {
                action: *game_button_action
            });

            // if any button was pressed, turn back all the ui text that was turned red
            reset_button_text_color_event_writer.send(ui_event::ResetButtonTextColor);
        }
    }
}
