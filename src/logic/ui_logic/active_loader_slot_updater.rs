use crate::prelude::*;

pub struct ActiveLoaderSlotUpdaterPlugin;

impl Plugin for ActiveLoaderSlotUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, listen_for_loader_slot_set_request.after(show_currently_displayed_saved_layouts_screen)
        );
    }
}

fn listen_for_loader_slot_set_request(
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    mut loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    mut layout_slot_text_query: Query<&mut Text>,
    mut layout_preview_parent_node_query: Query<&mut LayoutPreviewParentNode>
){
    for loader_slot_set_request in event_reader.read(){
        if let Err(entity_error) = listen_for_loader_slot_set_request_inner(
            loader_slot_set_request.layout_entity,
            loader_slot_set_request.slot_to_show_in,
            &mut loader_screen_actions_query,
            &mut layout_slot_text_query,
            &mut layout_preview_parent_node_query
        ){
            print_entity_related_error(entity_error);
        }
    }
}

fn listen_for_loader_slot_set_request_inner(
    layout_entity: Entity,
    slot_to_show_in: LoaderScreenSlot
    loader_screen_actions_query: &mut Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    layout_slot_text_query: &mut Query<&mut Text>,
    layout_preview_parent_node_query: &mut Query<&mut LayoutPreviewParentNode>
) -> Result<(), EntityRelatedCostumeError>
{
    
    //TODO: think how to reorder everything here. Maybe fetch the screen slot itself then do
    // a different function for each from: visibility, text set, layout set
    
    match domain_boards_query.get(layout_entity){
        Ok(domain_board_to_display) => {

        },
        Err(_entity_error) => return Err(EntityRelatedCostumeError::EntityNotInQuery)
    };
    
    
    for (loader_action, mut layout_slot_on_screen_tag, children)
    in loader_screen_actions_query
    {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action {
            if layout_slot == slot_to_show_in {
                layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Visible);
                for child_entity in children.iter() {
                    let layout_slot_text_result =
                        layout_slot_text_query.get_mut(*child_entity);
                    if let Ok(mut slot_text) = layout_slot_text_result {
                        slot_text.sections[0].value = domain_board_to_display.to_string_for_button();
                        return Ok(());
                    }
                }
                return Err(EntityRelatedCostumeError::EntityNotInQuery)
            }
        }
    }
    Ok(())
}