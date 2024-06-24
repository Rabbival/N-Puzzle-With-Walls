use bevy::render::view::RenderLayers;
use crate::prelude::*;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct StayForNextBoardTag;

pub struct BoardSpawningTileGraphicsPlugin;

impl Plugin for BoardSpawningTileGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                    (
                        move_loader_slot_existing_tiles,
                        despawn_loader_slot_unused_tiles_and_clear_tag,
                        spawn_loader_slot_new_tiles
                    ).chain().after(show_currently_displayed_saved_layouts_screen)
            )
            .add_systems(
                OnEnter(GameState::GameBoardGenerated),
                (
                    move_game_board_existing_tiles,
                    despawn_game_board_unused_tiles_and_clear_tag,
                    spawn_game_board_new_tiles,
                    declare_post_game_board_gen_changes_done
                )
                    .chain()
                    .in_set(InputSystemSets::MainChanges),
            );
    }
}

fn move_game_board_existing_tiles(
    mut event_writer: EventWriter<SpawnTileInLocation>,
    board_query: Query<&TileBoard, With<GameBoard>>,
    tile_dictionary: Query<&TileDictionary, Without<LoaderScreenSlot>>,
    mut tile_transforms: Query<&mut Transform, With<Tile>>,
    mut commands: Commands,
) {
    if let Err(error) = move_existing_tiles_inner(
        &mut event_writer,
        &board_query.single().grid,
        &tile_dictionary.single().entity_by_tile,
        None,
        &mut tile_transforms,
        &mut commands,
    ) {
        print_entity_related_error(error);
    }
}

fn move_loader_slot_existing_tiles(
    mut tile_spawn_event_writer: EventWriter<SpawnTileInLocation>,
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    tile_dictionary_query: Query<(&TileDictionary, &LoaderScreenSlot)>,
    domain_tile_board_query: Query<&TileBoard, With<DomainBoard>>,
    mut tile_transforms: Query<&mut Transform, With<Tile>>,
    mut commands: Commands
){
    for loader_slot_set_request in event_reader.read(){
        for (tile_dictionary, loader_slot) in &tile_dictionary_query{
            if *loader_slot == loader_slot_set_request.slot_to_set{
                match domain_tile_board_query.get(loader_slot_set_request.layout_entity){
                    Ok(domain_tile_board) => {
                        if let Err(entity_error) = move_existing_tiles_inner(
                            &mut tile_spawn_event_writer,
                            &domain_tile_board.grid,
                            &tile_dictionary.entity_by_tile,
                            Some(*loader_slot),
                            &mut tile_transforms,
                            &mut commands,
                        ){
                            print_entity_related_error(entity_error);
                        }
                    },
                    Err(_query_entity_error) => print_entity_related_error(EntityRelatedCostumeError::EntityNotInQuery)
                };
            }
        }
    }
}

fn move_existing_tiles_inner(
    event_writer: &mut EventWriter<SpawnTileInLocation>,
    grid: &Grid<Tile>,
    tile_dictionary: &HashMap<Tile, Option<Entity>>,
    optional_loader_slot: Option<LoaderScreenSlot>,
    tile_transforms: &mut Query<&mut Transform, With<Tile>>,
    commands: &mut Commands,
) -> Result<(), EntityRelatedCostumeError> {
    for (grid_location, tile_from_cell) in grid.iter() {
        let updated_tile_location = grid_location.to_world();
        match tile_dictionary.get(tile_from_cell) {
            None => {
                event_writer.send(SpawnTileInLocation {
                    optional_loader_slot,
                    tile: *tile_from_cell,
                    location: updated_tile_location,
                });
            }
            Some(optional_entity) => match optional_entity {
                None => {
                    return Err(EntityRelatedCostumeError::NoEntity);
                }
                Some(entity) => {
                    if let Ok(mut tile_transform) = tile_transforms.get_mut(*entity) {
                        tile_transform.translation = updated_tile_location;
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

fn despawn_game_board_unused_tiles_and_clear_tag(
    mut tile_dictionary_query: Query<&mut TileDictionary, Without<LoaderScreenSlot>>,
    tagged_tiles: Query<Entity, (With<Tile>, With<StayForNextBoardTag>)>,
    untagged_tiles: Query<(Entity, &Tile, &LoaderSlotOwnershipTag), Without<StayForNextBoardTag>>,
    mut commands: Commands,
) {
    if tagged_tiles.is_empty() {
        return;
    }

    despawn_unused_tiles_and_clear_tag_inner(
        &mut tile_dictionary_query.single_mut().entity_by_tile,
        None,
        &tagged_tiles,
        &untagged_tiles,
        &mut commands,
    );
}

fn despawn_loader_slot_unused_tiles_and_clear_tag(
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    mut tile_dictionary_query: Query<(&mut TileDictionary, &LoaderScreenSlot)>,
    tagged_tiles: Query<Entity, (With<Tile>, With<StayForNextBoardTag>)>,
    untagged_tiles: Query<(Entity, &Tile, &LoaderSlotOwnershipTag), Without<StayForNextBoardTag>>,
    mut commands: Commands
){
    if tagged_tiles.is_empty() {
        return;
    }
    
    for loader_slot_set_request in event_reader.read(){
        for (mut tile_dictionary, loader_slot) in &mut tile_dictionary_query{
            if *loader_slot == loader_slot_set_request.slot_to_set{
                despawn_unused_tiles_and_clear_tag_inner(
                    &mut tile_dictionary.entity_by_tile,
                    Some(*loader_slot),
                    &tagged_tiles,
                    &untagged_tiles,
                    &mut commands,
                );
            }
        }
    }
}

fn despawn_unused_tiles_and_clear_tag_inner(
    tile_dictionary: &mut HashMap<Tile, Option<Entity>>,
    optional_loader_slot: Option<LoaderScreenSlot>, 
    tagged_tiles: &Query<Entity, (With<Tile>, With<StayForNextBoardTag>)>,
    untagged_tiles: &Query<(Entity, &Tile, &LoaderSlotOwnershipTag), Without<StayForNextBoardTag>>,
    commands: &mut Commands,
) {
    for (tile_entity, tile, loader_slot_tag) in untagged_tiles {
        if loader_slot_tag.0 == optional_loader_slot{
            tile_dictionary.remove(tile);
            commands.entity(tile_entity).despawn_recursive();
        }
    }
    for tile_entity in tagged_tiles {
        commands.entity(tile_entity).remove::<StayForNextBoardTag>();
    }
}

fn spawn_game_board_new_tiles(
    mut spawn_tile_addons_event_writer: EventWriter<SpawnTileAddons>,
    mut event_reader: EventReader<SpawnTileInLocation>,
    mut tile_dictionary_query: Query<&mut TileDictionary, Without<LoaderScreenSlot>>,
    tile_sprite_atlas: Res<TileSpriteAtlas>,
    mut commands: Commands,
) {
    let tile_dictionary = &mut tile_dictionary_query.single_mut().entity_by_tile;
    for spawn_request in event_reader.read() {
        if spawn_request.optional_loader_slot.is_none(){
            spawn_tile_in_location(
                &mut spawn_tile_addons_event_writer,
                spawn_request,
                tile_dictionary,
                None,
                &tile_sprite_atlas,
                &mut commands
            )
        }
    }
}

fn spawn_loader_slot_new_tiles(
    mut spawn_tile_addons_event_writer: EventWriter<SpawnTileAddons>,
    mut event_reader: EventReader<SpawnTileInLocation>,
    mut tile_dictionary_query: Query<(&mut TileDictionary, &LoaderScreenSlot)>,
   tile_sprite_atlas: Res<TileSpriteAtlas>,
    mut commands: Commands,
) {
    for spawn_request in event_reader.read() {
        for (mut tile_dictionary, loader_slot) in &mut tile_dictionary_query{
            if let Some(request_loader_slot) = spawn_request.optional_loader_slot{
                if request_loader_slot == *loader_slot{
                    spawn_tile_in_location(
                        &mut spawn_tile_addons_event_writer,
                        &spawn_request,
                        &mut tile_dictionary.entity_by_tile,
                        Some(*loader_slot),
                        &tile_sprite_atlas,
                        &mut commands
                    )
                }
            }
        }
    }
}

fn spawn_tile_in_location(
    spawn_tile_addons_event_writer: &mut EventWriter<SpawnTileAddons>,
    spawn_request: &SpawnTileInLocation,
    tile_dictionary: &mut HashMap<Tile, Option<Entity>>,
    optional_loader_slot: Option<LoaderScreenSlot>,
    tile_sprite_atlas: &TileSpriteAtlas,
    commands: &mut Commands
){
    let tile_to_spawn = spawn_request.tile;
    let spawn_location = Vec3::from((spawn_request.location.truncate(), 0.0));
    let loader_slot_ownership_tag = LoaderSlotOwnershipTag(optional_loader_slot);
    let on_screen_tags = match optional_loader_slot{
        None => MultipleOnScreenTags(vec!(simple_on_screen_tag(AppState::Game))),
        Some(_) => {
            MultipleOnScreenTags(vec!(
                simple_on_screen_tag(AppState::Game),
                simple_on_screen_tag(AppState::Loader)
            ))
        }
    };

    let tile_entity_id = commands
        .spawn((
            SpriteSheetBundle {
                atlas: TextureAtlas{
                    layout: tile_sprite_atlas.atlas_handle.clone(),
                    index: tile_to_spawn.to_tiles_atlas_index()
                },
                texture: tile_sprite_atlas.image_handle.clone(),
                transform: Transform::from_translation(spawn_location),
                ..default()
            },
            TileBundle {
                tile: tile_to_spawn,
                on_screen_tags,
                loader_slot_ownership_tag,
                render_layers: RenderLayers::layer(loader_slot_ownership_tag.to_render_layer())
            },
        ))
        .id();

    if tile_to_spawn.tile_type != TileType::Wall {
        spawn_tile_addons_event_writer.send(SpawnTileAddons{
            tile_to_add_to: tile_to_spawn,
            tile_entity_id,
            tile_loader_slot_ownership_tag: loader_slot_ownership_tag,
        });
    }

    tile_dictionary.insert(tile_to_spawn, Some(tile_entity_id));
}

fn declare_post_game_board_gen_changes_done(
    mut game_state: ResMut<NextState<GameState>>
){
    game_state.set(GameState::PostGameBoardGenerationChangesDone);
}