use crate::prelude::*;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x = GRID_SIZE as f32 / 2.0;
    camera.transform.translation.y = GRID_SIZE as f32 / 2.0;

    commands.spawn(
        camera
    );
}
