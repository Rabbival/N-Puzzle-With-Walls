use crate::input::keyboard_utilities::try_get_string_from_keycode;
use crate::prelude::*;

pub struct PopUpMessageLogicPlugin;

impl Plugin for PopUpMessageLogicPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    listen_for_set_confirm_allowed_requests,
                    (
                        listen_for_show_pop_up_to_set_newborn_board_name_requests,
                        listen_for_newborn_domain_board_change_requests.in_set(InputSystemSets::InitialChanges),
                        set_newborn_board_displayed_name_and_message
                            .before(set_pop_up_dynamic_text_box_color)
                            .in_set(InputSystemSets::MainChanges)
                    ).run_if(in_state(AppState::Game)),
                    listen_for_loader_screen_actions,
                    listen_for_delete_related_button_events,
                    listen_for_db_related_button_events,
                )
            );
    }
}

fn listen_for_set_confirm_allowed_requests(
    mut entity_visibility_event_writer: EventWriter<SetEntityVisibility>,
    mut event_reader: EventReader<SetConfirmAllowed>,
    pop_up_buttons_query: Query<(Entity, &PopUpMessageButtonAction)>,
    mut confirm_allowed_query: Query<&mut ConfirmAllowed, With<PopUpMessageType>>,
    pop_up_dynamic_text_query: Query<&Text, With<PopUpMessageDynamicTextTag>>,
    mut newborn_domain_board_name: ResMut<NewbornDomainBoardName>,
){
    for request in event_reader.read(){
        if let Err(entity_error) =
            listen_for_set_confirm_allowed_requests_inner(
                request,
                &mut entity_visibility_event_writer,
                &pop_up_buttons_query,
                &mut confirm_allowed_query,
                &pop_up_dynamic_text_query,
                &mut newborn_domain_board_name,
            ) {
                print_entity_related_error(entity_error);
            }
    }
}

fn listen_for_set_confirm_allowed_requests_inner(
    set_confirm_allowed_request: &SetConfirmAllowed,
    entity_visibility_event_writer: &mut EventWriter<SetEntityVisibility>,
    pop_up_buttons_query: &Query<(Entity, &PopUpMessageButtonAction)>,
    confirm_allowed_query: &mut Query<&mut ConfirmAllowed, With<PopUpMessageType>>,
    pop_up_dynamic_text_query: &Query<&Text, With<PopUpMessageDynamicTextTag>>,
    newborn_domain_board_name: &mut ResMut<NewbornDomainBoardName>,
) -> Result<(), EntityRelatedCostumeError>{
    let optional_confirm_button =
        try_get_confirm_button_entity(pop_up_buttons_query);
    match optional_confirm_button {
        None => Err(EntityRelatedCostumeError::EntityNotInQuery),
        Some(confirm_button_entity) => {
            confirm_allowed_query.single_mut().0 = set_confirm_allowed_request.0;
            if set_confirm_allowed_request.0 {
                entity_visibility_event_writer.send(SetEntityVisibility{
                    entity: confirm_button_entity,
                    visibility: Visibility::Inherited
                });
                set_newborn_domain_board_name_res_to_displayed(
                    pop_up_dynamic_text_query,
                    newborn_domain_board_name
                );
            }else{
                entity_visibility_event_writer.send(SetEntityVisibility{
                    entity: confirm_button_entity,
                    visibility: Visibility::Hidden
                });
                newborn_domain_board_name.optional_name = None;
            }
            Ok(())
        }
    }
}

fn set_newborn_domain_board_name_res_to_displayed(
    pop_up_dynamic_text_query: &Query<&Text, With<PopUpMessageDynamicTextTag>>,
    newborn_domain_board_name: &mut ResMut<NewbornDomainBoardName>,
){
    let currently_displayed_name =
        DomainBoardName(pop_up_dynamic_text_query.single().sections[0].value.clone());
    newborn_domain_board_name.optional_name = Some(currently_displayed_name);
}

fn listen_for_newborn_domain_board_change_requests(
    mut update_name_event_writer: EventWriter<UpdateNewbornDomainBoardName>,
    mut event_reader: EventReader<KeyboardKeyTypedEvent>,
    mut text_above_pop_up_buttons_query: Query<
        &mut Text,
        (With<TextAbovePopUpMessageButtons>, Without<PopUpMessageDynamicTextTag>)
    >,
    pop_up_dynamic_text_query: Query<
        (&BackgroundColor, &Text),
        (With<PopUpMessageDynamicTextTag>, Without<TextAbovePopUpMessageButtons>)
    >,
){
    for key_typed in event_reader.read(){
        let pop_up_dynamic_text = &pop_up_dynamic_text_query.single().1.sections[0].value;
        let pop_up_text_background_color = pop_up_dynamic_text_query.single().0;
        let first_set_since_default = pop_up_text_background_color.0 == GRAY_TEXT_COLOR;
        match key_typed.keycode{
            KeyCode::Backspace | KeyCode::Delete => {
                if !pop_up_dynamic_text.is_empty() {
                    shorten_name(&mut update_name_event_writer, pop_up_dynamic_text, first_set_since_default);
                }
            }
            keycode => {
                if let Some(parsed_keycode) = try_get_string_from_keycode(keycode, key_typed.shift_pressed){
                    if pop_up_dynamic_text.len() < MAX_DOMAIN_BOARD_NAME_LENGTH {
                        if first_set_since_default {
                            update_name_event_writer.send(UpdateNewbornDomainBoardName(
                                DomainBoardName(parsed_keycode)
                            ));
                        }else{
                            update_name_event_writer.send(UpdateNewbornDomainBoardName(
                                DomainBoardName(format!("{}{}",pop_up_dynamic_text,parsed_keycode))
                            ));
                        }
                    }else{
                        set_text_section_value_and_color(
                            &mut text_above_pop_up_buttons_query.single_mut().sections[0],
                            None,
                            Some(TextAbovePopUpButtonsType::CantHaveALongerName.to_string())
                        );
                    }
                }
            }
        }
    }
}

fn shorten_name(
    update_name_event_writer: &mut EventWriter<UpdateNewbornDomainBoardName>,
    pop_up_dynamic_text: &str,
    first_set_since_default: bool
){
    let shortened_name = if first_set_since_default{
        ""
    }else{
        &pop_up_dynamic_text[..pop_up_dynamic_text.len()-1]
    };
    update_name_event_writer.send(UpdateNewbornDomainBoardName(
        DomainBoardName(String::from(shortened_name))
    ));
}


fn set_newborn_board_displayed_name_and_message(
    mut event_writer: EventWriter<SetConfirmAllowed>,
    mut event_reader: EventReader<UpdateNewbornDomainBoardName>,
    mut pop_up_dynamic_text_query: Query<&mut Text, (With<PopUpMessageDynamicTextTag>, Without<TextAbovePopUpMessageButtons>)>,
    mut text_above_pop_up_buttons_entity_query: Query<&mut Text, (With<TextAbovePopUpMessageButtons>, Without<PopUpMessageDynamicTextTag>)>,
    newborn_domain_board_name: Res<NewbornDomainBoardName>
){
    for name_request in event_reader.read() {
        let requested_name = name_request.0.clone();
        let pop_up_dynamic_text =
            &mut pop_up_dynamic_text_query.single_mut().sections[0];
        let text_above_pop_up_buttons =
            &mut text_above_pop_up_buttons_entity_query.single_mut().sections[0];

        set_text_section_value_and_color(
            pop_up_dynamic_text,
            None,
            Some(requested_name.0.clone())
        );


        println!("existing board index: {:?}", newborn_domain_board_name.index_of_existing_board_with_name);


        if requested_name.0.is_empty(){
            set_text_section_value_and_color(
                text_above_pop_up_buttons,
                None,
                Some(TextAbovePopUpButtonsType::MustGiveAName.to_string())
            );
            event_writer.send(SetConfirmAllowed(false));
        }else if newborn_domain_board_name.index_of_existing_board_with_name.is_some(){
            set_text_section_value_and_color(
                text_above_pop_up_buttons,
                None,
                Some(TextAbovePopUpButtonsType::BoardNameAlreadyExists.to_string())
            );
            event_writer.send(SetConfirmAllowed(true));
        }else{
            set_text_section_value_and_color(
                text_above_pop_up_buttons,
                None,
                Some(TextAbovePopUpButtonsType::NoText.to_string())
            );
            event_writer.send(SetConfirmAllowed(true));
        }
    }
}

fn listen_for_show_pop_up_to_set_newborn_board_name_requests(
    mut visibility_toggle_event_writer: EventWriter<SetEntityVisibility>,
    mut event_reader: EventReader<SetNewbornDomainBoardNameToDefault>,
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
            Some(TextAbovePopUpButtonsType::NoText.to_string())
        );
    }
}

fn listen_for_loader_screen_actions(
    mut allow_board_name_setting_event_writer: EventWriter<SetConfirmAllowed>,
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
            allow_board_name_setting_event_writer.send(SetConfirmAllowed(true));
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
                PopUpMessageType::DeleteBoard(Some((_ , saved_layout_index))) => {
                    remove_from_db_event_writer.send(RemoveFromDB(*saved_layout_index));
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
        if *pop_up_message_query.single() == PopUpMessageType::ChooseNewbornDomainBoardName{
            if let PopUpMessageButtonAction::Confirm = action_request.action{
                if let Some(newborn_domain_board_name) = &newborn_domain_board_name_res.optional_name{
                    save_to_db_event_writer.send(SaveToDB{
                        board: DomainBoard{
                            board_props: *applied_board_props_query.single(),
                            grid: game_board_query.single().grid.clone()
                        },
                        name: newborn_domain_board_name.clone(),
                        index_of_existing_board_with_name: newborn_domain_board_name_res.index_of_existing_board_with_name
                    });
                }
            }
            game_board_query.single_mut().ignore_player_input = false;
        }
    }
}

fn try_get_confirm_button_entity(buttons_query: &Query<(Entity, &PopUpMessageButtonAction)>) -> Option<Entity>{
    for (button_entity, button_action) in buttons_query{
        if let PopUpMessageButtonAction::Confirm = button_action{
            return Some(button_entity);
        }
    }
    None
}