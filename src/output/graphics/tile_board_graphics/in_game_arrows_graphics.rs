use bevy::render::view::RenderLayers;
use crate::prelude::*;

pub struct InGameArrowsGraphicsPlugin;

impl Plugin for InGameArrowsGraphicsPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Regular), refresh_arrows_upon_new_generation)
            .add_systems(
          Update, show_arrows_in_valid_directions_if_empty.in_set(InputSystemSets::LateChanges)
        );
    }
}

fn refresh_arrows_upon_new_generation(
    mut empty_tile_arrows: Query<(Entity, &mut Visibility, &mut CustomOnScreenTag, &EmptyTileArrow)>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
    mut tiles_with_children_query: Query<(&Tile, &mut Children, &RenderLayers)>,
){
    if let Err(tile_board_error) = check_type_and_toggle_arrows(
        &mut empty_tile_arrows,
        game_board_query.single(),
        &mut tiles_with_children_query
    ){
        print_tile_board_error(tile_board_error)
    }
}

fn show_arrows_in_valid_directions_if_empty(
    update_tile_graphics_event_reader: EventReader<UpdateTileLocationGraphics>,
    mut empty_tile_arrows: Query<(Entity, &mut Visibility, &mut CustomOnScreenTag, &EmptyTileArrow)>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
    mut tiles_with_children_query: Query<(&Tile, &mut Children, &RenderLayers)>,
){
    if !update_tile_graphics_event_reader.is_empty(){
        if let Err(tile_board_error) = check_type_and_toggle_arrows(
            &mut empty_tile_arrows,
            game_board_query.single(),
            &mut tiles_with_children_query
        ){
            print_tile_board_error(tile_board_error)
        }
    }
}

fn check_type_and_toggle_arrows(
    empty_tile_arrows: &mut Query<(Entity, &mut Visibility, &mut CustomOnScreenTag, &EmptyTileArrow)>,
    game_board: &TileBoard,
    tiles_with_children_query: &mut Query<(&Tile, &mut Children, &RenderLayers)>,
) -> Result<(), TileBoardError>{
    for empty_tile in game_board.try_get_all_empty_tiles()?{
        for (query_tile, mut children, &render_layers) in &mut *tiles_with_children_query{
            for render_layer in render_layers.iter(){
                if render_layer == 0 && *empty_tile == *query_tile{
                    if let Err(tile_board_error) = show_arrows_in_valid_directions(
                        empty_tile_arrows,
                        game_board,
                        empty_tile.index,
                        &mut children
                    ){
                        print_tile_board_error(tile_board_error);
                    }
                }
            }
        }
    }
    Ok(())
}

fn show_arrows_in_valid_directions(
    empty_tile_arrows: &mut Query<(Entity, &mut Visibility, &mut CustomOnScreenTag, &EmptyTileArrow)>,
    game_board: &TileBoard,
    empty_tile_index: usize,
    empty_tile_children_entities: &mut Children
) -> Result<(), TileBoardError>{
    let neighbors = 
        game_board.get_direct_neighbors_of_empty(empty_tile_index)?;
    for (
        arrow_entity,
        mut visibility,
        mut on_screen_tag,
        arrow
    ) in empty_tile_arrows
    {
        if empty_tile_children_entities.contains(&arrow_entity){
            let new_visibility = if neighbors.contains_key(&arrow.0){
                Visibility::Visible
            }else{
                Visibility::Hidden
            };
            *visibility = new_visibility;
            on_screen_tag.on_own_screen_visibility = Some(new_visibility);
        }
    }
    Ok(())
}