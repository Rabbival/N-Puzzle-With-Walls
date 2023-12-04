use crate::prelude::*;

pub const WIDTH: f32 = 960.0;
pub const HEIGHT: f32 = 540.0;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WIDTH, HEIGHT).into(),
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