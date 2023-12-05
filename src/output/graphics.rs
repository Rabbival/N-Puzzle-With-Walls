use crate::prelude::*;

#[derive(Resource, Default, PartialEq, Eq)]
pub enum TilesSpawned{
    #[default]
    First,
    NotFirst
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TilesSpawned>()
            .add_systems(PostStartup, draw_board)
            ;
    }
}

fn draw_board(
    tiles_spawned: ResMut<TilesSpawned>,
    commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    board_query: Query<&Board, With<GameBoard>>,
    tiles: Query<&mut Transform, With<Tile>>
){
    let tiles_spawned_value = tiles_spawned.into_inner();
    if *tiles_spawned_value == TilesSpawned::First {
        spawn_tiles(commands, sprite_atlas.clone().0, board_query.single().clone());
        *tiles_spawned_value = TilesSpawned::NotFirst;
    } else{
        move_existing_tiles(board_query.single().clone(), tiles);
    }
}

fn spawn_tiles(
    mut commands: Commands,
    atlas_handle: Handle<TextureAtlas>,
    board: Board
){
    let mut spawn_pos=Vec2::new(0.0,0.0);
    for row in board.grid{
        for mut tile_from_cell in row{
            let entity_id=commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(tile_from_cell.to_atlas_index()),
                    transform: Transform::from_translation(
                        Vec3::new(
                            spawn_pos.x,
                            spawn_pos.y,
                            0.0
                        )),
                    ..default()
                },
                tile_from_cell
            )).id();
            tile_from_cell.tile_entity=Some(entity_id);

            spawn_pos.x+=ATLAS_CELL_SQUARE_SIZE;
        }
        spawn_pos.y-=ATLAS_CELL_SQUARE_SIZE;
        spawn_pos.x=0.0;
    }
}

fn move_existing_tiles(
    board: Board,
    tiles: Query<&mut Transform, With<Tile>>
){
    //TODO
}

pub fn switch_tile_entity_positions(
    mut tiles: Query<&mut Transform, With<Tile>>,
    board: &Board,
    first_grid_location: &GridLocation, 
    second_grid_location: &GridLocation
) -> Result<(),TileMoveError>
{
    let temp_position_first;
    let temp_position_second;

    let first_tile_entity=extract_tile_entity(board, first_grid_location)?;
    let second_tile_entity=extract_tile_entity( board, second_grid_location)?;
    if let Ok(first_transform_immutable) = tiles.get(first_tile_entity) {
        temp_position_first=first_transform_immutable.translation;
    }else{
        return Err(TileMoveError::EntityNotInQuery);
    }
    if let Ok(transform) = tiles.get_mut(second_tile_entity) {
        let second_tile_transform=transform.into_inner();
        temp_position_second=second_tile_transform.translation;
        second_tile_transform.translation=temp_position_first;
    }else{
        return Err(TileMoveError::EntityNotInQuery);
    }
    if let Ok(transform) = tiles.get_mut(first_tile_entity) {
        let first_tile_transform=transform.into_inner();
        first_tile_transform.translation=temp_position_second;
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