use bevy::prelude::*;
use crate::prelude::*;

pub const WINDOW_SIZE: f32 = GRID_SIDE_LENGTH as f32 * ATLAS_CELL_SQUARE_SIZE / CAMERA_ZOOM;

pub struct ScreenSetupPlugin;

impl Plugin for ScreenSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
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
            ;
    }
}