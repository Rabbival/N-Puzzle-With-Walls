use crate::prelude::*;

use super::error_handler;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, draw_board)
            ;
    }
}

fn draw_board(
    mut commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    board: Res<Board>
){
    let texture_atlas_handle=sprite_atlas.clone().0;
    let mut spawn_pos=Vec2::new(0.0,0.0);
    for row in board.grid{
        for mut tile_from_cell in row{
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(tile_from_cell.to_index()),
                    transform: Transform::from_translation(
                        Vec3::new(
                            spawn_pos.x,
                            spawn_pos.y,
                            0.0
                        )),
                    ..default()
                },
                tile_from_cell
            ));
            tile_from_cell.translation=Some(spawn_pos);

            spawn_pos.x+=ATLAS_CELL_SQUARE_SIZE;
        }
        spawn_pos.y-=ATLAS_CELL_SQUARE_SIZE;
        spawn_pos.x=0.0;
    }

}

pub fn switch_tile_entity_positions(
    board: Res<Board>,
    first: &GridLocation, 
    second: &GridLocation
) -> Result<(),error_handler::InitializationError>
{
    let mut first_tile_translation=get_tile_translation(board.clone(), first)?;
    let mut second_tile_translation=get_tile_translation(board.clone(), second)?;
    let temp_translation=first_tile_translation

    Ok(())
}

fn get_tile_translation(board: Board, tile_location: &GridLocation) 
-> Result<Vec2,error_handler::InitializationError>{
    let tile=board[tile_location];
    match tile.translation{
        None => {
            Err(InitializationError::NoTileTranslationConfigured(tile.tile_type))
        }
        Some(translation) => {
            Ok(translation)
        }
    }
}