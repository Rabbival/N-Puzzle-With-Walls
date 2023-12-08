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
    mut board_query: Query<&mut Board<Tile>, With<GameBoard>>,
    mut tile_dictionary: Query<&mut tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>
){
    let mut spawn_pos=Vec2::new(0.0,0.0);
    for row in &mut board_query.single_mut().grid{
        for tile_from_cell in row{
            let entity_id=commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: sprite_atlas.clone().0.clone(),
                    sprite: TextureAtlasSprite::new(tile_from_cell.to_atlas_index()),
                    transform: Transform::from_translation(
                        Vec3::new(
                            spawn_pos.x,
                            spawn_pos.y,
                            0.0
                        )),
                    ..default()
                },
                *tile_from_cell
            )).id();
            tile_from_cell.tile_entity=Some(entity_id);

            spawn_pos.x+=ATLAS_CELL_SQUARE_SIZE;
        }
        spawn_pos.y-=ATLAS_CELL_SQUARE_SIZE;
        spawn_pos.x=0.0;
    }

    let mut tile_dictionary_instance=tile_dictionary.single_mut();
    for (entity, tile, _) in tiles.iter_mut(){
        tile_dictionary_instance.insert(tile.tile_type, Some(entity));
    }
    
}

pub fn switch_tile_entity_positions(
    mut tiles: Query<&mut Transform, With<Tile>>,
    board: &Board<Tile>,
    first_grid_location: &GridLocation, 
    second_grid_location: &GridLocation
) -> Result<(),TileMoveError>
{
    let first_tile_entity=extract_tile_entity(board, first_grid_location)?;
    let second_tile_entity=extract_tile_entity(board, second_grid_location)?;
    if let Ok([mut transform_first, mut transform_second]) = 
        tiles.get_many_mut([first_tile_entity, second_tile_entity]) {
            std::mem::swap(&mut *transform_first, &mut *transform_second);
    }else{
        return Err(TileMoveError::EntityRelated(EntityRelatedCustomError::EntityNotInQuery));
    }
    Ok(())
}

fn extract_tile_entity(
    board: &Board<Tile>,
    grid_location: &GridLocation
) -> Result<Entity,TileMoveError>
{
    match board[grid_location].tile_entity{
        None=> {Err(TileMoveError::EntityRelated(EntityRelatedCustomError::NoEntity))},
        Some(entity)=> {Ok(entity)}
    }
}

pub fn move_existing_tiles_after_reset(
    board: &mut Board<Tile>,
    mut tiles: Query<(Entity, &mut Tile, &mut Transform)>,
    tile_dictionary: &HashMap<TileType,Option<Entity>>,
)-> Result<(),EntityRelatedCustomError>
{
    let mut target_pos=Vec3::new(0.0,0.0,0.0);
    for row in &mut board.grid{
        for tile_from_cell in row{
            match tile_dictionary.remove(&tile_from_cell.tile_type){
                None=> { return Err(EntityRelatedCustomError::ItemNotInMap
                    (ItemNotFoundInMapError::EntityNotFoundInMap)); },
                Some(optional_entity)=> { 
                    match optional_entity{
                        None=>{return Err(EntityRelatedCustomError::NoEntity);},
                        Some(entity)=>{
                            tile_from_cell.tile_entity=optional_entity;
                            if let Ok((_,_,tile_transform)) = tiles.get_mut(entity) {
                                tile_transform.into_inner().translation=target_pos;
                            }else{
                                return Err(EntityRelatedCustomError::EntityNotInQuery);
                            }
                        }
                    }
                }
            }
            target_pos.x+=ATLAS_CELL_SQUARE_SIZE;
        }
        target_pos.y-=ATLAS_CELL_SQUARE_SIZE;
        target_pos.x=0.0;
    }
    Ok(())
}