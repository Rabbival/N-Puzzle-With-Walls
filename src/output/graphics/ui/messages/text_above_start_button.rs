use crate::prelude::*;
#[derive(Component)]
pub struct TextAboveStartButton;

pub struct TextAboveStartButtonPlugin;

impl Plugin for TextAboveStartButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    /*(*/   warn_of_unapplied_changes
                        .run_if(resource_changed::<UnappliedMenuWallCount>()),
                    //cancel_unapplied_changes_warning).chain(),
                    show_board_couldnt_be_generated_message
                )
                    .run_if(in_state(AppState::Menu)),
            );
    }
}

fn warn_of_unapplied_changes(
    mut text_above_start_button_query: Query<&mut Text, With<TextAboveStartButton>>
) {
    let text_above_start_button = &mut text_above_start_button_query.single_mut().sections[0].value;
    *text_above_start_button = TextAboveStartButtonType::UnappliedChanges.to_string();
}

fn show_board_couldnt_be_generated_message(
    mut event_listener: EventReader<ShowGenerationError>,
    mut text_above_start_button_query: Query<&mut Text, With<TextAboveStartButton>>
) {
    for _ in event_listener.read() {
        let text_above_start_button = &mut text_above_start_button_query.single_mut().sections[0].value;
        *text_above_start_button = TextAboveStartButtonType::CouldntGenerateBoard.to_string();
    }
}
