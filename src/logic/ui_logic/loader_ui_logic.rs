use enum_iterator::all;
use crate::prelude::*;


pub struct LoaderUiLogicPlugin;

impl Plugin for LoaderUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Loader),(
                show_currently_displayed_saved_layouts_screen,
                only_show_arrows_if_theres_more_than_one_available_screen,
                ).in_set(StateChangeSystemSets::PrepareToHandleStateChange),
        )
            .add_systems(
                Update,(
                    (
                        update_slots_info_after_change,
                        update_arrows_after_change
                    )
                        .run_if(
                            resource_changed::<DisplayedLoaderScreenNumber>
                                .or_else(resource_changed::<DataBaseManager>)
                        ).in_set(StateChangeSystemSets::PrepareToHandleStateChange),
                )
            );
    }
}

fn show_currently_displayed_saved_layouts_screen(
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    mut layout_slots_query: Query<(&LoaderScreenSlotTag, &mut CustomOnScreenTag, &Children)>,
    mut layout_slot_text_query: Query<&mut Text>,
){
    for screen_slot in all::<LoaderScreenSlot>(){
        let optional_index_from_slot = 
            SavedLayoutIndex::try_from_screen_and_slot(
                displayed_loader_screen_number.0,
                screen_slot
            );
        if optional_index_from_slot.is_none()  { continue; }
        let optional_layout_to_display =
            data_base_manager.try_get_layout_ref(&optional_index_from_slot.unwrap());
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
        for (layout_slot_tag, mut layout_slot_on_screen_tag, _)
        in layout_slots_query.iter_mut()
        {
            if layout_slot_tag.0 == slot{
                layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Hidden);
            }
        }
        Ok(())
    }
}

fn only_show_arrows_if_theres_more_than_one_available_screen(
    data_base_manager: Res<DataBaseManager>,
    mut arrows_visibility_tags_query: Query<&mut CustomOnScreenTag, With<ScreenChangeArrowTag>>
){
    let saved_layouts_count = data_base_manager.get_saved_layouts_ref().len();
    for mut visibility_tag in arrows_visibility_tags_query.iter_mut(){
        if saved_layouts_count <= SAVED_LAYOUTS_PER_SCREEN {
            visibility_tag.on_own_screen_visibility = Some(Visibility::Hidden);
        }else{
            visibility_tag.on_own_screen_visibility = Some(Visibility::Visible);
        }
    }
}

fn update_slots_info_after_change(
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    layout_slots_query: Query<(&LoaderScreenSlotTag, &mut CustomOnScreenTag, &Children)>,
    layout_slot_text_query: Query<&mut Text>,
){
    show_currently_displayed_saved_layouts_screen(
        data_base_manager,
        displayed_loader_screen_number,
        layout_slots_query,
        layout_slot_text_query,
    );
}

fn update_arrows_after_change(
    data_base_manager: Res<DataBaseManager>,
    arrows_visibility_tags_query: Query<&mut CustomOnScreenTag, With<ScreenChangeArrowTag>>
){
    only_show_arrows_if_theres_more_than_one_available_screen(
        data_base_manager,
        arrows_visibility_tags_query
    );
}