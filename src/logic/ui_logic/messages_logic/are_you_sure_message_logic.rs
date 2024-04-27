use crate::costume_event::ui_event::AreYouSureMessageButtonEvent;
use crate::prelude::*;


pub struct AreYouSureMessageLogicPlugin;

impl Plugin for AreYouSureMessageLogicPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    listen_for_loader_screen_actions,
                    listen_for_button_events
                )
            );
    }
}

fn set_are_you_sure_message_type_and_text(
    requested_new_type: AreYouSureMessageType,
    text_ref: &mut String,
    type_ref: &mut AreYouSureMessageType
){
    *text_ref = requested_new_type.to_string();
    *type_ref = requested_new_type;
}


fn listen_for_loader_screen_actions(
    mut visibility_toggle_event_writer: EventWriter<SetEntityVisibility>,
    mut event_reader: EventReader<LoaderScreenActionInitiated>,
    mut are_you_sure_message_query: Query<(Entity, &mut AreYouSureMessageType)>,
    mut are_you_sure_text_query: Query<&mut Text, With<AreYouSureMessageTextTag>>,
){
    for loader_screen_action in event_reader.read(){
        if let LoaderScreenAction::WarnBeforeDeletion(are_you_sure_message_requested_type) =
            loader_screen_action.action.clone()
        {
           let are_you_sure_text_ref =
               &mut are_you_sure_text_query.single_mut().sections[0].value;
            let (
                are_you_sure_message_entity, 
                mut are_you_sure_message_type
            ) = are_you_sure_message_query.single_mut();
            visibility_toggle_event_writer.send(SetEntityVisibility{
                entity: are_you_sure_message_entity,
                visibility: Visibility::Visible
            });
            set_are_you_sure_message_type_and_text(
                are_you_sure_message_requested_type,
                are_you_sure_text_ref,
                &mut are_you_sure_message_type
            )
        }
    }
}

fn listen_for_button_events(
    mut entity_visibility_event_writer: EventWriter<SetEntityVisibility>,
    mut clear_db_event_writer: EventWriter<ClearDB>,
    mut remove_from_db_event_writer: EventWriter<RemoveFromDB>,
    mut event_reader: EventReader<AreYouSureMessageButtonEvent>,
    are_you_sure_message_query: Query<(Entity, &AreYouSureMessageType)>,
){
    for action_request in event_reader.read(){
        let (
            are_you_sure_message_entity, 
            are_you_sure_message_type
        ) = are_you_sure_message_query.single();
        if let AreYouSureMessageButtonAction::Confirm = action_request.action{
            match are_you_sure_message_type{
                AreYouSureMessageType::DeleteAllBoards => {
                    clear_db_event_writer.send(ClearDB);
                },
                AreYouSureMessageType::DeleteBoard(optional_domain_board_to_delete) => {
                    if let Some((_ , saved_layout_index)) = optional_domain_board_to_delete {
                        remove_from_db_event_writer.send(RemoveFromDB(*saved_layout_index));
                    }
                } 
            }
        }
        entity_visibility_event_writer.send(SetEntityVisibility{
            entity: are_you_sure_message_entity,
            visibility: Visibility::Hidden
        });
    }
}