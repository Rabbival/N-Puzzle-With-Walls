use crate::{prelude::*, logic::tile_dictionary, costume_event::{move_tile_event, board_set_event}};
use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct StayForNextBoardTag;

pub struct TileGraphicsPlugin;

impl Plugin for TileGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(PostStartup, spawn_tiles)
            .add_systems(Update, (
                switch_tile_entity_positions
                    .in_set(InputSystemSets::ChangesBasedOnInput),
                (
                    move_existing_tiles,
                    despawn_unused_tiles_and_clear_tag,
                    spawn_tiles,   
                )
                .chain()
                .in_set(InputSystemSets::PostMainChanges)
            ))
            ;
    }
}


fn move_existing_tiles(
    mut event_writer: EventWriter<board_set_event::SpawnTileInLocation>,
    mut event_listener: EventReader<board_set_event::BuildNewBoard>,
    mut board_query: Query<&mut TileTypeBoard, With<GameBoard>>,
    tile_dictionary: Query<&tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>,
    mut tile_transforms: Query<&mut Transform, With<TileType>>,
    mut commands: Commands
){
    for event in event_listener.read(){
        if let Err(error) = move_existing_tiles_inner(
            &mut event_writer,
            &event.reroll_solved,
            &mut board_query.single_mut().grid,
            &tile_dictionary.single().entity_by_tile_type,
            &mut tile_transforms,
            &mut commands
        ){
            print_entity_related_error(error);
        }
    }
}

fn move_existing_tiles_inner(
    event_writer: &mut EventWriter<board_set_event::SpawnTileInLocation>,
    solved_rerolled: &bool,
    grid: &mut Grid<TileType>,
    tile_dictionary: &HashMap<TileType,Option<Entity>>,
    tile_transforms: &mut Query<&mut Transform, With<TileType>>,
    commands: &mut Commands
)-> Result<(),EntityRelatedCustomError>
{
    for (grid_location, cell_reference) in grid.iter_mut(){
        if let Some(tile_type_from_cell) = cell_reference{
            let spawn_location=grid_location.to_world();
            match tile_dictionary.get(tile_type_from_cell){
                // the tile doesn't exist yet and thus should be created there when we're done
                None=> { 
                    if *solved_rerolled {
                        event_writer.send(board_set_event::SpawnTileInLocation{
                            tiletype: *tile_type_from_cell,
                            location: spawn_location
                        })
                    }else{
                        return Err(EntityRelatedCustomError::ItemNotInMap
                            (ItemNotFoundInMapError::EntityNotFoundInMap));
                    }
                 },
                // the tile exists and should therefore be moved
                Some(optional_entity)=> { 
                    match optional_entity{
                        None=>{return Err(EntityRelatedCustomError::NoEntity);},
                        Some(entity)=>{
                            if let Ok(mut tile_transform) = tile_transforms.get_mut(*entity) {
                                tile_transform.translation= spawn_location;
                                if *solved_rerolled{
                                    commands.entity(*entity).insert(StayForNextBoardTag);
                                }
                            }else{
                                return Err(EntityRelatedCustomError::EntityNotInQuery);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn despawn_unused_tiles_and_clear_tag(
    tagged_tiles: Query<Entity, (With<TileType>, With<StayForNextBoardTag>)>,
    untagged_tiles: Query<(Entity, &TileType), Without<StayForNextBoardTag>>,
    mut tile_dictionary_query: Query<
        &mut tile_dictionary::TileDictionary, 
        With<tile_dictionary::TileDictionaryTag>
    >,
    mut commands: Commands
){
    // the only time the function should be used is when a solved board of a smaller size was generated
    if tagged_tiles.is_empty(){
        return;
    }

    // delete all unused
    for (tile_entity, tile_type) in untagged_tiles.iter(){
        tile_dictionary_query.single_mut().entity_by_tile_type.remove(tile_type);
        commands.entity(tile_entity).despawn_recursive();
    }
    // delete tags from the ones left
    for tile_entity in tagged_tiles.iter(){
        commands.entity(tile_entity).remove::<StayForNextBoardTag>();
    }
}

fn spawn_tiles(
    mut event_listener: EventReader<board_set_event::SpawnTileInLocation>,
    mut commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    font: Res<TileTextFont>,
    mut tile_dictionary: Query<&mut tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>
){
    if event_listener.is_empty(){
        return;
    }
    let mut tile_dictionary_instance=tile_dictionary.single_mut();
    for spawn_request in event_listener.read(){
        let tile_type_to_spawn = spawn_request.tiletype;
        let spawn_location = Vec3::new(
            spawn_request.location.x,
            spawn_request.location.y,
            0.0
        );
        let text_spawn_loc_relative=Vec3::Z;        

        let tile_entity_id=commands.spawn((
            SpriteSheetBundle {
                texture_atlas: sprite_atlas.0.clone(),
                sprite: TextureAtlasSprite::new(tile_type_to_spawn.to_atlas_index()),
                transform: Transform::from_translation(spawn_location),
                visibility: Visibility::Hidden, 
                ..default()
            },
            TileBundle{
                tile_type: tile_type_to_spawn,
                tag: OnScreenTag::Game
            },
        )).with_children(|parent|{
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                            match tile_type_to_spawn.to_number(){
                                None=> String::from(""),
                                Some(number)=> number.to_string()
                            },
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 29.0,
                                color: Color::INDIGO
                            }
                        )],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
                },
                transform: Transform::from_translation(text_spawn_loc_relative),
                ..default()
            });
        }).id();

        tile_dictionary_instance.entity_by_tile_type.insert(
            tile_type_to_spawn, 
            Some(tile_entity_id)
        );
    }
}


fn switch_tile_entity_positions(
    mut graphics_switch_tiles_listener: EventReader<move_tile_event::SwitchTilesGraphics>,
    mut board_query: Query<&mut TileTypeBoard, With<GameBoard>>,
    tile_dictionary: Query<&tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>,
    mut tile_transforms: Query<&mut Transform, With<TileType>>,
){
    for tile_switch_request in graphics_switch_tiles_listener.read(){
        if let Err(move_error) = switch_tile_entity_positions_inner(
            &mut tile_transforms,
            &tile_dictionary.single().entity_by_tile_type,
            &board_query.single_mut().grid,
            &tile_switch_request.first_grid_location,
            &tile_switch_request.second_grid_location,
        ){
            print_tile_move_error(move_error);
        }
    }
}

fn switch_tile_entity_positions_inner(
    tile_transforms: &mut Query<&mut Transform, With<TileType>>,
    tile_dictionary: &HashMap<TileType,Option<Entity>>,
    grid: &Grid<TileType>,
    first_grid_location: &GridLocation, 
    second_grid_location: &GridLocation
) -> Result<(),TileMoveError>
{
    let first_tile_entity=extract_tile_entity(tile_dictionary, grid, first_grid_location)?;
    let second_tile_entity=extract_tile_entity(tile_dictionary, grid, second_grid_location)?;
    if let Ok([mut transform_first, mut transform_second]) = 
        tile_transforms.get_many_mut([first_tile_entity, second_tile_entity]) {
            std::mem::swap(&mut *transform_first, &mut *transform_second);
    }else{
        return Err(TileMoveError::EntityRelated(EntityRelatedCustomError::EntityNotInQuery));
    }
    Ok(())
}

fn extract_tile_entity(
    tile_dictionary: &HashMap<TileType,Option<Entity>>,
    grid: &Grid<TileType>,
    grid_location: &GridLocation
) -> Result<Entity,TileMoveError>
{
    match grid.get(grid_location){
        None => {Err(TileMoveError::NoTileInCell(*grid_location))},
        Some(tile_type_from_cell) => {
            match tile_dictionary.get(tile_type_from_cell){
                None=> {Err(TileMoveError::EntityRelated
                    (EntityRelatedCustomError::ItemNotInMap
                        (ItemNotFoundInMapError::EntityNotFoundInMap)
                    )
                )},
                Some(optional_entity)=> {
                    match optional_entity{
                        None=>{Err(TileMoveError::EntityRelated
                            (EntityRelatedCustomError::NoEntity))},
                        Some(entity)=>{ Ok(*entity) }
                    }
                }
            }
        }
    }
}