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
                    mark_chosen_slot_if_visible
                    ).in_set(StateChangeSystemSets::PrepareToHandleStateChange),
            )
            .add_systems(
                Update,(
                    listen_to_jump_to_page_requests.in_set(InputSystemSets::InputHandling),
                    (
                        update_slots_info_after_change.run_if(resource_changed::<DataBaseManager>
                                .or_else(resource_changed::<DisplayedLoaderScreenNumber>)),
                        update_arrows_after_change.run_if(resource_changed::<DataBaseManager>),
                    )
                        .in_set(InputSystemSets::MainChanges),
                    update_bottom_line_to_fit_new_chosen
                        .run_if(resource_changed::<ChosenLayoutScreenAndSlot>),
                    update_chosen_mark_after_change.run_if(resource_changed::<ChosenLayoutScreenAndSlot>
                        .or_else(resource_changed::<DisplayedLoaderScreenNumber>))
                )
            );
    }
}

fn listen_to_jump_to_page_requests(
    mut event_reader: EventReader<LoaderScreenActionInitiated>,
    mut displayed_loader_screen_number: ResMut<DisplayedLoaderScreenNumber>,
){
    for event in event_reader.read(){
        if let LoaderScreenAction::JumpToChosenLayoutScreen(Some(screen_number)) = event.action{
            displayed_loader_screen_number.0 = screen_number;
        }
    }
}

fn mark_chosen_slot_if_visible(
    mut loader_screen_actions_query: Query<(Entity, &LoaderScreenAction, &mut BackgroundColor)>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    chosen_layout_screen_and_slot: Res<ChosenLayoutScreenAndSlot>,
    mut commands: Commands
){
    for (entity, action, mut slot_background_color)
        in loader_screen_actions_query.iter_mut()
    {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *action {
            if let Some(chosen_screen_and_slot) = chosen_layout_screen_and_slot.0{
                if chosen_screen_and_slot.screen == displayed_loader_screen_number.0{
                    if layout_slot == chosen_screen_and_slot.slot{
                        set_color_to_pressed(&mut slot_background_color);
                        commands
                            .entity(entity)
                            .insert(SelectedOptionTag);
                        continue;
                    }
                }
            }
            set_color_to_normal(&mut slot_background_color);
            commands
                .entity(entity)
                .remove::<SelectedOptionTag>();
        }
    }
}

fn update_chosen_mark_after_change(
    loader_screen_actions_query: Query<(Entity, &LoaderScreenAction, &mut BackgroundColor)>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    chosen_layout_screen_and_slot: Res<ChosenLayoutScreenAndSlot>,
    commands: Commands
){
    mark_chosen_slot_if_visible(
        loader_screen_actions_query,
        displayed_loader_screen_number,
        chosen_layout_screen_and_slot,
        commands
    )
}

fn update_bottom_line_to_fit_new_chosen(
    optional_chosen_layout_screen_and_slot: Res<ChosenLayoutScreenAndSlot>,
    mut loader_screen_action_query: Query<&mut LoaderScreenAction>,
    mut chosen_layout_text_query: Query<&mut Text, With<ChosenLayoutTextTag>>,
    domain_boards_query: Query<&DomainBoard>,
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

        if let Some(entity) = new_chosen_ref_value{
            let board_query_result = domain_boards_query.get(*entity);
            if let Ok(board_ref) = board_query_result{
                updated_chosen_layout_text = String::from("chosen: ") + &board_ref.board_name.to_string();
                updated_optional_index = Some(calculate_db_index);
                updated_layout_name = DomainBoardName(board_ref.board_name.0.clone());
                updated_page_number = Some(chosen_layout_screen_and_slot.screen)
            }
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
            LoaderScreenAction::JumpToChosenLayoutScreen(optional_index) => {
                *optional_index = updated_page_number;
            },
            _ => {}
        }
    }
}

fn show_currently_displayed_saved_layouts_screen(
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    domain_boards_query: Query<&DomainBoard>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    layout_slot_text_query: Query<&mut Text>,
){
    if let Err(entity_error) = show_currently_displayed_saved_layouts_screen_inner(
        data_base_manager,
        displayed_loader_screen_number,
        domain_boards_query,
        loader_screen_actions_query,
        layout_slot_text_query,
    ) {
        print_entity_related_error(entity_error);
    }
}

fn show_currently_displayed_saved_layouts_screen_inner(
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    domain_boards_query: Query<&DomainBoard>,
    mut loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    mut layout_slot_text_query: Query<&mut Text>,
) -> Result<(), EntityRelatedCostumeError>
{
    for currently_checked_screen_slot in all::<LoaderScreenSlot>(){
        let index_from_slot = 
            SavedLayoutIndex::from_screen_and_slot(LayoutLoaderScreenAndSlot{
                screen: displayed_loader_screen_number.0,
                slot: currently_checked_screen_slot
            });
        let optional_layout_to_display =
            data_base_manager.try_get_layout_ref(&index_from_slot);
        match optional_layout_to_display{
            Some(layout_entity) => {
                match domain_boards_query.get(*layout_entity){
                    Ok(domain_board) => {
                        show_loader_slot_and_update_its_content(
                            currently_checked_screen_slot,
                            domain_board,
                            &mut loader_screen_actions_query,
                            &mut layout_slot_text_query
                        )?
                    },
                    Err(_entity_error) => return Err(EntityRelatedCostumeError::EntityNotInQuery)
                }
            },
            None => hide_loader_slot(currently_checked_screen_slot, &mut loader_screen_actions_query)
        }
    }
    Ok(())
}

fn hide_loader_slot(
    currently_checked_screen_slot: LoaderScreenSlot,
    loader_screen_actions_query: &mut Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
){
    for (loader_action, mut layout_slot_on_screen_tag, _) 
        in loader_screen_actions_query
    {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action{
            if layout_slot == currently_checked_screen_slot{
                layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Hidden);
            }
        }
    }
}

fn show_loader_slot_and_update_its_content(
    currently_checked_screen_slot: LoaderScreenSlot,
    domain_board_to_display: &DomainBoard,
    loader_screen_actions_query: &mut Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    layout_slot_text_query: &mut Query<&mut Text>
) -> Result<(), EntityRelatedCostumeError>
{
    for (loader_action, mut layout_slot_on_screen_tag, children)
        in loader_screen_actions_query
    {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action {
            if layout_slot == currently_checked_screen_slot {
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

fn update_slots_info_after_change(
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    domain_boards_query: Query<&DomainBoard>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag, &Children)>,
    layout_slot_text_query: Query<&mut Text>,
){
    show_currently_displayed_saved_layouts_screen(
        data_base_manager,
        displayed_loader_screen_number,
        domain_boards_query,
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