use crate::prelude::*;

#[derive(Debug, Resource, Default, Clone)]
pub struct MultipleEmptyTilesChoiceManager {
    pub choice_pending: bool,
    pub possible_empty_tiles_locations_and_directions: Option<Vec<TileInDirectLine>>,
}

pub struct MultipleEmptyTilesChoiceManagerPlugin;

impl Plugin for MultipleEmptyTilesChoiceManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MultipleEmptyTilesChoiceManager>()
            .add_systems(
                OnEnter(GameState::GameBoardGenerated),
                cancel_choice_pending_upon_new_generation,
            )
            .add_systems(Update, cancel_choice_pending_when_a_tile_moves);
    }
}

fn cancel_choice_pending_upon_new_generation(
    mut multiple_empty_tiles_choice_manager: ResMut<MultipleEmptyTilesChoiceManager>,
) {
    multiple_empty_tiles_choice_manager.choice_pending = false;
}

fn cancel_choice_pending_when_a_tile_moves(
    mut event_reader: EventReader<UpdateTileLocationGraphics>,
    mut multiple_empty_tiles_choice_manager: ResMut<MultipleEmptyTilesChoiceManager>,
) {
    for _graphics_update_request in event_reader.read() {
        multiple_empty_tiles_choice_manager.choice_pending = false;
    }
}
