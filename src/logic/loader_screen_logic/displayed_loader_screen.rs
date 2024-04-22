use crate::prelude::*;

#[derive(Resource, Default)]
pub struct DisplayedLoaderScreenNumber(pub usize);

pub struct DisplayedLoaderScreenPlugin;

impl Plugin for DisplayedLoaderScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DisplayedLoaderScreenNumber>()
            .add_systems(Update,
                listen_to_screen_change_arrows_presses
            );
    }
}

fn listen_to_screen_change_arrows_presses(
    mut event_listener: EventReader<LoaderScreenArrowPressed>, 
    mut displayed_loader_screen_number: ResMut<DisplayedLoaderScreenNumber>,
    data_base_manager: Res<DataBaseManager>,
){
    for change_request in event_listener.read(){
        let max_not_empty_screen =
            data_base_manager.get_saved_layouts_ref().len() / SAVED_LAYOUTS_PER_SCREEN;
        match change_request.action{
            ScreenChangeArrowsAction::Next => {
                if displayed_loader_screen_number.0 < max_not_empty_screen {
                    displayed_loader_screen_number.0 += 1;
                } else{
                    displayed_loader_screen_number.0 = 0;
                }
            } 
            ScreenChangeArrowsAction::Previous => {
                if displayed_loader_screen_number.0 > 0 {
                    displayed_loader_screen_number.0 -= 1;
                } else{
                    displayed_loader_screen_number.0 = max_not_empty_screen;
                }
            }
        }
    }
}