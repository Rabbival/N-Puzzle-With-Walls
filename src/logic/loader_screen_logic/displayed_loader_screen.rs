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
            )
            .add_systems(OnEnter(AppState::Loader),
                clamp_displayed_loader_screen
            );
    }
}

fn clamp_displayed_loader_screen(
    mut displayed_loader_screen_number: ResMut<DisplayedLoaderScreenNumber>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    data_base_manager: Res<DataBaseManager>,
){
    let applied_board_properties = applied_board_properties_query.single();
    let last_screen = calculate_last_screen_number(
        applied_board_properties,
        &data_base_manager
    );
    let current_displayed_screen = displayed_loader_screen_number.as_mut();
    if current_displayed_screen.0 > last_screen{
        current_displayed_screen.0 = last_screen;
    } 
}

fn listen_to_screen_change_arrows_presses(
    mut event_reader: EventReader<LoaderScreenActionEvent>,
    mut displayed_loader_screen_number: ResMut<DisplayedLoaderScreenNumber>,
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    data_base_manager: Res<DataBaseManager>,
){
    for loader_screen_action in event_reader.read(){
        if let LoaderScreenAction::ChangeScreen(change_request) = 
            loader_screen_action.action
        {
            let applied_board_properties = applied_board_properties_query.single();
            let last_screen = calculate_last_screen_number(
                applied_board_properties,
                &data_base_manager
            );
            match change_request{
                ScreenChangeRequestType::Next => {
                    if displayed_loader_screen_number.0 < last_screen {
                        displayed_loader_screen_number.0 += 1;
                    } else{
                        displayed_loader_screen_number.0 = 0;
                    }
                }
                ScreenChangeRequestType::Previous => {
                    if displayed_loader_screen_number.0 > 0 {
                        displayed_loader_screen_number.0 -= 1;
                    } else{
                        displayed_loader_screen_number.0 = last_screen;
                    }
                }
            }
        }
    }
}

fn calculate_last_screen_number(
    applied_board_properties: &BoardProperties, 
    data_base_manager: &DataBaseManager
) 
    -> usize 
{
    let optional_layouts_with_dif_count =
        data_base_manager.get_layouts_count_by_difficulty(&applied_board_properties.board_difficulty);
    let layouts_with_dif_count = optional_layouts_with_dif_count.unwrap_or_default();
    let mut last_screen = layouts_with_dif_count / SAVED_LAYOUTS_PER_SCREEN;
    if layouts_with_dif_count % SAVED_LAYOUTS_PER_SCREEN == 0 {
        last_screen -= 1;
    }
    last_screen
}