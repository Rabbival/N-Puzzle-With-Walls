use crate::prelude::*;

pub mod eternal_buttons_spawner;
pub mod menu_spawner;
pub mod button_and_text_styles;
pub mod messages;
pub mod save_walls_layout_button;
pub mod loader_screen_spawner;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const HOVERED_PRESSED_BUTTON_COLOR_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

pub const NORMAL_TEXT_COLOR: Color = Color::WHITE;
pub const RED_TEXT_COLOR: Color = Color::ORANGE_RED;
pub const GREEN_TEXT_COLOR: Color = Color::LIME_GREEN;


#[derive(Component)]
pub struct ButtonText;

pub struct UiGraphicsPlugin;

impl Plugin for UiGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MenuSpawnerPlugin,
            EternalButtonsSpawnerPlugin,
            MessagesGraphicsPlugin,
            ButtonAndTextStylesPlugin,
            GameScreenButtonSpawnerPlugin,
            LoaderScreenSpawnerPlugin,
        ))
        .add_systems(Update, (
            update_button_color,
            reset_color_for_button_text,
        ));
    }
}

pub fn build_node_bundle_with_full_percentage_style(
    align_items: AlignItems,
    justify_content: JustifyContent,
    visibility: Visibility,
    flex_direction: Option<FlexDirection>
) -> NodeBundle
{
    let flex_direction = flex_direction.unwrap_or_default();
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items,
            justify_content,
            flex_direction,
            ..default()
        },
        visibility,
        ..default()
    }
}

fn update_button_color(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SelectedOptionTag>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON_COLOR_COLOR.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON_COLOR.into(),
            (Interaction::None, None) => NORMAL_BUTTON_COLOR.into(),
        }
    }
}

fn reset_color_for_button_text(
    mut event_listener: EventReader<ResetButtonTextColor>,
    mut generation_text_query: Query<&mut Text, With<ButtonText>>,
) {
    for _ in event_listener.read() {
        for mut button_text in generation_text_query.iter_mut() {
            let button_text_color = &mut button_text.sections[0].style.color;
            if *button_text_color != NORMAL_TEXT_COLOR {
                *button_text_color = NORMAL_TEXT_COLOR;
            }
        }
    }
}

pub fn set_color_to_normal(background_color: &mut BackgroundColor) {
    *background_color = NORMAL_BUTTON_COLOR.into();
}

pub fn set_color_to_pressed(background_color: &mut BackgroundColor) {
    *background_color = PRESSED_BUTTON_COLOR.into();
}

pub fn set_text_section_value_and_color(
    text_section_ref: &mut TextSection, 
    new_color: Option<Color>, 
    new_value: Option<String>
){

    if let Some(text_new_color) =  new_color {
        let text_above_save_button_color = &mut text_section_ref.style.color;
        *text_above_save_button_color = text_new_color;
    }
    if let Some(text_new_value) =  new_value {
        let text_above_save_button_value = &mut text_section_ref.value;
        *text_above_save_button_value = text_new_value.to_string();
    }
}