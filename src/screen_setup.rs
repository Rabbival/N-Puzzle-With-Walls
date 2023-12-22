use bevy::window::WindowResolution;

use crate::prelude::*;

pub const WINDOW_RESOLUTION: f32 = 720.0;
pub const BOARD_SIZE_IN_PIXELS: f32 = 600.0;

pub struct ScreenSetupPlugin;

impl Plugin for ScreenSetupPlugin {
    fn build(&self, app: &mut App) {
        app
        
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resolution: WindowResolution::new(
                                WINDOW_RESOLUTION, 
                                WINDOW_RESOLUTION
                            ),
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