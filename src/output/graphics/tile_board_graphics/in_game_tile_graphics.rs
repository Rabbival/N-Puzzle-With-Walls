use crate::prelude::*;

const CHOICE_PENDING_ATLAS_INDEX: usize = 3;

pub struct InGameTileGraphicsPlugin;

impl Plugin for InGameTileGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_tile_entity_positions.in_set(InputSystemSets::PostInitialChanges),
                set_possible_empties_sprites
                    .run_if(resource_changed::<MultipleEmptyTilesChoiceManager>),
            ),
        );
    }
}

fn update_tile_entity_positions(
    mut graphics_switch_tiles_listener: EventReader<UpdateTileLocationGraphics>,
    tile_dictionary: Query<&TileDictionary, Without<LoaderScreenSlot>>,
    mut tile_transforms: Query<&mut Transform, With<Tile>>,
) {
    for tile_location_graphic_update_request in graphics_switch_tiles_listener.read() {
        if let Err(move_error) = update_tile_entity_positions_inner(
            &mut tile_transforms,
            tile_dictionary.single(),
            tile_location_graphic_update_request.tile,
            tile_location_graphic_update_request.new_location,
        ) {
            print_tile_move_error(move_error);
        }
    }
}

fn update_tile_entity_positions_inner(
    tile_transforms: &mut Query<&mut Transform, With<Tile>>,
    tile_dictionary: &TileDictionary,
    tile_to_reposition: Tile,
    new_location_for_tile: GridLocation,
) -> Result<(), TileMoveError> {
    let tile_entity = tile_dictionary.extract_tile_entity(&tile_to_reposition)?;
    if let Ok(mut tile_transform) = tile_transforms.get_mut(tile_entity) {
        tile_transform.translation = new_location_for_tile.to_world();
    } else {
        return Err(TileMoveError::EntityRelated(
            EntityRelatedCustomError::EntityNotInQuery,
        ));
    }
    Ok(())
}

fn set_possible_empties_sprites(
    multiple_empty_tiles_choice_manager: Res<MultipleEmptyTilesChoiceManager>,
    tile_dictionary: Query<&TileDictionary, Without<LoaderScreenSlot>>,
    mut tile_sprites_query: Query<&mut TextureAtlas, With<Tile>>,
) {
    if let Some(empty_tile_locations) =
        &multiple_empty_tiles_choice_manager.possible_empty_tiles_locations_and_directions
    {
        let atlas_index = if multiple_empty_tiles_choice_manager.choice_pending {
            CHOICE_PENDING_ATLAS_INDEX
        } else {
            TileType::Empty.to_tiles_atlas_index()
        };
        for tile_in_direct_line in empty_tile_locations {
            if let Err(move_error) = update_tile_sprite(
                atlas_index,
                &mut tile_sprites_query,
                &tile_in_direct_line.tile,
                tile_dictionary.single(),
            ) {
                print_tile_move_error(move_error);
            }
        }
    }
}

fn update_tile_sprite(
    atlas_index: usize,
    tile_sprites_query: &mut Query<&mut TextureAtlas, With<Tile>>,
    empty_tile: &Tile,
    tile_dictionary: &TileDictionary,
) -> Result<(), TileMoveError> {
    let tile_entity = tile_dictionary.extract_tile_entity(empty_tile)?;
    let possible_texture_atlas = tile_sprites_query.get_mut(tile_entity);
    if let Ok(mut texture_atlas) = possible_texture_atlas {
        texture_atlas.index = atlas_index;
    }
    Ok(())
}
