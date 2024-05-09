use crate::prelude::*;

pub struct GameStarterFromLoaderPlugin;

impl Plugin for GameStarterFromLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_systems(
                Update,
                listen_to_game_start_from_loader_requests
            );
    }
}

fn listen_to_game_start_from_loader_requests(
    mut event_reader: EventReader<LoaderScreenActionEvent>,
    saved_layout_query: Query<(&DomainBoard, &TileBoard), Without<SolvedBoard>>,
    mut game_board_query: Query<&mut TileBoard, With<SolvedBoard>>,
    mut applied_board_props_query: Query<&mut BoardProperties, With<AppliedBoardProperties>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    for loader_action in event_reader.read(){
        if let LoaderScreenAction::GenerateBoard(Some(chosen_layout_entity)) =
            loader_action.action
        {
            match start_game_from_loader(
                &chosen_layout_entity,
                &saved_layout_query,
                &mut applied_board_props_query,
            ){
                Ok(mut saved_layout_tile_board) => {
                    saved_layout_tile_board.ignore_player_input = false;
                    *game_board_query.single_mut() = saved_layout_tile_board;
                    game_state.set(GameState::GameBoardGenerated)
                },
                Err(entity_error) => {
                    print_entity_related_error(entity_error);
                }
            }
        }
    }
}

fn start_game_from_loader(
    entity: &Entity,
    saved_layout_query: &Query<(&DomainBoard, &TileBoard), Without<SolvedBoard>>,
    applied_board_props_query: &mut Query<&mut BoardProperties, With<AppliedBoardProperties>>,
) -> Result<TileBoard, EntityRelatedCostumeError>
{
    match saved_layout_query.get(*entity){
        Ok((chosen_domain_board, chosen_tiles_board)) => {
            let mut board_propertied_ref = applied_board_props_query.single_mut();
            board_propertied_ref.size = chosen_domain_board.board_props.size;
            board_propertied_ref.wall_count = chosen_domain_board.board_props.wall_count;
            board_propertied_ref.empty_count = chosen_domain_board.board_props.empty_count;
            Ok(chosen_tiles_board.clone())
        },
        Err(_) => Err(EntityRelatedCostumeError::EntityNotInQuery)
    }
}