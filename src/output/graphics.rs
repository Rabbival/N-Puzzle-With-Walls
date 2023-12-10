use crate::{prelude::*, logic::tile_dictionary};
use bevy::{prelude::*, utils::HashMap};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, spawn_tiles);
    }
}

fn spawn_tiles(
    mut commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    board_query: Query<&TileBoard, With<GameBoard>>,
    mut tile_dictionary: Query<&mut tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>
){
    let mut tile_dictionary_instance=tile_dictionary.single_mut();
    for (grid_location, cell_reference) in board_query.single().grid.iter(){
        let rw_lock = cell_reference.as_ref();
        let mut write = rw_lock.write().unwrap();
        if let Some(tile_from_cell) = &mut *write{
            let spawn_location_before_atlas_square_size=grid_location.to_world();
            let entity_id=commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: sprite_atlas.clone().0.clone(),
                    sprite: TextureAtlasSprite::new(tile_from_cell.to_atlas_index()),
                    transform: Transform::from_translation(
                        Vec3::new(
                            spawn_location_before_atlas_square_size.x*ATLAS_CELL_SQUARE_SIZE,
                            spawn_location_before_atlas_square_size.y*ATLAS_CELL_SQUARE_SIZE,
                            0.0
                        )),
                    ..default()
                },
                *tile_from_cell
            )).id();
            tile_from_cell.tile_entity=Some(entity_id);
            tile_dictionary_instance.entity_by_tile_type.insert(tile_from_cell.tile_type, Some(entity_id));
        }
    }
}

pub fn switch_tile_entity_positions(
    mut tiles: Query<&mut Transform, With<Tile>>,
    grid: &InteriorMutGrid<Tile>,
    first_grid_location: &GridLocation, 
    second_grid_location: &GridLocation
) -> Result<(),TileMoveError>
{
    let first_tile_entity=extract_tile_entity(grid, first_grid_location)?;
    let second_tile_entity=extract_tile_entity(grid, second_grid_location)?;
    if let Ok([mut transform_first, mut transform_second]) = 
        tiles.get_many_mut([first_tile_entity, second_tile_entity]) {
            std::mem::swap(&mut *transform_first, &mut *transform_second);
    }else{
        return Err(TileMoveError::EntityRelated(EntityRelatedCustomError::EntityNotInQuery));
    }
    Ok(())
}

fn extract_tile_entity(
    grid: &InteriorMutGrid<Tile>,
    grid_location: &GridLocation
) -> Result<Entity,TileMoveError>
{
    match grid[grid_location]{
        None => {return Err(TileMoveError::NoTileInCell(grid_location.clone()))},
        Some(tile_in_cell) => {
            match tile_in_cell.tile_entity{
                None=> {Err(TileMoveError::EntityRelated(EntityRelatedCustomError::NoEntity))},
                Some(entity)=> {Ok(entity)}
            }
        }
    }
}

pub fn move_existing_tiles_after_reset(
    grid: &mut InteriorMutGrid<Tile>,
    mut tiles: Query<(Entity, &mut Tile, &mut Transform)>,
    tile_dictionary: &HashMap<TileType,Option<Entity>>,
)-> Result<(),EntityRelatedCustomError>
{
    for (grid_location, cell_reference) in grid.iter(){
        let rw_lock = cell_reference.as_ref();
        let mut write = rw_lock.write().unwrap();
        if let Some(tile_from_cell) = &mut *write{
            let spawn_location_before_atlas_square_size=grid_location.to_world();
            match tile_dictionary.remove(&tile_from_cell.tile_type){
                None=> { return Err(EntityRelatedCustomError::ItemNotInMap
                    (ItemNotFoundInMapError::EntityNotFoundInMap)); },
                Some(optional_entity)=> { 
                    match optional_entity{
                        None=>{return Err(EntityRelatedCustomError::NoEntity);},
                        Some(entity)=>{
                            tile_from_cell.tile_entity=optional_entity;
                            if let Ok((_,_,tile_transform)) = tiles.get_mut(entity) {
                                tile_transform.into_inner().translation= Vec3::new(
                                    spawn_location_before_atlas_square_size.x*ATLAS_CELL_SQUARE_SIZE, 
                                    spawn_location_before_atlas_square_size.y*ATLAS_CELL_SQUARE_SIZE, 
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