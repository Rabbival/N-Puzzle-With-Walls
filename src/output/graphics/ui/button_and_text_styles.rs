use crate::prelude::*;

pub struct ButtonAndTextStylesPlugin;

impl Plugin for ButtonAndTextStylesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            button_and_text_styles_emitter
        );
    }
}

pub fn button_and_text_styles_emitter(
    mut button_event_writer: EventWriter<SpawnButtons>,
    mut big_button_event_writer: EventWriter<SpawnBigButtons>,
    mut tile_count_buttons_event_writer: EventWriter<SpawnTileCountButtons>,
    mut eternal_buttons_event_writer: EventWriter<SpawnEternalButtons>,
) {
    let button_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        ..default()
    };

    let big_button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(80.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let big_button_text_style = TextStyle {
        font_size: 60.0,
        ..default()
    };

    let thin_button_style = Style {
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let eternal_button_text_style = TextStyle {
        font_size: 50.0,
        ..default()
    };

    let small_text_style = TextStyle {
        font_size: 30.0,
        ..default()
    };

    let tiny_red_text_style = TextStyle {
        font_size: 22.0,
        color: Color::RED,
        ..default()
    };

    let save_walls_layout_button_style = Style {
        width: Val::Px(450.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let save_walls_layout_button_text_style = TextStyle {
        font_size: 40.0,
        ..default()
    };

    eternal_buttons_event_writer.send(SpawnEternalButtons {
        thin_button_style: thin_button_style.clone(),
        button_text_style: eternal_button_text_style.clone(),
    });
    button_event_writer.send(SpawnButtons {
        button_style: button_style.clone(),
        button_text_style: button_text_style.clone(),
    });
    big_button_event_writer.send(SpawnBigButtons {
        big_button_style,
        big_button_text_style,
        save_walls_layout_button_style,
        save_walls_layout_button_text_style,
        tiny_red_text_style
    });
    tile_count_buttons_event_writer.send(SpawnTileCountButtons {
        regular_button_style: button_style,
        thin_button_style,
        button_text_style,
        small_text_style
    });
}