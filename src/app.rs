#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::{prelude::*, costume_event::app_event};
use bevy::app::AppExit;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app
        //bevy basics
        .add_plugins(ScreenSetupPlugin)
        //costume basics
        .add_plugins((
            SystemSetsPlugin,
            EventPlugins,
            GameStatePlugin,
            ErrorHandlerPlugin
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

        .add_systems(Update, listen_for_app_close_request)
        ;

    app.run();
}


fn listen_for_app_close_request(
    mut end_game_listener: EventReader<app_event::EndGame>,
    mut app_exit_events: EventWriter<AppExit>,    
){
    for _ in end_game_listener.read(){
        app_exit_events.send(AppExit);
    }
}