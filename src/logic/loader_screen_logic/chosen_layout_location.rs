use crate::prelude::*;

#[derive(Resource, Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct ChosenLayoutLocation(pub Option<ScreenSlotAndDifficulty>);

pub struct ChosenLayoutLocationPlugin;

impl Plugin for ChosenLayoutLocationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChosenLayoutLocation>()
            .add_systems(Update, ((
                    listen_for_successful_save_to_db,
                    listen_for_successful_removal_from_db,
                    listen_for_successful_db_clear
                ).in_set(InputSystemSets::InitialChanges),
                listen_for_new_layout_picks.in_set(InputSystemSets::InputHandling), 
              update_bottom_line_to_fit_new_chosen
                      .run_if(resource_changed::<ChosenLayoutLocation>)
                      .in_set(InputSystemSets::MainChanges),
            ));
    }
}

fn update_bottom_line_to_fit_new_chosen(
    mut visibility_set_event_writer: EventWriter<SetEntityVisibility>,
    mut loader_screen_action_query: Query<(&mut LoaderScreenAction, Entity, Option<&mut CustomOnScreenTag>)>,
    mut chosen_layout_text_query: Query<&mut Text, With<ChosenLayoutTextTag>>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    optional_chosen_layout_location: Res<ChosenLayoutLocation>,
    domain_board_name_query: Query<(Entity, &DomainBoardName)>,
    data_base_manager: Res<DataBaseManager>,
){
    let currently_shown_difficulty = applied_board_properties_query.single().board_difficulty;
    let chosen_layout_config = 
        determine_chosen_layout_config(
            currently_shown_difficulty,
            optional_chosen_layout_location,
            domain_board_name_query,
            data_base_manager
        );

    set_text_section_value_and_color(
        &mut chosen_layout_text_query.single_mut().sections[0],
        None,
        Some(chosen_layout_config.chosen_layout_button_text)
    );

    for (
        mut action_carrier, 
        button_entity, 
        optional_screen_tag
    ) 
        in &mut loader_screen_action_query
    {
        match action_carrier.as_mut(){
            LoaderScreenAction::GenerateBoard(optional_layout_entity) => {
                *optional_layout_entity = chosen_layout_config.optional_layout_entity;
                set_load_button_visibility(
                    &mut visibility_set_event_writer,
                    button_entity,
                    optional_layout_entity,
                    optional_screen_tag
                );
            },
            LoaderScreenAction::WarnBeforeDeletion(PopUpMessageType::DeleteBoard(optional_tuple)) => {
                *optional_tuple =
                    chosen_layout_config.optional_index.map(|config_index|
                        (chosen_layout_config.layout_name.clone(), config_index)
                    );
            },
            LoaderScreenAction::JumpToChosenLayoutScreen(
                optional_page_to_jump_to, 
                board_difficulty
            ) => 
                {
                    *optional_page_to_jump_to = chosen_layout_config.optional_page_number;
                    *board_difficulty = currently_shown_difficulty;
                },
            _ => {}
        }
    }
}

fn determine_chosen_layout_config(
    currently_shown_difficulty: BoardDifficulty,
    optional_chosen_layout_location: Res<ChosenLayoutLocation>,
    domain_board_name_query: Query<(Entity, &DomainBoardName)>,
    data_base_manager: Res<DataBaseManager>,
) -> LoaderChosenLayoutConfig
{
    if let Some(chosen_layout_location) = optional_chosen_layout_location.0 {
        let calculated_db_index =
            SavedLayoutIndexInDifficultyVec::from_screen_and_slot(
                &currently_shown_difficulty,
                &chosen_layout_location.screen_and_slot
            );
        let new_chosen_ref_value = data_base_manager.try_get_layout_ref(&calculated_db_index);

        if let Some(entity) = new_chosen_ref_value{
            let board_name_query_result = domain_board_name_query.get(*entity);
            if let Ok((layout_entity, board_name)) = board_name_query_result{
                return LoaderChosenLayoutConfig{
                    chosen_layout_button_text: String::from("chosen: ") + &board_name.to_string(),
                    layout_name: DomainBoardName(board_name.0.clone()),
                    optional_layout_entity: Some(layout_entity),
                    optional_index: Some(calculated_db_index),
                    optional_page_number: Some(chosen_layout_location.screen_and_slot.screen)
                };
            }
        }
    }
    LoaderChosenLayoutConfig::default()
}

fn set_load_button_visibility(
    visibility_set_event_writer: &mut EventWriter<SetEntityVisibility>,
    button_entity: Entity,
    optional_layout_entity: &mut Option<Entity>,
    optional_screen_tag: Option<Mut<CustomOnScreenTag>>
){
    let new_visibility =
        if optional_layout_entity.is_some(){
            Visibility::Visible
        }else{
            Visibility::Hidden
        };
    if let Some(mut screen_tag) = optional_screen_tag{
        screen_tag.on_own_screen_visibility = Some(new_visibility);
    }
    visibility_set_event_writer.send(
        SetEntityVisibility{entity: button_entity, visibility: new_visibility}
    );
}

fn listen_for_new_layout_picks(
    mut event_reader: EventReader<LoaderScreenActionEvent>,
    currently_displayed_loader_screen: Res<DisplayedLoaderScreenNumber>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut chosen_layout_screen_and_slot: ResMut<ChosenLayoutLocation>
){
    for loader_action in event_reader.read(){
        if let LoaderScreenAction::ChooseLayoutInSlot(loader_slot) = loader_action.action{
            let applied_board_properties = applied_board_properties_query.single();
            chosen_layout_screen_and_slot.0 = Some(ScreenSlotAndDifficulty{
                screen_and_slot: LayoutLoaderScreenAndSlot{
                    screen: currently_displayed_loader_screen.0,
                    slot: loader_slot
                },
                difficulty: applied_board_properties.board_difficulty
            });
        }
    }
}

fn listen_for_successful_save_to_db(
    mut event_reader: EventReader<SuccessSavingToDB>,
    mut optional_chosen_layout_location: ResMut<ChosenLayoutLocation>
){
    for saving_to_db in event_reader.read(){
        if let Some(chosen_layout_location) = &mut optional_chosen_layout_location.0{
            let current_chosen_index =
                SavedLayoutIndexInDifficultyVec::from_screen_and_slot(
                    &chosen_layout_location.difficulty,
                    &chosen_layout_location.screen_and_slot
                );
            if saving_to_db.0 <= current_chosen_index{
                chosen_layout_location.screen_and_slot.increment();
            }
        }
    }
}

fn listen_for_successful_removal_from_db(
    mut event_reader: EventReader<SuccessRemovingFromDB>,
    mut chosen_layout_location: ResMut<ChosenLayoutLocation>
){
    for removal_from_db in event_reader.read(){
        if let Some(chosen_screen_slot_and_difficulty) = 
            &mut chosen_layout_location.0
        {
            let current_chosen_index =
                SavedLayoutIndexInDifficultyVec::from_screen_and_slot(
                    &chosen_screen_slot_and_difficulty.difficulty,
                    &chosen_screen_slot_and_difficulty.screen_and_slot
                );
            if removal_from_db.0 == current_chosen_index{
                chosen_layout_location.0 = None;
            }else if removal_from_db.0 < current_chosen_index{
                chosen_screen_slot_and_difficulty.screen_and_slot.decrement_if_possible();
            }
        }
    }
}

fn listen_for_successful_db_clear(
    mut event_reader: EventReader<SuccessClearingDB>,
    mut chosen_layout_screen_and_slot: ResMut<ChosenLayoutLocation>
){
    for _db_clearing in event_reader.read(){
        chosen_layout_screen_and_slot.0 = None;
    }
}