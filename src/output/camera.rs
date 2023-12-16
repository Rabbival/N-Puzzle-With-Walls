use crate::prelude::*;

pub const CAMERA_ZOOM: f32 = 0.25;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x = (GRID_SIDE_LENGTH-1) as f32 / 2.0 * ATLAS_CELL_SQUARE_SIZE;
    camera.transform.translation.y = -1.0 * (GRID_SIDE_LENGTH-1) as f32 / 2.0 * ATLAS_CELL_SQUARE_SIZE;
    camera.projection.scale=CAMERA_ZOOM;

    commands.spawn(
        camera
    );
}
