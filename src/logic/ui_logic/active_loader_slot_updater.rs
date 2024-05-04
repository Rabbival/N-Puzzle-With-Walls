use crate::prelude::*;

pub struct ActiveLoaderSlotUpdaterPlugin;

impl Plugin for ActiveLoaderSlotUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, (
                show_slot,
                set_slot_text,
                set_slot_layout_preview
            ).after(show_currently_displayed_saved_layouts_screen)
        );
    }
}

fn show_slot(
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    mut loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag)>,
){
    for slot_set_request in event_reader.read(){
        for (loader_action, mut layout_slot_on_screen_tag)
            in loader_screen_actions_query.iter_mut()
        {
            if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action {
                if layout_slot == slot_set_request.slot_to_set {
                    layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Visible);
                }
            }
        }
    }
}

fn set_slot_text(
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &Children)>,
    mut layout_slot_text_query: Query<&mut Text>,
    domain_board_query: Query<(&DomainBoard, &DomainBoardName)>,
){
    for loader_slot_set_request in event_reader.read(){
        match domain_board_query.get(loader_slot_set_request.layout_entity){
            Ok((domain_board, domain_board_name)) => {
                if let Err(entity_error) = set_slot_text_inner(
                    domain_board,
                    domain_board_name,
                    loader_slot_set_request.slot_to_set,
                    &loader_screen_actions_query,
                    &mut layout_slot_text_query,
                ){
                    print_entity_related_error(entity_error);
                }
            },
            Err(_query_entity_error) => print_entity_related_error(EntityRelatedCostumeError::EntityNotInQuery)
        };
    }
}

fn set_slot_text_inner(
    domain_board_to_set_text_to: &DomainBoard,
    domain_board_name_to_set_text_to: &DomainBoardName,
    slot_to_set: LoaderScreenSlot,
    loader_screen_actions_query: &Query<(&LoaderScreenAction, &Children)>,
    layout_slot_text_query: &mut Query<&mut Text>,
) -> Result<(), EntityRelatedCostumeError> 
{
    for (loader_action, children) in loader_screen_actions_query {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action {
            if layout_slot == slot_to_set {
                for child_entity in children.iter() {
                    let layout_slot_text_result =
                        layout_slot_text_query.get_mut(*child_entity);
                    if let Ok(mut slot_text) = layout_slot_text_result {
                        let new_button_text = domain_board_name_to_set_text_to.0.clone() + "\n"
                            + &domain_board_to_set_text_to.to_string_for_button();
                        slot_text.sections[0].value = new_button_text;
                        return Ok(());
                    }
                }
                return Err(EntityRelatedCostumeError::EntityNotInQuery)
            }
        }
    }
    Ok(())
}

pub fn set_slot_layout_preview(
    mut event_writer: EventWriter<SetNodeToPreviewLayout>,
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &Children)>,
    layout_preview_image_node_query: Query<Entity, With<LayoutPreviewNode>>,
){
    for loader_slot_set_request in event_reader.read(){
        if let Err(entity_error) = set_slot_layout_preview_inner(
            &mut event_writer,
            loader_slot_set_request.layout_entity,
            loader_slot_set_request.slot_to_set,
            &loader_screen_actions_query,
            &layout_preview_image_node_query,
        ){
            print_entity_related_error(entity_error);
        }
    }
}

fn set_slot_layout_preview_inner(
    event_writer: &mut EventWriter<SetNodeToPreviewLayout>,
    entity_of_layout_to_preview: Entity,
    slot_to_set: LoaderScreenSlot,
    loader_screen_actions_query: &Query<(&LoaderScreenAction, &Children)>,
    layout_preview_image_node_query: &Query<Entity, With<LayoutPreviewNode>>,
) -> Result<(), EntityRelatedCostumeError>
{
    for (loader_action, children) in loader_screen_actions_query {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action {
            if layout_slot == slot_to_set {
                for child_entity in children.iter() {
                    let layout_slot_preview_node_result =
                        layout_preview_image_node_query.get(*child_entity);
                    if let Ok(preview_image_node) = layout_slot_preview_node_result {
                        event_writer.send(SetNodeToPreviewLayout{
                            entity_with_ui_image: preview_image_node,
                            entity_of_layout_to_preview
                        });
                        return Ok(());
                    }
                }
                return Err(EntityRelatedCostumeError::EntityNotInQuery)
            }
        }
    }
    Ok(())
}