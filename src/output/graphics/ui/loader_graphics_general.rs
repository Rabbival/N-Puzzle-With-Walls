use enum_iterator::all;
use crate::prelude::*;


pub struct LoaderGraphicsGeneralPlugin;

impl Plugin for LoaderGraphicsGeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loader), show_currently_displayed_saved_layouts_screen)
            .add_systems(Update, listen_for_layouts_screens_change_requests);
    }
}

//TODO: iterate over the screen slots enum, then for each matching one in the queries:
// layout_entities_query - if its screen tag exists in the currently displayed, show it, hide if not
// layout_texts_query - if the screen tag exists in the currently displayed, take its properties
// from the DB manager by name
fn show_currently_displayed_saved_layouts_screen(
    data_base_manager: Res<DataBaseManager>,
    saved_layouts_screens_manager: ResMut<DisplayedLoaderScreenNumber>
){
    for screen_slot in all::<LoaderScreenSlot>(){
        // let currently_displayed_screen_ref =
        //     &data_base_manager.get_saved_layouts_ref().get(
        //         get_layout_index_by_screen_and_slot(
        //              saved_layouts_screens_manager.0, 
        //              screen_slot
        //          )
        //     );
    }
}


//TODO: get requests when arrows are pressed. make sure to ensure the values would stay within
// the saved_layouts_screens_manager's boundaries
fn listen_for_layouts_screens_change_requests(){}