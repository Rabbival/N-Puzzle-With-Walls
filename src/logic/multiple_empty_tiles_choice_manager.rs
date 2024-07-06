use crate::prelude::*;

#[derive(Debug, Resource, Default, Clone)]
pub struct MultipleEmptyTilesChoiceManager{
    pub choice_pending: bool,
    pub possible_empty_tiles_locations_and_directions: Option<HashMap<BasicDirection, Tile>>,
}

pub struct MultipleEmptyTilesChoiceManagerPlugin;

impl Plugin for MultipleEmptyTilesChoiceManagerPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<MultipleEmptyTilesChoiceManager>()
            .add_systems(OnEnter(GameState::GameBoardGenerated), cancel_choice_pending)
        ;
    }
}

fn cancel_choice_pending(
    mut multiple_empty_tiles_choice_manager: ResMut<MultipleEmptyTilesChoiceManager>
){
    multiple_empty_tiles_choice_manager.choice_pending = false;
}