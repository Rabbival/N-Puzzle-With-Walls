use crate::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
pub const HOVERED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
pub const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

pub const NORMAL_TEXT_COLOR: Color = Color::WHITE;
pub const RED_TEXT_COLOR: Color = Color::ORANGE_RED;

pub struct MenuGraphicsPlugin;

impl Plugin for MenuGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
    (
                update_button_color,
                (
                    flash_generation_text_red, 
                    reset_color_for_button_text,
                    update_generate_button_text
                )
                    .run_if(in_state(AppState::Menu)),
                (update_wall_tiles_count_visuals)
                    .run_if(resource_changed::<UnappliedMenuWallCount>()),
            ),
        );
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
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn update_wall_tiles_count_visuals(
    unapplied_menu_wall_count: Res<UnappliedMenuWallCount>,
    mut wall_count_text_query: Query<&mut Text, With<WallCountTextTag>>,
) {
    let mut text = wall_count_text_query.single_mut();
    text.sections[0].value = unapplied_menu_wall_count.0.to_string();
}

fn update_generate_button_text(
    mut button_event_listener: EventReader<MenuButtonPressed>,
    mut generation_text_query: Query<&mut Text, With<BoardGenerationTextTag>>,
) {
    for button_event in button_event_listener.read() {
        if let MenuButtonAction::ChangeGenerationMethod(generation_method) 
            = button_event.action 
        {
            let mut text = generation_text_query.single_mut();
            text.sections[0].value = generation_method.to_generation_button_text();
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
            if *button_text_color == RED_TEXT_COLOR {
                *button_text_color = NORMAL_TEXT_COLOR;
            }
        }
    }
}

fn flash_generation_text_red(
    mut event_listener: EventReader<ShowGenerationError>,
    mut generation_text_query: Query<&mut Text, With<BoardGenerationTextTag>>,
) {
    let generation_text_color = &mut generation_text_query.single_mut().sections[0].style.color;
    for _ in event_listener.read() {
        *generation_text_color = RED_TEXT_COLOR;
    }
}

pub fn set_color_to_normal(background_color: &mut BackgroundColor) {
    *background_color = NORMAL_BUTTON.into();
}

pub fn set_color_to_pressed(background_color: &mut BackgroundColor) {
    *background_color = PRESSED_BUTTON.into();
}
