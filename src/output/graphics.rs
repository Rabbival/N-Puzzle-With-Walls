use crate::prelude::*;

use super::error_handler;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, draw_board)
            ;
    }
}

fn draw_board(
    mut commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    board_query: Query<&Board, With<GameBoard>>
){
    let texture_atlas_handle=sprite_atlas.clone().0;
    let mut spawn_pos=Vec2::new(0.0,0.0);
    for row in board_query.single().grid{
        for tile_from_cell in row{
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
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
            ));
            spawn_pos.x+=ATLAS_CELL_SQUARE_SIZE;
        }
        spawn_pos.y-=ATLAS_CELL_SQUARE_SIZE;
        spawn_pos.x=0.0;
    }

}

pub fn switch_tile_entity_positions(
    query: Query<&mut Transform, With<Tile>>,
    first_grid_location: &GridLocation, 
    second_grid_location: &GridLocation,
) -> Result<(),error_handler::SearchError>
{
    let mut first_tile_translation=Vec3::default();
    let mut second_tile_translation=Vec3::default();
    let (first_world_position, mut first_found)=(first_grid_location.to_world(), false);
    let (second_world_position, mut second_found)=(second_grid_location.to_world(), false);
    for mut transform in query.iter(){
        let current_tile_world_pos = Vec2::new(
            transform.translation.x,
            transform.translation.y
        );
        if current_tile_world_pos == first_world_position {
            first_tile_translation=transform.translation;
            first_found=true;
            if second_found {
                break;
            }
        }
        if current_tile_world_pos == second_world_position {
            second_tile_translation=transform.translation;
            second_found=true;
            if first_found {
                break;
            }
        }
    }
    let temp_translation=first_tile_translation;
    first_tile_translation=second_tile_translation;
    second_tile_translation=temp_translation;
    
    Ok(())
}