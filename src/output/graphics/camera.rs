use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(GameState::GameBoardGenerated),
                 adjust_camera_zoom_to_new_settings
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn adjust_camera_zoom_to_new_settings(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
) {
    let grid_side_length = applied_board_props_query.single().size.to_grid_side_length();
    let new_camera_zoom =
        grid_side_length as f32 * ATLAS_CELL_SQUARE_SIZE / BOARD_SIZE_IN_PIXELS;

    let (mut camera_transform, mut camera_projection) = camera_query.single_mut();
    camera_transform.translation.x =
        (grid_side_length - 1) as f32 / 2.0 * ATLAS_CELL_SQUARE_SIZE;
    camera_transform.translation.y =
        -1.0 * (grid_side_length - 1) as f32 / 2.0 * ATLAS_CELL_SQUARE_SIZE;
    camera_projection.scale = new_camera_zoom;
}
