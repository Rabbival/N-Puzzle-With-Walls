use enum_iterator::all;
use crate::prelude::*;


pub struct LoaderUiLogicPlugin;

impl Plugin for LoaderUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(AppState::Loader), (
                    show_currently_displayed_saved_layouts_screen,
                    only_show_arrows_if_theres_more_than_one_available_screen,
                    ).in_set(StateChangeSystemSets::PrepareToHandleStateChange),
            )
            .add_systems(
                Update,(
                    (
                        update_slots_info_after_change,
                        update_arrows_after_change
                    ).run_if(
                        resource_changed::<DataBaseManager>
                        .or_else(resource_changed::<DisplayedLoaderScreenNumber>)
                    )
                        .in_set(InputSystemSets::MainChanges),
                )
            )
            .add_systems(
                Update,(
                    update_bottom_line_to_fit_new_chosen
                    .run_if(resource_changed::<ChosenLayoutScreenAndSlot>),
                )
            );
    }
}

fn update_bottom_line_to_fit_new_chosen(
    optional_chosen_layout_screen_and_slot: Res<ChosenLayoutScreenAndSlot>,
    mut loader_screen_action_query: Query<&mut LoaderScreenAction>,
    mut chosen_layout_text_query: Query<&mut Text, With<ChosenLayoutTextTag>>,
    data_base_manager: Res<DataBaseManager>,
){
    let mut updated_chosen_layout_text = String::from("no chosen board");
    let mut updated_optional_index = None;
    let mut updated_layout_name = DomainBoardName(String::new());
    let mut updated_page_number = None;
    
    if let Some(chosen_layout_screen_and_slot) = 
        optional_chosen_layout_screen_and_slot.0
    {
        let calculate_db_index =
            SavedLayoutIndex::from_screen_and_slot(chosen_layout_screen_and_slot);
        let new_chosen_ref_value = data_base_manager.try_get_layout_ref(&calculate_db_index);

        if let Some(board_ref) = new_chosen_ref_value{
            updated_chosen_layout_text = String::from("chosen: ") + &board_ref.board_name;
            updated_optional_index = Some(calculate_db_index);
            updated_layout_name = DomainBoardName(board_ref.board_name.clone());
            updated_page_number = Some(chosen_layout_screen_and_slot.screen)
        }
    }

    chosen_layout_text_query.single_mut().sections[0].value = updated_chosen_layout_text;
    for mut action_carrier in loader_screen_action_query.iter_mut(){
        match action_carrier.as_mut(){
            LoaderScreenAction::GenerateBoard(optional_index) => {
                *optional_index = updated_optional_index;
            },
            LoaderScreenAction::WarnBeforeDeletion(AreYouSureMessageType::DeleteBoard(optional_tuple)) => {
                if updated_optional_index.is_none() {
                    *optional_tuple = None;
                }else{
                    *optional_tuple = Some((updated_layout_name.clone(), updated_optional_index.unwrap()));
                }
            },
            LoaderScreenAction::JumpToChosenLayoutPage(optional_index) => {
                *optional_index = updated_page_number;
            },
            _ => {}
        }
    }
}

fn show_currently_displayed_saved_layouts_screen(
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    mut loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    mut layout_slot_text_query: Query<&mut Text>,
){
    for currently_checked_screen_slot in all::<LoaderScreenSlot>(){
        let index_from_slot = 
            SavedLayoutIndex::from_screen_and_slot(LayoutLoaderScreenAndSlot{
                screen: displayed_loader_screen_number.0,
                slot: currently_checked_screen_slot
            });
        let optional_layout_to_display =
            data_base_manager.try_get_layout_ref(&index_from_slot);
        if let Err(entity_error) = handle_screen_slot_content_and_visibility(
            currently_checked_screen_slot,
            optional_layout_to_display,
            &mut loader_screen_actions_query,
            &mut layout_slot_text_query
        ){
            print_entity_related_error(entity_error);
        }
    }
}

fn handle_screen_slot_content_and_visibility(
    currently_checked_screen_slot: LoaderScreenSlot,
    optional_layout_to_display: Option<&DomainBoard>,
    loader_screen_actions_query: &mut Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    layout_slot_text_query: &mut Query<&mut Text>
) -> Result<(), EntityRelatedCostumeError>
{
    let layout_slots_iter =
        loader_screen_actions_query.iter_mut().filter_map(
            |(action, visibility_tag, children)|
            if let LoaderScreenAction::ChooseLayoutInSlot(slot) = *action {
                Some((slot, visibility_tag, children))
            } else {
                None
            }
        );
    if let Some(layout) = optional_layout_to_display{
        for (
            loader_screen_slot,
            mut layout_slot_on_screen_tag,
            children
        )
        in layout_slots_iter
        {
            if loader_screen_slot == currently_checked_screen_slot{
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
        in layout_slots_iter
        {
            if layout_slot_tag == currently_checked_screen_slot{
                layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Hidden);
            }
        }
        Ok(())
    }
}

fn update_slots_info_after_change(
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    layout_slot_text_query: Query<&mut Text>,
){
    show_currently_displayed_saved_layouts_screen(
        data_base_manager,
        displayed_loader_screen_number,
        loader_screen_actions_query,
        layout_slot_text_query,
    );
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

fn update_arrows_after_change(
    data_base_manager: Res<DataBaseManager>,
    arrows_visibility_tags_query: Query<&mut CustomOnScreenTag, With<ScreenChangeArrowTag>>
){
    only_show_arrows_if_theres_more_than_one_available_screen(
        data_base_manager,
        arrows_visibility_tags_query
    );
}