use bevy::render::view::RenderLayers;
use crate::prelude::*;

pub struct InGameArrowsGraphicsPlugin;

impl Plugin for InGameArrowsGraphicsPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Regular), refresh_arrows_upon_new_generation)
            .add_systems(
          Update, 
          (
            show_arrows_in_valid_directions_if_empty,
            show_pressed_arrow_in_just_moved_direction
          ).chain().in_set(InputSystemSets::LateChanges)
        );
    }
}

fn refresh_arrows_upon_new_generation(
    mut empty_tile_arrows: Query<(
        Entity, 
        &mut Visibility, 
        &mut TextureAtlas,
        &mut CustomOnScreenTag, 
        &EmptyTileArrow
    )>,
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
    mut update_tile_graphics_event_reader: EventReader<UpdateTileLocationGraphics>,
    mut empty_tile_arrows: Query<(
        Entity, 
        &mut Visibility, 
        &mut TextureAtlas,
        &mut CustomOnScreenTag, 
        &EmptyTileArrow
    )>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
    mut tiles_with_children_query: Query<(&Tile, &mut Children, &RenderLayers)>,
){
    for _event in update_tile_graphics_event_reader.read(){
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
    empty_tile_arrows: &mut Query<(
        Entity, 
        &mut Visibility, 
        &mut TextureAtlas,
        &mut CustomOnScreenTag, 
        &EmptyTileArrow
    )>,
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
                        empty_tile,
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
    empty_tile_arrows: &mut Query<(
        Entity, 
        &mut Visibility, 
        &mut TextureAtlas,
        &mut CustomOnScreenTag, 
        &EmptyTileArrow
    )>,
    game_board: &TileBoard,
    empty_tile: &Tile,
    empty_tile_children_entities: &mut Children
) -> Result<(), TileBoardError>{
    let neighbors = 
        game_board.get_direct_neighbors_of_empty(empty_tile.index)?;
    for (
        arrow_entity,
        mut visibility,
        mut texture_atlas,
        mut on_screen_tag,
        arrow
    ) in empty_tile_arrows
    {
        if empty_tile_children_entities.contains(&arrow_entity){
            let new_visibility = if neighbors.contains_key(&arrow.0){
                texture_atlas.index = empty_tile.to_regular_arrows_atlas_index().unwrap();
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

fn show_pressed_arrow_in_just_moved_direction(
    mut event_reader: EventReader<SwitchTilesLogic>,
    mut empty_tile_arrows: Query<(&mut TextureAtlas,&EmptyTileArrow)>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
){
    for tile_switch_request in event_reader.read(){
        if let Err(tile_board_error) = show_pressed_arrow_in_just_moved_direction_inner(
            tile_switch_request,
            &mut empty_tile_arrows,
            game_board_query.single()
        ){
            print_tile_board_error(tile_board_error);
        }
    }
}

fn show_pressed_arrow_in_just_moved_direction_inner(
    tile_switch_request: &SwitchTilesLogic,
    empty_tile_arrows: &mut Query<(&mut TextureAtlas,&EmptyTileArrow)>,
    tile_board: &TileBoard
)-> Result<(), TileBoardError>{
    let empty_index = tile_switch_request.empty_tile_index;
    let direction_moved_from = tile_switch_request.move_neighbor_from_direction;
    let empty_tile = tile_board.try_get_empty_tile(empty_index)?;
    for (mut texture_atlas, arrow) in empty_tile_arrows{
        if arrow.0 == direction_moved_from{
            texture_atlas.index = empty_tile.to_highlighted_arrows_atlas_index().unwrap();
        }
    }
    Ok(())
}