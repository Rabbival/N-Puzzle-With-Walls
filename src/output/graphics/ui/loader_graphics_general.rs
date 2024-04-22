use enum_iterator::all;
use crate::prelude::*;


pub struct LoaderGraphicsGeneralPlugin;

impl Plugin for LoaderGraphicsGeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                OnEnter(AppState::Loader), 
                show_currently_displayed_saved_layouts_screen
                    .in_set(StateChangeSystemSets::PrepareToHandleStateChange),
            )
            .add_systems(Update, listen_for_layouts_screens_change_requests);
    }
}

// layout_entities_query - if its screen tag exists in the currently displayed, show it, hide if not
// layout_texts_query - if the screen tag exists in the currently displayed, show its properties
fn show_currently_displayed_saved_layouts_screen(
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: ResMut<DisplayedLoaderScreenNumber>,
    mut layout_slots_query: Query<(&LoaderScreenSlotTag, &mut CustomOnScreenTag, &Children)>,
    mut layout_slot_text_query: Query<&mut Text>,
){
    for screen_slot in all::<LoaderScreenSlot>(){
        let optional_layout_to_display =
            data_base_manager.get_saved_layouts_ref().get(
                get_layout_index_by_screen_and_slot(
                    displayed_loader_screen_number.0,
                    screen_slot
                ).0
            );
        if let Err(entity_error) = handle_screen_slot_content_and_visibility(
            screen_slot,
            optional_layout_to_display,
            &mut layout_slots_query,
            &mut layout_slot_text_query
        ){
            print_entity_related_error(entity_error);
        }
    }
}

fn handle_screen_slot_content_and_visibility(
    slot: LoaderScreenSlot,
    optional_layout_to_display: Option<&DomainBoard>,
    layout_slots_query: &mut Query<(&LoaderScreenSlotTag, &mut CustomOnScreenTag, &Children)>,
    layout_slot_text_query: &mut Query<&mut Text>
) -> Result<(), EntityRelatedCostumeError>
{

    if let Some(layout) = optional_layout_to_display{
        for (
            layout_slot_tag,
            mut layout_slot_on_screen_tag,
            children
        )
        in layout_slots_query.iter_mut()
        {
            if layout_slot_tag.0 == slot{
                layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Visible);
                for child_entity in children.iter(){
                    let layout_slot_text_result =
                        layout_slot_text_query.get_mut(*child_entity);
                    if let Ok(mut slot_text) = layout_slot_text_result{
                        slot_text.sections[0].value = layout.to_string_for_button();
                        return Ok(());
                    }
                }
                return Err(EntityRelatedCostumeError::EntityNotInQuery)
            }
        }
        Ok(())
    }else{
        for (
            layout_slot_tag,
            mut layout_slot_on_screen_tag,
            _
        )
        in layout_slots_query.iter_mut()
        {
            if layout_slot_tag.0 == slot{
                layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Hidden);
            }
        }
        Ok(())
    }
}


//TODO: get requests when arrows are pressed. make sure to ensure the values would stay within
// the saved_layouts_screens_manager's boundaries
fn listen_for_layouts_screens_change_requests(){}