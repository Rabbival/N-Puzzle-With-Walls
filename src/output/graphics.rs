use crate::{prelude::*, logic::tile_dictionary, costume_event::{reset_event, move_tile_event}};
use bevy::{prelude::*, utils::HashMap};

use super::print_to_console;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, spawn_tiles)
            .add_systems(Update, (
                    switch_tile_entity_positions,
                    move_existing_tiles_after_reset,
                )
                .chain()
                .in_set(CostumeSystemSets::ChangesBasedOnInput)
            )
            //.add_systems(Update, debug_text_position.run_if(run_once()))
            ;
    }
}

fn spawn_tiles(
    mut commands: Commands,
    sprite_atlas: Res<SpriteAtlas>,
    font: Res<TileTextFont>,
    mut board_query: Query<&mut TileTypeBoard, With<GameBoard>>,
    mut tile_dictionary: Query<&mut tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>
){
    let mut tile_dictionary_instance=tile_dictionary.single_mut();
    for (grid_location, cell_reference) in board_query.single_mut().grid.iter_mut(){
        if let Some(tile_type_from_cell) = cell_reference{
            let grid_location_in_world=grid_location.to_world();
            let tile_spawn_location=Vec3::new(
                grid_location_in_world.x,
                grid_location_in_world.y,
                0.0
            );
            let text_spawn_loc_relative=Vec3::Z;

            let tile_entity_id=commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: sprite_atlas.0.clone(),
                    sprite: TextureAtlasSprite::new(tile_type_from_cell.to_atlas_index()),
                    transform: Transform::from_translation(tile_spawn_location),
                    ..default()
                },
                *tile_type_from_cell
            )).with_children(|parent|{
                parent.spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                                match tile_type_from_cell.to_number(){
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
                *tile_type_from_cell, 
                Some(tile_entity_id)
            );
        }
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


fn move_existing_tiles_after_reset(
    mut graphics_reset_listener: EventReader<reset_event::ResetBoardGraphics>,
    mut board_query: Query<&mut TileTypeBoard, With<GameBoard>>,
    tile_dictionary: Query<&tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>,
    mut tile_transforms: Query<(&mut Transform, With<TileType>)>,
){
    for _reset_request in graphics_reset_listener.read(){
        if let Err(error) = move_existing_tiles_after_reset_inner(
            &mut board_query.single_mut().grid,
            &tile_dictionary.single().entity_by_tile_type,
            &mut tile_transforms
        ){
            print_to_console::print_entity_related_error(error);
        }
    }
}

fn move_existing_tiles_after_reset_inner(
    grid: &mut Grid<TileType>,
    tile_dictionary: &HashMap<TileType,Option<Entity>>,
    tile_transforms: &mut Query<(&mut Transform, With<TileType>)>,
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