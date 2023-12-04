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
    for row in board.grid{
        for tile_from_cell in row{
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
                    //sprite: TextureAtlasSprite::new(//index here),
                    //transform: 
                    ..default()
                },
                GridLocation::new(10, 10),
                tile_from_cell
            ));
        }
    }

}

pub fn move_tile_entity(tile: Entity, from: &GridLocation, to: &GridLocation){

}