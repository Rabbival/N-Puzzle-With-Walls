use crate::prelude::*;

pub struct MenuGraphicsPlugin;

impl Plugin for MenuGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    (update_wall_tiles_count_visuals)
                        .run_if(resource_changed::<UnappliedMenuWallCount>()),
                    flash_generation_text_red,
                    update_generate_button_text
                )
                    .run_if(in_state(AppState::Menu)),
            ),
        );
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

fn flash_generation_text_red(
    mut event_listener: EventReader<ShowGenerationError>,
    mut generation_text_query: Query<&mut Text, With<BoardGenerationTextTag>>,
) {
    for _ in event_listener.read() {
        let generation_text_color = &mut generation_text_query.single_mut().sections[0].style.color;
        *generation_text_color = super::RED_TEXT_COLOR;
    }
}