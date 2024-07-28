#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::prelude::game_session_log::GameSessionLogPlugin;
use crate::prelude::*;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app
        //bevy basics
        .add_plugins(ScreenSetupPlugin)
        //custom basics
        .add_plugins((
            SystemSetsPlugin,
            EventPlugin,
            StatePlugin,
            ErrorHandlerPlugin,
        ))
        //custom
        .add_plugins((
            DataBasePlugin,
            InputPlugin,
            AssetLoaderPlugin,
            BoardPlugin,
            GraphicsPlugin,
            TileDictionaryPlugin,
            GameSessionLogPlugin,
        ));

    app.run();
}
