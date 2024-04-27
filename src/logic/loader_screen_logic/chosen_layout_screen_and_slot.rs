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
                listen_for_new_layout_picks.in_set(InputSystemSets::InputHandling)
            ));
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