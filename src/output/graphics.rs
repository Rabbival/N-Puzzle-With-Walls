use crate::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, draw_board);
    }
}

fn draw_board(
    mut commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    board: Res<Board>
){
    let atlas_handle=&sprite_atlas.0;
    let mut spawn_pos=Vec2::new(0.0,0.0);
    for row in board.grid{
        for tile_from_cell in row{
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
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
            spawn_pos.x+=ATLAS_CELL_SQUARE_SIZE;
        }
        spawn_pos.y-=ATLAS_CELL_SQUARE_SIZE;
        spawn_pos.x=0.0;
    }

}

/* 
pub fn move_tile_entity(tile: Entity, from: &GridLocation, to: &GridLocation){

}
*/