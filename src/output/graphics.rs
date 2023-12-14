use crate::{prelude::*, logic::tile_dictionary, costume_event::reset_event};
use bevy::{prelude::*, utils::HashMap};

use super::print_to_console;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, spawn_tiles)
            .add_systems(Update, 
                move_existing_tiles_after_reset.in_set(CostumeSystemSets::ChangesBasedOnInput)
            )
            ;
    }
}

fn spawn_tiles(
    mut commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    mut board_query: Query<&mut TileTypeBoard, With<GameBoard>>,
    mut tile_dictionary: Query<&mut tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>
){
    let mut tile_dictionary_instance=tile_dictionary.single_mut();
    for (grid_location, cell_reference) in board_query.single_mut().grid.iter_mut(){
        if let Some(tile_type_from_cell) = cell_reference{
            let spawn_location_before_atlas_square_size=grid_location.to_world();
            let entity_id=commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: sprite_atlas.clone().0.clone(),
                    sprite: TextureAtlasSprite::new(tile_type_from_cell.to_atlas_index()),
                    transform: Transform::from_translation(
                        Vec3::new(
                            spawn_location_before_atlas_square_size.x,
                            spawn_location_before_atlas_square_size.y,
                            0.0
                        )),
                    ..default()
                },
                *tile_type_from_cell
            )).id();
            tile_dictionary_instance.entity_by_tile_type.insert(*tile_type_from_cell, Some(entity_id));
        }
    }
}

pub fn switch_tile_entity_positions(
    mut tiles: Query<&mut Transform, With<TileType>>,
    tile_dictionary: &HashMap<TileType,Option<Entity>>,
    grid: &Grid<TileType>,
    first_grid_location: &GridLocation, 
    second_grid_location: &GridLocation
) -> Result<(),TileMoveError>
{
    let first_tile_entity=extract_tile_entity(&tile_dictionary, grid, first_grid_location)?;
    let second_tile_entity=extract_tile_entity(&tile_dictionary, grid, second_grid_location)?;
    if let Ok([mut transform_first, mut transform_second]) = 
        tiles.get_many_mut([first_tile_entity, second_tile_entity]) {
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
    match grid[grid_location]{
        None => {return Err(TileMoveError::NoTileInCell(grid_location.clone()))},
        Some(tile_type_from_cell) => {
            match tile_dictionary.get(&tile_type_from_cell){
                None=> { return Err(TileMoveError::EntityRelated
                    (EntityRelatedCustomError::ItemNotInMap
                        (ItemNotFoundInMapError::EntityNotFoundInMap)
                    )
                );},
                Some(optional_entity)=> {
                    match optional_entity{
                        None=>{return Err(TileMoveError::EntityRelated
                            (EntityRelatedCustomError::NoEntity));},
                        Some(entity)=>{ Ok(*entity) }
                    }
                }
            }
        }
    }
}

fn move_existing_tiles_after_reset(
    mut graphics_reset_listener: EventReader<reset_event::ResetBoardGraphics>,
    mut board_query: Query<&mut TileTypeBoard, With<GameBoard>>,
    tile_dictionary: Query<&tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>,
    tile_transforms: Query<(&mut Transform, With<TileType>)>,
){
    //got to be able to move the mut query, and it doesn't pass any info
    if graphics_reset_listener.is_empty(){
        return;
    }

    if let Err(error) = post_reset_tile_moving(
        &mut board_query.single_mut().grid,
        &tile_dictionary.single().entity_by_tile_type,
        tile_transforms
    ){
        print_to_console::print_entity_related_error(error);
    }
    
    graphics_reset_listener.clear();
}

fn post_reset_tile_moving(
    grid: &mut Grid<TileType>,
    tile_dictionary: &HashMap<TileType,Option<Entity>>,
    mut tile_transforms: Query<(&mut Transform, With<TileType>)>,
)-> Result<(),EntityRelatedCustomError>
{
    for (grid_location, cell_reference) in grid.iter_mut(){
        if let Some(tile_type_from_cell) = cell_reference{
            let spawn_location_before_atlas_square_size=grid_location.to_world();
            match tile_dictionary.get(tile_type_from_cell){
                None=> { return Err(EntityRelatedCustomError::ItemNotInMap
                    (ItemNotFoundInMapError::EntityNotFoundInMap)); },
                Some(optional_entity)=> { 
                    match optional_entity{
                        None=>{return Err(EntityRelatedCustomError::NoEntity);},
                        Some(entity)=>{
                            if let Ok(mut tile_transform) = tile_transforms.get_mut(*entity) {
                                tile_transform.0.translation= Vec3::new(
                                    spawn_location_before_atlas_square_size.x, 
                                    spawn_location_before_atlas_square_size.y, 
                                    0.0
                                );
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