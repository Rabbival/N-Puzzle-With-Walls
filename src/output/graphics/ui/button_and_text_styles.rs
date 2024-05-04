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
    mut spawn_texts_and_buttons_writer: EventWriter<SpawnTextsAndButtons>,
) {
    let board_props_button_style = Style {
        width: Val::Px(270.0),
        height: Val::Px(270.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
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
    let space_bar_looking_button_style = Style {
        width: Val::Px(450.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let common_button_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
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
    
    let giant_text_style = TextStyle {
        font_size: 60.0,
        ..default()
    };
    let big_text_style = TextStyle {
        font_size: 50.0,
        ..default()
    };
    let medium_text_style = TextStyle {
        font_size: 40.0,
        ..default()
    };
    let small_text_style = TextStyle {
        font_size: 30.0,
        ..default()
    };
    let tiny_text_style = TextStyle {
        font_size: 22.0,
        ..default()
    };
    let tiny_red_text_style = TextStyle {
        font_size: 22.0,
        color: Color::RED,
        ..default()
    };

    spawn_texts_and_buttons_writer.send(SpawnTextsAndButtons {
        board_props_button_style,
        big_button_style,
        space_bar_looking_button_style,
        common_button_style,
        thin_button_style,
        giant_text_style,
        big_text_style,
        medium_text_style,
        small_text_style,
        tiny_text_style,
        tiny_red_text_style,
    });
}