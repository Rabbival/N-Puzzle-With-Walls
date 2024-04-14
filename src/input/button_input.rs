use crate::prelude::*;

pub struct ButtonInputPlugin;

impl Plugin for ButtonInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_menu_buttons.run_if(in_state(AppState::Menu)),
                handle_victory_buttons.run_if(in_state(GameState::Victory)),
                handle_eternal_buttons,
            )
                .in_set(InputSystemSets::InputListening),
        );
    }
}

fn handle_eternal_buttons(
    mut end_game_event_writer: EventWriter<EndGame>,
    mut menu_toggle_event_writer: EventWriter<ToggleMenu>,
    interaction_query: Query<
        (&Interaction, &EternalButtonAction),
        (Changed<Interaction>, With<EternalButton>),
    >,
) {
    for (interaction, eternal_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match eternal_button_action {
                EternalButtonAction::EndGame => end_game_event_writer.send(EndGame),
                EternalButtonAction::ToggleMenu => {
                    menu_toggle_event_writer.send(ToggleMenu);
                }
            }
        }
    }
}

fn handle_menu_buttons(
    mut button_event_writer: EventWriter<MenuButtonPressed>,
    mut apply_button_event_writer: EventWriter<ApplyButtonPressed>,
    mut reset_button_text_color_event_writer: EventWriter<ResetButtonTextColor>,
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
                apply_button_event_writer.send(ApplyButtonPressed {
                    action: *menu_button_action,
                });
            } else {
                button_event_writer.send(MenuButtonPressed {
                    entity,
                    action: *menu_button_action,
                });
            }
            
            reset_button_text_color_event_writer.send(ResetButtonTextColor);

            match menu_button_action {
                MenuButtonAction::MainButtonPressed | MenuButtonAction::ChangeWallTilesCount(_) => {}
                _ => {
                    game_log(GameLog::BoardSettingsChanged(menu_button_action));
                }
            }
        }
    }
}

fn handle_victory_buttons(
    mut button_event_writer: EventWriter<VictoryButtonPressed>,
    mut reset_button_text_color_event_writer: EventWriter<ResetButtonTextColor>,
    interaction_query: Query<
        (
            &Interaction,
            &VictoryButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, game_button_action) in
        interaction_query.iter()
    {
        if *interaction == Interaction::Pressed {
            button_event_writer.send(VictoryButtonPressed {
                action: *game_button_action
            });

            reset_button_text_color_event_writer.send(ResetButtonTextColor);
        }
    }
}
