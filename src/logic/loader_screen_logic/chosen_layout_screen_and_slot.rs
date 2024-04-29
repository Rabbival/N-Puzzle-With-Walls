use crate::prelude::*;

#[derive(Resource, Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct ChosenLayoutScreenAndSlot(pub Option<LayoutLoaderScreenAndSlot>);

pub struct ChosenLayoutScreenAndSlotPlugin;

impl Plugin for ChosenLayoutScreenAndSlotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChosenLayoutScreenAndSlot>()
            .add_systems(Update, ((
                    listen_for_successful_save_to_db,
                    listen_for_successful_removal_from_db,
                    listen_for_successful_db_clear
                ).in_set(InputSystemSets::InitialChanges),
                listen_for_new_layout_picks.in_set(InputSystemSets::InputHandling), 
              update_bottom_line_to_fit_new_chosen
                      .run_if(resource_changed::<ChosenLayoutScreenAndSlot>)
                      .in_set(InputSystemSets::MainChanges),
            ));
    }
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

fn listen_for_new_layout_picks(
    mut event_reader: EventReader<LoaderScreenActionInitiated>,
    currently_displayed_loader_screen: Res<DisplayedLoaderScreenNumber>,
    mut chosen_layout_screen_and_slot: ResMut<ChosenLayoutScreenAndSlot>
){
    for loader_action in event_reader.read(){
        if let LoaderScreenAction::ChooseLayoutInSlot(loader_slot) = loader_action.action{
            chosen_layout_screen_and_slot.0 = Some(LayoutLoaderScreenAndSlot{
                screen: currently_displayed_loader_screen.0,
                slot: loader_slot
            });
        }
    }
}

fn listen_for_successful_save_to_db(
    mut event_reader: EventReader<SuccessSavingToDB>,
    mut chosen_layout_screen_and_slot: ResMut<ChosenLayoutScreenAndSlot>
){
    for saving_to_db in event_reader.read(){
        if let Some(chosen_screen_and_slot) = &mut chosen_layout_screen_and_slot.0{
            let current_chosen_index =
                SavedLayoutIndex::from_screen_and_slot(*chosen_screen_and_slot);
            if saving_to_db.0 < current_chosen_index{
                chosen_screen_and_slot.increment();
            }
        }
    }
}

fn listen_for_successful_removal_from_db(
    mut event_reader: EventReader<SuccessRemovingFromDB>,
    mut chosen_layout_screen_and_slot: ResMut<ChosenLayoutScreenAndSlot>
){
    for removal_from_db in event_reader.read(){
        if let Some(chosen_screen_and_slot) = &mut chosen_layout_screen_and_slot.0{
            let current_chosen_index =
                SavedLayoutIndex::from_screen_and_slot(*chosen_screen_and_slot);
            if removal_from_db.0 == current_chosen_index{
                chosen_layout_screen_and_slot.0 = None;
            }else if removal_from_db.0 < current_chosen_index{
                chosen_screen_and_slot.decrement_if_possible();
            }
        }
    }
}

fn listen_for_successful_db_clear(
    mut event_reader: EventReader<SuccessClearingDB>,
    mut chosen_layout_screen_and_slot: ResMut<ChosenLayoutScreenAndSlot>
){
    for _db_clearing in event_reader.read(){
        chosen_layout_screen_and_slot.0 = None;
    }
}