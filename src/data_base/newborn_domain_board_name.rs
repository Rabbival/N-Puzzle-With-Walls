use crate::prelude::*;


pub const MAX_DOMAIN_BOARD_NAME_LENGTH: usize = 22;

#[derive(Resource, Default)]
pub struct NewbornDomainBoardName(pub Option<DomainBoardName>);

pub struct NewbornDomainBoardNamePlugin;

impl Plugin for NewbornDomainBoardNamePlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NewbornDomainBoardName>()
            .add_systems(Update, (
                generate_default,
                set_newborn_domain_board_name
                    .after(listen_to_newborn_domain_board_change_requests)
                    .before(set_pop_up_dynamic_text_box_color)
            ));
    }
}

fn generate_default(
    mut event_reader: EventReader<AllowPlayerToSetBoardName>,
    domain_board_names_query: Query<&DomainBoardName>,
    mut text_above_pop_up_buttons_query: Query<&mut Text, (With<TextAbovePopUpMessageButtons>, Without<PopUpMessageDynamicTextTag>)>,
    mut pop_up_dynamic_text_query: Query<&mut Text, (With<PopUpMessageDynamicTextTag>, Without<TextAbovePopUpMessageButtons>)>,
    mut newborn_domain_board_name: ResMut<NewbornDomainBoardName>,
    db_manager: Res<DataBaseManager>
){
    for _event in event_reader.read(){
        let default_name =
            db_manager.generate_unique_default_name_for_board(&domain_board_names_query);
        set_displayed_and_saved_newborn_name(
            default_name,
            &mut pop_up_dynamic_text_query.single_mut(),
            &mut newborn_domain_board_name.0,
        );
        set_text_section_value_and_color(
            &mut text_above_pop_up_buttons_query.single_mut().sections[0],
            None,
            Some(TextAbovePopUpButtonsType::NoText.to_string())
        );
    }
}

fn set_newborn_domain_board_name(
    mut entity_visibility_event_writer: EventWriter<SetEntityVisibility>,
    mut event_reader: EventReader<UpdateNewbornDomainBoardName>,
    domain_board_names_query: Query<&DomainBoardName>,
    mut pop_up_dynamic_text_query: Query<&mut Text, (With<PopUpMessageDynamicTextTag>, Without<TextAbovePopUpMessageButtons>)>,
    mut newborn_domain_board_name: ResMut<NewbornDomainBoardName>,
    mut text_above_pop_up_buttons_entity_query: Query<&mut Text, (With<TextAbovePopUpMessageButtons>, Without<PopUpMessageDynamicTextTag>)>,
    pop_up_buttons_query: Query<(Entity, &PopUpMessageButtonAction)>
){
    for name_request in event_reader.read(){
        if let Err(entity_error) = set_newborn_domain_board_name_inner(
            &mut entity_visibility_event_writer,
            &name_request.0,
            &domain_board_names_query,
            &mut pop_up_dynamic_text_query.single_mut(),
            &mut newborn_domain_board_name,
            text_above_pop_up_buttons_entity_query.single_mut().as_mut(),
            &pop_up_buttons_query
        ) {
            print_entity_related_error(entity_error);
        }
    }
}

fn set_newborn_domain_board_name_inner(
    entity_visibility_event_writer: &mut EventWriter<SetEntityVisibility>,
    requested_name: &DomainBoardName,
    domain_board_names_query: &Query<&DomainBoardName>,
    mut pop_up_dynamic_text: &mut Text,
    newborn_domain_board_name: &mut ResMut<NewbornDomainBoardName>,
    text_above_pop_up_buttons: &mut Text,
    pop_up_buttons_query: &Query<(Entity, &PopUpMessageButtonAction)>
) -> Result<(), EntityRelatedCostumeError>{
    let mut optional_confirm_button = None;
    for (button_entity, button_action) in pop_up_buttons_query{
        if let PopUpMessageButtonAction::Confirm = button_action{
            optional_confirm_button = Some(button_entity);
            break;
        }
    }
    match optional_confirm_button{
        None => Err(EntityRelatedCostumeError::EntityNotInQuery),
        Some(confirm_button_entity) => {
            set_displayed_and_saved_newborn_name(
                requested_name.clone(),
                &mut pop_up_dynamic_text,
                &mut newborn_domain_board_name.0,
            );
            
            if DataBaseManager::domain_board_name_already_exists(
                requested_name,
                &domain_board_names_query
            ){
                entity_visibility_event_writer.send(SetEntityVisibility{
                    entity: confirm_button_entity,
                    visibility: Visibility::Hidden
                });
                set_text_section_value_and_color(
                    &mut text_above_pop_up_buttons.sections[0],
                    None,
                    Some(TextAbovePopUpButtonsType::BoardNameAlreadyExists.to_string())
                );
                newborn_domain_board_name.0 = None;
            }else{
                entity_visibility_event_writer.send(SetEntityVisibility{
                    entity: confirm_button_entity,
                    visibility: Visibility::Inherited
                });
            }
            Ok(())
        }
    }
}

fn set_displayed_and_saved_newborn_name(
    name_to_set_to: DomainBoardName,
    displayed_newborn_name: &mut Text,
    saved_newborn_name: &mut Option<DomainBoardName>
){
    *saved_newborn_name = Some(name_to_set_to.clone());
    set_text_section_value_and_color(
        &mut displayed_newborn_name.sections[0],
        None,
        Some(name_to_set_to.0.clone())
    );
}