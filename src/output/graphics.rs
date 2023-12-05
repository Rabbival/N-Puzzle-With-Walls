use crate::prelude::*;
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
    mut board_query: Query<&mut Board, With<GameBoard>>,
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
}

pub fn switch_tile_entity_positions(
    mut tiles: Query<&mut Transform, With<Tile>>,
    board: &Board,
    first_grid_location: &GridLocation, 
    second_grid_location: &GridLocation
) -> Result<(),TileMoveError>
{
    let first_tile_entity=extract_tile_entity(board, first_grid_location)?;
    let second_tile_entity=extract_tile_entity(board, second_grid_location)?;
    if let Ok(second_tile_transform) = tiles.get_mut(second_tile_entity) {
        second_tile_transform.into_inner().translation=first_grid_location.to_world();
    }else{
        return Err(TileMoveError::EntityNotInQuery);
    }
    if let Ok(first_tile_transform) = tiles.get_mut(first_tile_entity) {
        first_tile_transform.into_inner().translation=second_grid_location.to_world();
    }else{
        return Err(TileMoveError::EntityNotInQuery);
    }
    Ok(())
}

fn extract_tile_entity(
    board: &Board,
    grid_location: &GridLocation
) -> Result<Entity,TileMoveError>
{
    match board[grid_location].tile_entity{
        None=> {Err(TileMoveError::NoEntity)},
        Some(entity)=> {Ok(entity)}
    }
}

pub fn move_existing_tiles_after_reset(
    board: &mut Board,
    mut tiles: Query<(&mut Transform, &Tile)>
){
    let mut entity_by_tile_type:HashMap<TileType,Option<Entity>>=HashMap::new();
    for (_, tile) in tiles.iter(){
        entity_by_tile_type.insert(tile.tile_type, tile.tile_entity);
    }

    let mut target_pos=Vec2::new(0.0,0.0);
    for row in board.grid{
        for tile_from_cell in row{
            

            target_pos.x+=ATLAS_CELL_SQUARE_SIZE;
        }
        target_pos.y-=ATLAS_CELL_SQUARE_SIZE;
        target_pos.x=0.0;
    }
}