use enum_iterator::all;
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct SavedLayoutsScreensManager {
    pub saved_layouts_screens: Vec<SavedLayoutsScreen>,
    pub currently_displayed_screen: usize
}

pub struct SavedLayoutsScreensManagerPlugin;

impl Plugin for SavedLayoutsScreensManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SavedLayoutsScreensManager>()
            .add_systems(Startup, 
                spawn_saved_layouts_screens_from_data_base
            );
    }
}

fn spawn_saved_layouts_screens_from_data_base(
    data_base_manager: Res<DataBaseManager>,
    saved_layouts_screens_manager: ResMut<SavedLayoutsScreensManager>
){
    let mut next_loader_screen_slot : Option<LoaderScreenSlot> = None;
    let mut loader_screen_slots_iterator = all::<LoaderScreenSlot>();
    let mut next_saved_layouts_screen_to_spawn : Option<SavedLayoutsScreen> = None ;
    for (layout_name, _) in data_base_manager.get_saved_layouts_ref(){
        if next_loader_screen_slot == None {
            loader_screen_slots_iterator = all::<LoaderScreenSlot>();
            if let Some(saved_layout_screen) = next_saved_layouts_screen_to_spawn{
                saved_layouts_screens_manager.saved_layouts_screens.push(saved_layout_screen);
            }
            next_saved_layouts_screen_to_spawn = Some(SavedLayoutsScreen::default());
        }else{
            if let Some(saved_screen_layout_ref) 
                = &mut next_saved_layouts_screen_to_spawn
            {
                saved_screen_layout_ref.0.insert(next_loader_screen_slot.unwrap(), Some((*layout_name).clone()));
            }
        }
        next_loader_screen_slot = loader_screen_slots_iterator.next();
    }
    if let Some(saved_layout_screen) = next_saved_layouts_screen_to_spawn{
        saved_layouts_screens_manager.saved_layouts_screens.push(saved_layout_screen);
    }
}