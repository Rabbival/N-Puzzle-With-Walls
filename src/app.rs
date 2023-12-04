use crate::prelude::*;

pub const WINDOW_SIZE: f32 = GRID_SIZE as f32 * ATLAS_CELL_SQUARE_SIZE / CAMERA_ZOOM;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(
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
    .init_resource::<CursorPosition>()
    .add_plugins((
        CameraPlugin,
        InputHandlerPlugin,
        AssetLoaderPlugin,
        BoardManagerPlugin,
        BoardPlugin,
        GraphicsPlugin
    ))
    ;

    app.run();
}