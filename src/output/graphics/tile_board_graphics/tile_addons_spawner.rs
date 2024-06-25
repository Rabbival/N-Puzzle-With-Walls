use bevy::render::view::RenderLayers;
use crate::prelude::*;

pub struct TileAddonsSpawnerPlugin;

impl Plugin for TileAddonsSpawnerPlugin{
    fn build(&self, app: &mut App) {
        app.
            add_systems(
                Update,
                    (
                        spawn_text_for_tile,
                        spawn_arrows_for_tile_if_empty
                    ).in_set(InputSystemSets::PostMainChanges)
            );
    }
}


fn spawn_text_for_tile(
    mut event_reader: EventReader<SpawnTileAddons>,
    tile_text_font: Res<TileTextFont>,
    mut commands: Commands
){
    for addons_request in event_reader.read(){
        let tile_to_spawn = addons_request.tile_to_add_to;
        let tile_entity_id = addons_request.tile_entity_id;
        let loader_slot_ownership_tag = addons_request.tile_loader_slot_ownership_tag;

        let text_spawn_loc_relative = Vec3::Z;
        let text_color = match tile_to_spawn.tile_type {
            TileType::Numbered => INDIGO_TEXT_COLOR,
            TileType::Empty => GRAY_TEXT_COLOR,
            _ => Color::NONE,
        };
        let mut number_to_display = tile_to_spawn.index;
        if let TileType::Numbered = tile_to_spawn.tile_type {
            number_to_display += 1;
        }

        let tile_text_entity_id = commands
            .spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            number_to_display.to_string(),
                            TextStyle {
                                font: tile_text_font.0.clone(),
                                font_size: BIG_ATLAS_CELL_SQUARE_SIZE*0.88,
                                color: text_color,
                            },
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
                    },
                    transform: Transform::from_translation(text_spawn_loc_relative),
                    ..default()
                },
                RenderLayers::layer(loader_slot_ownership_tag.to_render_layer())
            ))
            .id();
        commands
            .entity(tile_entity_id)
            .add_child(tile_text_entity_id);
    }
}

fn spawn_arrows_for_tile_if_empty(
    mut event_reader: EventReader<SpawnTileAddons>,
    arrow_sprite_atlas: Res<ArrowSpriteAtlas>,
    mut commands: Commands
){
    for addons_request in event_reader.read(){
        if addons_request.tile_to_add_to.tile_type != TileType::Empty {continue;}
        for direction in BasicDirection::collect_all(){
            spawn_arrow_in_direction(
                direction,
                addons_request,
                arrow_sprite_atlas.as_ref(),
                &mut commands
            );
        }
    }
}

fn spawn_arrow_in_direction(
    direction: BasicDirection,
    addons_request: &SpawnTileAddons,
    arrow_sprite_atlas: &ArrowSpriteAtlas,
    commands: &mut Commands
){
    let tile_to_spawn = addons_request.tile_to_add_to;
    let tile_entity_id = addons_request.tile_entity_id;
    let loader_slot_ownership_tag = addons_request.tile_loader_slot_ownership_tag;
    let location_offset =
        Vec3::from((
                     direction.to_world_direction()*(BIG_ATLAS_CELL_SQUARE_SIZE/1.9),
                     2.0
                 ));
    let rotation = direction.opposite_direction().unwrap().to_rotation();
    let arrow_entity_id = commands
        .spawn((
            SpriteSheetBundle {
                atlas: TextureAtlas{
                    layout: arrow_sprite_atlas.atlas_handle.clone(),
                    index: tile_to_spawn.to_regular_arrows_atlas_index().unwrap()
                },
                texture: arrow_sprite_atlas.image_handle.clone(),
                transform: Transform::from_translation(location_offset).with_rotation(rotation),
                ..default()
            },
            CustomOnScreenTag{
                screen: AppState::Game,
                on_own_screen_visibility: Some(Visibility::Visible)
            },
            EmptyTileArrow(direction),
            RenderLayers::layer(loader_slot_ownership_tag.to_render_layer()),
        ))
        .id();
    commands
        .entity(tile_entity_id)
        .add_child(arrow_entity_id);
}