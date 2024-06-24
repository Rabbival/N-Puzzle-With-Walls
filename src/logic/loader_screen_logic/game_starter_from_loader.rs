use crate::logic::board_building::solved_board_builder;
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
    mut set_applied_tag_event_writer: EventWriter<SetAppliedTagForProperty>,
    mut event_reader: EventReader<LoaderScreenActionEvent>,
    saved_layout_query: Query<(&DomainBoard, &TileBoard), (Without<GameBoard>, Without<SolvedBoard>)>,
    mut game_board_query: Query<&mut TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    mut solved_board_query: Query<&mut TileBoard, (With<SolvedBoard>, Without<GameBoard>)>,
    mut applied_board_props_query: Query<&mut BoardProperties, With<AppliedBoardProperties>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    for loader_action in event_reader.read(){
        if let LoaderScreenAction::GenerateBoard(Some(chosen_layout_entity)) =
            loader_action.action
        {
            let applied_board_properties = 
                &mut applied_board_props_query.single_mut();
            match load_chosen_board(
                &mut set_applied_tag_event_writer,
                &chosen_layout_entity,
                &saved_layout_query,
                applied_board_properties,
            ){
                Ok(saved_layout_tile_board) => {
                    match try_making_solved_tile_board_from_tile_board(
                        &saved_layout_tile_board,
                        applied_board_properties
                    ) {
                        Ok(solved_board) => {
                            *game_board_query.single_mut() = saved_layout_tile_board;
                            *solved_board_query.single_mut() = solved_board;
                            game_state.set(GameState::GameBoardGenerated)
                        },
                        Err(generation_err) => {
                            print_board_generation_error(generation_err);
                        }
                    }
                },
                Err(entity_error) => {
                    print_entity_related_error(entity_error);
                }
            }
        }
    }
}

fn load_chosen_board(
    set_applied_tag_event_writer: &mut EventWriter<SetAppliedTagForProperty>,
    entity: &Entity,
    saved_layout_query: &Query<(&DomainBoard, &TileBoard), (Without<GameBoard>, Without<SolvedBoard>)>,
    applied_board_props: &mut BoardProperties,
) -> Result<TileBoard, EntityRelatedCostumeError>
{
    match saved_layout_query.get(*entity){
        Ok((chosen_domain_board, chosen_tiles_board)) => {
            *applied_board_props = chosen_domain_board.board_props;
            applied_board_props.generation_method = BoardGenerationMethod::Load;
            request_applied_option_tags_for_menu_buttons(
                set_applied_tag_event_writer,
                applied_board_props
            );
            Ok(chosen_tiles_board.clone())
        },
        Err(_) => Err(EntityRelatedCostumeError::EntityNotInQuery)
    }
}

fn request_applied_option_tags_for_menu_buttons(
    event_writer: &mut EventWriter<SetAppliedTagForProperty>,
    loaded_board_properties: &BoardProperties
){
    let actions_to_set = vec!(
        MenuButtonAction::ChangeSize(loaded_board_properties.size),
        MenuButtonAction::ChangeEmptyTilesCount(loaded_board_properties.empty_count),
        MenuButtonAction::ChangeSpanningTreeGeneration(loaded_board_properties.tree_traveller_type),
    );
    for action_to_set in actions_to_set{
        event_writer.send(SetAppliedTagForProperty{give_tag_to_variant: action_to_set});
    }
}

fn try_making_solved_tile_board_from_tile_board(tile_board: &TileBoard, applied_props: &BoardProperties)
    -> Result<TileBoard, BoardGenerationError>
{
    let grid_side_length = *tile_board.grid.get_side_length();
    let mut solved_board = TileBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;
    let (wall_locations , _) : (Vec<GridLocation>, Vec<&Tile>) = tile_board.walls_iter().unzip();

    solved_board_builder::empty_tile_board_to_solved(
        &mut solved_board,
        wall_locations,
        applied_props,
        grid_side_length_u32
    )?;

    Ok(solved_board)
}