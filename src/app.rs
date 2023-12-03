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
    .init_resource::<CursorPosition>()
    .add_plugins((
        CameraPlugin,
        InputHandlingPlugin,
    ))
    ;

    app.run();
}