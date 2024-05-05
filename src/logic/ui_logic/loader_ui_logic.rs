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
                    update_chosen_mark_after_change.run_if(resource_changed::<ChosenLayoutScreenAndSlot>
                        .or_else(resource_changed::<DisplayedLoaderScreenNumber>))
                )
            );
    }
}

fn listen_to_jump_to_page_requests(
    mut event_reader: EventReader<LoaderScreenActionEvent>,
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
                if chosen_screen_and_slot.screen == displayed_loader_screen_number.0 && layout_slot == chosen_screen_and_slot.slot {
                    set_color_to_pressed(&mut slot_background_color);
                    commands
                        .entity(entity)
                        .insert(SelectedOptionTag);
                    continue;
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

pub fn show_currently_displayed_saved_layouts_screen(
    event_writer: EventWriter<LoaderSlotSetEvent>,
    data_base_manager: Res<DataBaseManager>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag)>,
){
    if let Err(entity_error) = show_currently_displayed_saved_layouts_screen_inner(
        event_writer,
        data_base_manager,
        applied_board_properties_query,
        displayed_loader_screen_number,
        loader_screen_actions_query
    ) {
        print_entity_related_error(entity_error);
    }
}

fn show_currently_displayed_saved_layouts_screen_inner(
    mut event_writer: EventWriter<LoaderSlotSetEvent>,
    data_base_manager: Res<DataBaseManager>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    mut loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag)>,
) -> Result<(), EntityRelatedCostumeError>
{
    for currently_checked_screen_slot in all::<LoaderScreenSlot>(){
        let applied_board_properties = applied_board_properties_query.single();
        let index_from_slot = 
            SavedLayoutIndexInDifficultyVec::from_screen_and_slot(
                &applied_board_properties.board_difficulty,
    &LayoutLoaderScreenAndSlot{
                    screen: displayed_loader_screen_number.0,
                    slot: currently_checked_screen_slot
                }
            );
        let optional_layout_to_display =
            data_base_manager.try_get_layout_ref(&index_from_slot);
        match optional_layout_to_display{
            Some(layout_entity) => {
                event_writer.send(LoaderSlotSetEvent{
                    layout_entity: layout_entity.to_owned(), 
                    slot_to_set: currently_checked_screen_slot
                });
            },
            None => hide_loader_slot(currently_checked_screen_slot, &mut loader_screen_actions_query)
        }
    }
    Ok(())
}

fn hide_loader_slot(
    currently_checked_screen_slot: LoaderScreenSlot,
    loader_screen_actions_query: &mut Query<(&LoaderScreenAction, &mut CustomOnScreenTag)>,
){
    for (loader_action, mut layout_slot_on_screen_tag) 
        in loader_screen_actions_query
    {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action{
            if layout_slot == currently_checked_screen_slot{
                layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Hidden);
            }
        }
    }
}

fn update_slots_info_after_change(
    event_writer: EventWriter<LoaderSlotSetEvent>,
    data_base_manager: Res<DataBaseManager>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag)>,
){
    show_currently_displayed_saved_layouts_screen(
        event_writer,
        data_base_manager,
        applied_board_properties_query,
        displayed_loader_screen_number,
        loader_screen_actions_query
    );
}

fn only_show_arrows_if_theres_more_than_one_available_screen(
    data_base_manager: Res<DataBaseManager>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut arrows_visibility_tags_query: Query<&mut CustomOnScreenTag, With<ScreenChangeArrowTag>>
){
    let mut show_arrows = false;
    let applied_board_properties = applied_board_properties_query.single();
    let optional_current_dif_layouts_count =
        data_base_manager.get_layouts_count_by_difficulty(&applied_board_properties.board_difficulty);
    if let Some(current_dif_layouts_count) = optional_current_dif_layouts_count{
        if current_dif_layouts_count > SAVED_LAYOUTS_PER_SCREEN{
            show_arrows = true;
        }
    }

    for mut visibility_tag in arrows_visibility_tags_query.iter_mut(){
        if show_arrows {
            visibility_tag.on_own_screen_visibility = Some(Visibility::Visible);
        }else{
            visibility_tag.on_own_screen_visibility = Some(Visibility::Hidden);
        }
    }
}

fn update_arrows_after_change(
    data_base_manager: Res<DataBaseManager>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    arrows_visibility_tags_query: Query<&mut CustomOnScreenTag, With<ScreenChangeArrowTag>>
){
    only_show_arrows_if_theres_more_than_one_available_screen(
        data_base_manager,
        applied_board_properties_query,
        arrows_visibility_tags_query
    );
}