use bevy::prelude::*;
use crate::prelude::*;

pub struct ScreenSetupPlugin;

impl Plugin for ScreenSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resizable: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .build(),
            )
            .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 0.75,
            })
            .add_systems(PostStartup, set_resolution_based_on_board_size)
            ;
    }
}

fn set_resolution_based_on_board_size(
    mut windows: Query<&mut Window>,
    solved_board_query: Query<&TileTypeBoard,(With<SolvedBoard>, Without<GameBoard>)>
){
    let mut window = windows.single_mut();
    let solved_board = solved_board_query.single();

    let res = solved_board.get_side_length().clone() as f32 * ATLAS_CELL_SQUARE_SIZE / CAMERA_ZOOM;
    window.resolution.set(res, res);
}