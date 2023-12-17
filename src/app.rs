#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::prelude::*;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app
        //bevy basics
        .add_plugins(ScreenSetupPlugin)
        //costume basics
        .add_plugins((
            SystemSetsPlugin,
            EventPlugins
        ))
        //costume
        .add_plugins((
            CameraPlugin,
            InputPlugin,
            AssetLoaderPlugin,
            BoardPlugins,
            GraphicsPlugin,
            TileDictionaryPlugin
        ))
        ;

    app.run();
}