use crate::prelude::*;

pub struct ErrorHandlerPlugin;

impl Plugin for ErrorHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, board_generation_error_handler);
    }
}

pub fn board_generation_error_handler(
    mut event_reader: EventReader<ShowGenerationError>,
) {
    for generation_error in event_reader.read() {
        print_board_generation_error(generation_error.0);
    }
}