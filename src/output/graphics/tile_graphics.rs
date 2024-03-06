use crate::prelude::*;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct StayForNextBoardTag;

pub struct TileGraphicsPlugin;

impl Plugin for TileGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    update_tile_entity_positions.in_set(InputSystemSets::PostInitialChanges),
                ),
            )
            .add_systems(
                OnEnter(GameState::GameBoardGenerated),
                (
                    move_existing_tiles,
                    apply_deferred,
                    despawn_unused_tiles_and_clear_tag,
                    spawn_tiles,
                    declare_post_game_board_gen_changes_done
                )
                    .chain()
                    .in_set(InputSystemSets::MainChanges),
            );
    }
}

fn move_existing_tiles(
    mut event_writer: EventWriter<SpawnTileInLocation>,
    board_query: Query<&TileBoard, With<GameBoard>>,
    tile_dictionary: Query<
        &TileDictionary,
        With<TileDictionaryTag>,
    >,
    mut tile_transforms: Query<&mut Transform, With<Tile>>,
    mut commands: Commands,
) {
    if let Err(error) = move_existing_tiles_inner(
        &mut event_writer,
        &board_query.single().grid,
        &tile_dictionary.single().entity_by_tile,
        &mut tile_transforms,
        &mut commands,
    ) {
        print_entity_related_error(error);
    }
}

fn move_existing_tiles_inner(
    event_writer: &mut EventWriter<SpawnTileInLocation>,
    grid: &Grid<Tile>,
    tile_dictionary: &HashMap<Tile, Option<Entity>>,
    tile_transforms: &mut Query<&mut Transform, With<Tile>>,
    commands: &mut Commands,
) -> Result<(), EntityRelatedCostumeError> {
    for (grid_location, tile_from_cell) in grid.iter() {
        let spawn_location = grid_location.to_world();
        match tile_dictionary.get(tile_from_cell) {
            None => {
                event_writer.send(SpawnTileInLocation {
                    tile: *tile_from_cell,
                    location: spawn_location,
                })
            }
            Some(optional_entity) => match optional_entity {
                None => {
                    return Err(EntityRelatedCostumeError::NoEntity);
                }
                Some(entity) => {
                    if let Ok(mut tile_transform) = tile_transforms.get_mut(*entity) {
                        tile_transform.translation = spawn_location;
                        commands.entity(*entity).insert(StayForNextBoardTag);
                    } else {
                        return Err(EntityRelatedCostumeError::EntityNotInQuery);
                    }
                }
            },
        }
    }
    Ok(())
}

fn despawn_unused_tiles_and_clear_tag(
    tagged_tiles: Query<Entity, (With<Tile>, With<StayForNextBoardTag>)>,
    untagged_tiles: Query<(Entity, &Tile), Without<StayForNextBoardTag>>,
    mut tile_dictionary_query: Query<
        &mut TileDictionary,
        With<TileDictionaryTag>,
    >,
    mut commands: Commands,
) {
    // should only execute if a solved board rerolled
    if tagged_tiles.is_empty() {
        return;
    }

    for (tile_entity, tile) in untagged_tiles.iter() {
        tile_dictionary_query
            .single_mut()
            .entity_by_tile
            .remove(tile);
        commands.entity(tile_entity).despawn_recursive();
    }
    for tile_entity in tagged_tiles.iter() {
        commands.entity(tile_entity).remove::<StayForNextBoardTag>();
    }
}

fn spawn_tiles(
    mut event_listener: EventReader<SpawnTileInLocation>,
    mut commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    font: Res<TileTextFont>,
    mut tile_dictionary: Query<
        &mut TileDictionary,
        With<TileDictionaryTag>,
    >,
) {
    if event_listener.is_empty() {
        return;
    }

    let mut tile_dictionary_instance = tile_dictionary.single_mut();
    for spawn_request in event_listener.read() {
        let tile_to_spawn = spawn_request.tile;
        let spawn_location = Vec3::new(spawn_request.location.x, spawn_request.location.y, 0.0);
        let text_spawn_loc_relative = Vec3::Z;

        let tile_entity_id = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: sprite_atlas.0.clone(),
                    sprite: TextureAtlasSprite::new(tile_to_spawn.tile_type.to_atlas_index()),
                    transform: Transform::from_translation(spawn_location),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                TileBundle {
                    tile: tile_to_spawn,
                    tag: CustomOnScreenTag::Game,
                },
            ))
            .id();

        if tile_to_spawn.tile_type != TileType::Wall {
            let text_color = match tile_to_spawn.tile_type {
                TileType::Numbered => Color::INDIGO,
                TileType::Empty => Color::DARK_GRAY,
                _ => Color::NONE,
            };
            let mut number_to_display = tile_to_spawn.index;
            if let TileType::Numbered = tile_to_spawn.tile_type {
                number_to_display += 1;
            }

            let tile_text_entity_id = commands
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            number_to_display.to_string(),
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 29.0,
                                color: text_color,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
                    },
                    transform: Transform::from_translation(text_spawn_loc_relative),
                    ..default()
                })
                .id();
            commands
                .entity(tile_entity_id)
                .add_child(tile_text_entity_id);
        }

        tile_dictionary_instance
            .entity_by_tile
            .insert(tile_to_spawn, Some(tile_entity_id));
    }
}

fn declare_post_game_board_gen_changes_done(
    mut game_state: ResMut<NextState<GameState>>
){
    game_state.set(GameState::PostGameBoardGenerationChangesDone);
}

fn update_tile_entity_positions(
    mut graphics_switch_tiles_listener: EventReader<UpdateTileLocationGraphics>,
    tile_dictionary: Query<
        &TileDictionary,
        With<TileDictionaryTag>,
    >,
    mut tile_transforms: Query<&mut Transform, With<Tile>>,
) {
    for tile_location_graphic_update_request in graphics_switch_tiles_listener.read() {
        
        
        // info!("got a request: {:?}", tile_location_graphic_update_request);
        
        
        if let Err(move_error) = update_tile_entity_positions_inner(
            &mut tile_transforms,
            &tile_dictionary.single().entity_by_tile,
            tile_location_graphic_update_request.tile,
            tile_location_graphic_update_request.new_location,
        ) {
            print_tile_move_error(move_error);
        }
    }
}

fn update_tile_entity_positions_inner(
    tile_transforms: &mut Query<&mut Transform, With<Tile>>,
    tile_dictionary: &HashMap<Tile, Option<Entity>>,
    tile_to_reposition: Tile,
    new_location_for_tile: GridLocation,
) -> Result<(), TileMoveError> {
    let tile_entity = extract_tile_entity(tile_dictionary, &tile_to_reposition)?;
    if let Ok(mut tile_transform) = tile_transforms.get_mut(tile_entity) {
        tile_transform.translation = new_location_for_tile.to_world();
    } else {
        return Err(TileMoveError::EntityRelated(
            EntityRelatedCostumeError::EntityNotInQuery,
        ));
    }
    Ok(())
}

fn extract_tile_entity(
    tile_dictionary: &HashMap<Tile, Option<Entity>>,
    tile: &Tile,
) -> Result<Entity, TileMoveError> {
    match tile_dictionary.get(tile) {
        None => Err(TileMoveError::EntityRelated(
            EntityRelatedCostumeError::DataStructError(DataStructError::ItemNotFound(*tile)),
        )),
        Some(optional_entity) => match optional_entity {
            None => Err(TileMoveError::EntityRelated(
                EntityRelatedCostumeError::NoEntity,
            )),
            Some(entity) => Ok(*entity),
        },
    }
}
