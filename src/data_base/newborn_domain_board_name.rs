use crate::prelude::*;


pub const MAX_DOMAIN_BOARD_NAME_LENGTH: usize = 22;

#[derive(Resource, Default)]
pub struct NewbornDomainBoardName{
    pub optional_name: Option<DomainBoardName>,
    pub already_exists: bool
}

pub struct NewbornDomainBoardNamePlugin;

impl Plugin for NewbornDomainBoardNamePlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NewbornDomainBoardName>()
            .add_systems(Update, generate_default);
    }
}

fn generate_default(
    mut event_writer: EventWriter<SetConfirmAllowed>,
    mut event_reader: EventReader<SetNewbornDomainBoardNameToDefault>,
    domain_board_names_query: Query<&DomainBoardName>,
    mut text_above_pop_up_buttons_query: Query<&mut Text, (With<TextAbovePopUpMessageButtons>, Without<PopUpMessageDynamicTextTag>)>,
    mut pop_up_dynamic_text_query: Query<&mut Text, (With<PopUpMessageDynamicTextTag>, Without<TextAbovePopUpMessageButtons>)>,
    db_manager: Res<DataBaseManager>
){
    for _event in event_reader.read(){
        let pop_up_dynamic_text = 
            &mut pop_up_dynamic_text_query.single_mut().sections[0];
        let text_above_pop_up_buttons = 
            &mut text_above_pop_up_buttons_query.single_mut().sections[0];
        event_writer.send(SetConfirmAllowed(true));
        let default_name =
            db_manager.generate_unique_default_name_for_board(&domain_board_names_query);
        set_text_section_value_and_color(
            pop_up_dynamic_text,
            None,
            Some(default_name.0.clone())
        );

        set_text_section_value_and_color(
            text_above_pop_up_buttons,
            None,
            Some(TextAbovePopUpButtonsType::NoText.to_string())
        );
    }
}