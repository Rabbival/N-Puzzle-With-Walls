use crate::prelude::*;

#[derive(Resource, Default)]
pub struct DisplayedLoaderScreenNumber(pub usize);

pub struct DisplayedLoaderScreenPlugin;

impl Plugin for DisplayedLoaderScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DisplayedLoaderScreenNumber>()
            .add_systems(Update,
                listen_to_screen_change_arrows_presses
                    .in_set(InputSystemSets::InputHandling)
            );
    }
}

fn listen_to_screen_change_arrows_presses(
    mut event_reader: EventReader<LoaderScreenActionEvent>,
    mut displayed_loader_screen_number: ResMut<DisplayedLoaderScreenNumber>,
    data_base_manager: Res<DataBaseManager>,
){
    for loader_screen_action in event_reader.read(){
        if let LoaderScreenAction::ChangeScreen(change_request) = 
            loader_screen_action.action
        {
            let saved_layouts_count = data_base_manager.get_saved_layouts_ref().len();
            if saved_layouts_count <= SAVED_LAYOUTS_PER_SCREEN {
                continue;
            } else{
                let mut max_full_screen = (saved_layouts_count / SAVED_LAYOUTS_PER_SCREEN) -1;
                if saved_layouts_count % SAVED_LAYOUTS_PER_SCREEN == 0 {
                    max_full_screen -= 1;
                } 
                match change_request{
                    ScreenChangeRequestType::Next => {
                        if displayed_loader_screen_number.0 <= max_full_screen {
                            displayed_loader_screen_number.0 += 1;
                        } else{
                            displayed_loader_screen_number.0 = 0;
                        }
                    }
                    ScreenChangeRequestType::Previous => {
                        if displayed_loader_screen_number.0 > 0 {
                            displayed_loader_screen_number.0 -= 1;
                        } else{
                            displayed_loader_screen_number.0 = max_full_screen + 1;
                        }
                    }
                }
            }
        }
    }
}