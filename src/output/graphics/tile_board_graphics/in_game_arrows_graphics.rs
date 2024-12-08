use crate::prelude::*;
use bevy::render::view::RenderLayers;

pub struct InGameArrowsGraphicsPlugin;

impl Plugin for InGameArrowsGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Regular),
            refresh_arrows_upon_new_generation,
        )
        .add_systems(
            Update,
            (
                listen_for_board_lock_changes,
                (
                    listen_for_tile_graphics_updates,
                    show_pressed_arrow_in_just_moved_direction,
                )
                    .chain(),
            )
                .in_set(InputSystemSets::LatestChanges),
        );
    }
}

fn refresh_arrows_upon_new_generation(
    mut empty_tile_arrows: Query<(
        Entity,
        &mut Visibility,
        &mut TextureAtlas,
        &mut CustomOnScreenTag,
        &EmptyTileArrow,
    )>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
    tiles_with_children_query: Query<(&Tile, &Children, &RenderLayers)>,
) {
    if let Err(tile_board_error) = try_get_empties_and_toggle_arrows(
        &mut empty_tile_arrows,
        game_board_query.single(),
        &tiles_with_children_query,
    ) {
        print_tile_board_error(tile_board_error)
    }
}

fn listen_for_board_lock_changes(
    mut lock_change_event_reader: EventReader<SetGameBoardLock>,
    mut empty_tile_arrows: Query<(
        Entity,
        &mut Visibility,
        &mut TextureAtlas,
        &mut CustomOnScreenTag,
        &EmptyTileArrow,
    )>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
    tiles_with_children_query: Query<(&Tile, &Children, &RenderLayers)>,
) {
    for lock_change_request in lock_change_event_reader.read() {
        if lock_change_request.0 {
            for (_, mut arrow_vis, _, mut arrow_on_screen_vis, _) in &mut empty_tile_arrows {
                *arrow_vis = Visibility::Hidden;
                arrow_on_screen_vis.on_own_screen_visibility = Some(Visibility::Hidden);
            }
        } else if let Err(tile_board_error) = try_get_empties_and_toggle_arrows(
            &mut empty_tile_arrows,
            game_board_query.single(),
            &tiles_with_children_query,
        ) {
            print_tile_board_error(tile_board_error)
        }
    }
}

fn listen_for_tile_graphics_updates(
    mut update_tile_graphics_event_reader: EventReader<UpdateTileLocationGraphics>,
    mut empty_tile_arrows: Query<(
        Entity,
        &mut Visibility,
        &mut TextureAtlas,
        &mut CustomOnScreenTag,
        &EmptyTileArrow,
    )>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
    tiles_with_children_query: Query<(&Tile, &Children, &RenderLayers)>,
) {
    let game_board = game_board_query.single();
    if !game_board.ignore_player_input {
        for _event in update_tile_graphics_event_reader.read() {
            if let Err(tile_board_error) = try_get_empties_and_toggle_arrows(
                &mut empty_tile_arrows,
                game_board,
                &tiles_with_children_query,
            ) {
                print_tile_board_error(tile_board_error)
            }
        }
    }
}

fn try_get_empties_and_toggle_arrows(
    empty_tile_arrows: &mut Query<(
        Entity,
        &mut Visibility,
        &mut TextureAtlas,
        &mut CustomOnScreenTag,
        &EmptyTileArrow,
    )>,
    game_board: &TileBoard,
    tiles_with_children_query: &Query<(&Tile, &Children, &RenderLayers)>,
) -> Result<(), TileBoardError> {
    for empty_tile in game_board.try_get_all_empty_tiles()? {
        if let Some(empty_tile_children) =
            try_get_empty_tile_children_if_from_game_board(empty_tile, tiles_with_children_query)
        {
            show_arrows_in_valid_directions(
                empty_tile_arrows,
                game_board,
                empty_tile,
                empty_tile_children,
            );
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
        &EmptyTileArrow,
    )>,
    game_board: &TileBoard,
    empty_tile: &Tile,
    empty_tile_children_entities: &Children,
) {
    let neighbors = game_board.get_direct_neighbors_of_empty(empty_tile.index);
    for (arrow_entity, mut visibility, mut texture_atlas, mut on_screen_tag, arrow) in
        empty_tile_arrows
    {
        if empty_tile_children_entities.contains(&arrow_entity) {
            let new_visibility = if neighbors.contains_key(&arrow.0) {
                texture_atlas.index = empty_tile.to_regular_arrows_atlas_index().unwrap();
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            *visibility = new_visibility;
            on_screen_tag.on_own_screen_visibility = Some(new_visibility);
        }
    }
}

fn show_pressed_arrow_in_just_moved_direction(
    mut event_reader: EventReader<SwitchTilesLogic>,
    mut empty_tile_arrows: Query<(Entity, &mut TextureAtlas, &EmptyTileArrow)>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
    tiles_with_children_query: Query<(&Tile, &Children, &RenderLayers)>,
) {
    let game_board = game_board_query.single();
    if !game_board.ignore_player_input {
        for tile_switch_request in event_reader.read() {
            if let Err(tile_board_error) = show_pressed_arrow_in_just_moved_direction_inner(
                tile_switch_request,
                &mut empty_tile_arrows,
                game_board,
                &tiles_with_children_query,
            ) {
                print_tile_board_error(tile_board_error);
            }
        }
    }
}

fn show_pressed_arrow_in_just_moved_direction_inner(
    tile_switch_request: &SwitchTilesLogic,
    empty_tile_arrows: &mut Query<(Entity, &mut TextureAtlas, &EmptyTileArrow)>,
    tile_board: &TileBoard,
    tiles_with_children_query: &Query<(&Tile, &Children, &RenderLayers)>,
) -> Result<(), TileBoardError> {
    let empty_index = tile_switch_request.empty_tile_index;
    let direction_moved_from = tile_switch_request.move_neighbor_from_direction;
    let empty_tile = tile_board.try_get_empty_tile(empty_index)?;
    if let Some(empty_tile_children) =
        try_get_empty_tile_children_if_from_game_board(empty_tile, tiles_with_children_query)
    {
        for (arrow_entity, mut texture_atlas, arrow) in empty_tile_arrows {
            if empty_tile_children.contains(&arrow_entity) && arrow.0 == direction_moved_from {
                texture_atlas.index = empty_tile.to_highlighted_arrows_atlas_index().unwrap();
            }
        }
    }
    Ok(())
}

fn try_get_empty_tile_children_if_from_game_board<'a>(
    empty_tile: &Tile,
    tiles_with_children_query: &'a Query<(&Tile, &Children, &RenderLayers)>,
) -> Option<&'a Children> {
    for (query_tile, children, &render_layers) in tiles_with_children_query {
        for render_layer in render_layers.iter() {
            if render_layer == 0 && *empty_tile == *query_tile {
                return Some(children);
            }
        }
    }
    None
}
