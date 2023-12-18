use crate::{prelude::*, screen_setup};

#[derive(Resource, Default)]
pub struct CameraZoom(pub f32);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .init_resource::<CameraZoom>()
        ;
    }
}

fn spawn_camera(
    mut commands: Commands,
    solved_board_query: Query<&TileTypeBoard,(With<SolvedBoard>, Without<GameBoard>)>,
    mut zoom: ResMut<CameraZoom>
) {
    let solved_board=solved_board_query.single();
    let grid_side_length= *solved_board.get_side_length();
    
    let camera_zoom = grid_side_length as f32 * ATLAS_CELL_SQUARE_SIZE / screen_setup::WINDOW_RESOLUTION;
    zoom.0=camera_zoom;

    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x = (grid_side_length-1) as f32 / 2.0 * ATLAS_CELL_SQUARE_SIZE;
    camera.transform.translation.y = -1.0 * (grid_side_length-1) as f32 / 2.0 * ATLAS_CELL_SQUARE_SIZE;
    camera.projection.scale=camera_zoom;

    commands.spawn(
        camera
    );
}