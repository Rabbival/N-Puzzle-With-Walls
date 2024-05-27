use crate::prelude::*;

pub struct PopUpMessageLogicPlugin;

impl Plugin for PopUpMessageLogicPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    listen_to_newborn_domain_board_change_requests.run_if(in_state(AppState::Game)),
                    listen_for_allow_player_to_set_board_name_requests,
                    listen_for_loader_screen_actions,
                    listen_for_delete_related_button_events,
                    listen_for_db_related_button_events
                )
            );
    }
}

fn listen_to_newborn_domain_board_change_requests(
    mut event_writer: EventWriter<UpdateNewbornDomainBoardName>,
    mut event_reader: EventReader<KeyboardKeyTypedEvent>,
    mut text_above_pop_up_buttons_query: Query<
        &mut Text,
        (With<TextAbovePopUpMessageButtons>, Without<PopUpMessageDynamicTextTag>)
    >,
    pop_up_dynamic_text_query: Query<
        &Text,
        (With<PopUpMessageDynamicTextTag>, Without<TextAbovePopUpMessageButtons>)
    >,
){
    for key_typed in event_reader.read(){
        let pop_up_dynamic_text = &mut pop_up_dynamic_text_query.single().sections[0].value;
        match key_typed.0{
            KeyCode::Backspace => {
                if pop_up_dynamic_text.len() > 0 {
                    event_writer.send(UpdateNewbornDomainBoardName(
                        DomainBoardName(String::from(&pop_up_dynamic_text[..pop_up_dynamic_text.len()-1]))
                    ));
                }
            }
            keycode if valid_key_for_name(keycode) => {
                if pop_up_dynamic_text.len() < MAX_DOMAIN_BOARD_NAME_LENGTH {
                    event_writer.send(UpdateNewbornDomainBoardName(
                        DomainBoardName(format!("{}{:?}",pop_up_dynamic_text,keycode))
                    ));
                }else{
                    set_text_section_value_and_color(
                        &mut text_above_pop_up_buttons_query.single_mut().sections[0],
                        None,
                        Some(TextAbovePopUpButtonsType::CantHaveALongerName.to_string())
                    );
                }
            }
            _ => {}
        }
    }
}

fn listen_for_allow_player_to_set_board_name_requests(
    mut visibility_toggle_event_writer: EventWriter<SetEntityVisibility>,
    mut event_reader: EventReader<AllowPlayerToSetBoardName>,
    mut pop_up_message_query: Query<(Entity, &mut PopUpMessageType)>,
    pop_up_dynamic_text_entity_query: Query<Entity, With<PopUpMessageDynamicTextTag>>,
    mut text_above_pop_up_buttons_query: Query<&mut Text, (With<TextAbovePopUpMessageButtons>, Without<PopUpMessageTextTag>)>,
    mut pop_up_text_query: Query<&mut Text, (With<PopUpMessageTextTag>, Without<TextAbovePopUpMessageButtons>)>,
){
    for _event in event_reader.read(){
        set_pop_up_message_text_and_type(
            PopUpMessageType::ChooseNewbornDomainBoardName,
            &mut visibility_toggle_event_writer,
            &mut pop_up_message_query,
            &mut pop_up_text_query,
        );
        visibility_toggle_event_writer.send(SetEntityVisibility{
            entity: pop_up_dynamic_text_entity_query.single(),
            visibility: Visibility::Inherited
        });
        set_text_section_value_and_color(
            &mut text_above_pop_up_buttons_query.single_mut().sections[0],
            None,
            Some(TextAbovePopUpButtonsType::BoardNameAlreadyExists.to_string())
        );
    }
}

fn listen_for_loader_screen_actions(
    mut visibility_toggle_event_writer: EventWriter<SetEntityVisibility>,
    mut event_reader: EventReader<LoaderScreenActionEvent>,
    mut pop_up_message_query: Query<(Entity, &mut PopUpMessageType)>,
    pop_up_dynamic_text_entity_query: Query<Entity, With<PopUpMessageDynamicTextTag>>,
    mut text_above_pop_up_buttons_query: Query<&mut Text, (With<TextAbovePopUpMessageButtons>, Without<PopUpMessageTextTag>)>,
    mut pop_up_text_query: Query<&mut Text, (With<PopUpMessageTextTag>, Without<TextAbovePopUpMessageButtons>)>,
){
    for loader_screen_action in event_reader.read(){
        if let LoaderScreenAction::WarnBeforeDeletion(pop_up_message_requested_type) =
            loader_screen_action.action.clone()
        {
            set_pop_up_message_text_and_type(
                pop_up_message_requested_type,
                &mut visibility_toggle_event_writer,
                &mut pop_up_message_query,
                &mut pop_up_text_query,
            );
            visibility_toggle_event_writer.send(SetEntityVisibility{
                entity: pop_up_dynamic_text_entity_query.single(),
                visibility: Visibility::Hidden
            });
            set_text_section_value_and_color(
                &mut text_above_pop_up_buttons_query.single_mut().sections[0],
                None,
                Some(TextAbovePopUpButtonsType::NoText.to_string())
            );
        }
    }
}

fn set_pop_up_message_text_and_type(
    requested_type: PopUpMessageType,
    visibility_toggle_event_writer: &mut EventWriter<SetEntityVisibility>,
    pop_up_message_query: &mut Query<(Entity, &mut PopUpMessageType)>,
    pop_up_text_query: &mut Query<&mut Text, (With<PopUpMessageTextTag>, Without<TextAbovePopUpMessageButtons>)>,
){
    let pop_up_text_ref =
        &mut pop_up_text_query.single_mut().sections[0].value;
    let (
        pop_up_message_entity,
        mut pop_up_message_type
    ) = pop_up_message_query.single_mut();
    visibility_toggle_event_writer.send(SetEntityVisibility{
        entity: pop_up_message_entity,
        visibility: Visibility::Inherited
    });
    set_are_you_sure_message_type_and_text(
        requested_type,
        pop_up_text_ref,
        pop_up_message_type.as_mut()
    );
}

fn listen_for_delete_related_button_events(
    mut entity_visibility_event_writer: EventWriter<SetEntityVisibility>,
    mut clear_db_event_writer: EventWriter<ClearDB>,
    mut remove_from_db_event_writer: EventWriter<RemoveFromDB>,
    mut event_reader: EventReader<PopUpMessageButtonEvent>,
    pop_up_message_query: Query<(Entity, &PopUpMessageType)>,
){
    for action_request in event_reader.read(){
        let (
            pop_up_message_entity,
            pop_up_message_type
        ) = pop_up_message_query.single();
        if let PopUpMessageButtonAction::Confirm = action_request.action{
            match pop_up_message_type{
                PopUpMessageType::DeleteAllBoards => {
                    clear_db_event_writer.send(ClearDB);
                },
                PopUpMessageType::DeleteBoard(optional_domain_board_to_delete) => {
                    if let Some((_ , saved_layout_index)) = optional_domain_board_to_delete {
                        remove_from_db_event_writer.send(RemoveFromDB(*saved_layout_index));
                    }
                },
                _ => {}
            }
        }
        entity_visibility_event_writer.send(SetEntityVisibility{
            entity: pop_up_message_entity,
            visibility: Visibility::Hidden
        });
    }
}

fn listen_for_db_related_button_events(
    mut save_to_db_event_writer: EventWriter<SaveToDB>,
    mut event_reader: EventReader<PopUpMessageButtonEvent>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    pop_up_message_query: Query<&PopUpMessageType>,
    mut game_board_query: Query<&mut TileBoard, With<GameBoard>>,
    newborn_domain_board_name_res: Res<NewbornDomainBoardName>,
){
    for action_request in event_reader.read(){
        let pop_up_message_type = pop_up_message_query.single();
        match pop_up_message_type{
            PopUpMessageType::ChooseNewbornDomainBoardName => {
                if let PopUpMessageButtonAction::Confirm = action_request.action{
                    if let Some(newborn_domain_board_name) = &newborn_domain_board_name_res.0{
                        save_to_db_event_writer.send(SaveToDB(
                            DomainBoard{
                                board_props: *applied_board_props_query.single(),
                                grid: game_board_query.single().grid.clone()
                            },
                            newborn_domain_board_name.clone()
                        ));
                    }
                }
                game_board_query.single_mut().ignore_player_input = false;
            }
            _ => {}
        }
    }
}