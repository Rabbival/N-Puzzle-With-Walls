use crate::prelude::*;


pub mod camera;
pub mod ui;
pub mod visibility_tags;
pub mod tile_graphics;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiGraphicsPlugin,
            CameraPlugin,
            TileGraphicsPlugin,
        ))
        .add_systems(
            Update,
            (
                show_only_if_has_specified_screen_tag
                    .run_if(not(resource_changed::<DataBaseManager>)
                        .and_then(not(resource_changed::<DisplayedLoaderScreenNumber>)))
                    .in_set(StateChangeSystemSets::HandleStateChange),
                show_only_if_has_specified_screen_tag
                    .run_if(
                        resource_changed::<DataBaseManager>
                            .or_else(resource_changed::<DisplayedLoaderScreenNumber>)
                    )
                    .in_set(InputSystemSets::PostMainChanges),
                set_visibility_for_entity
            )
        );
    }
}

/// pub so others may be placed relatively
pub fn show_only_if_has_specified_screen_tag(
    app_state: Res<State<AppState>>,
    data_base_manager: Res<DataBaseManager>,
    displayed_loader_screen_number: Res<DisplayedLoaderScreenNumber>,
    mut single_screen_entities: Query<(&mut Visibility, &CustomOnScreenTag), Without<MultipleOnScreenTags>>,
    mut multiple_screen_entities: Query<(&mut Visibility, &MultipleOnScreenTags), Without<CustomOnScreenTag>>,
) {
    if app_state.is_changed() || displayed_loader_screen_number.is_changed() || data_base_manager.is_changed(){
        for (mut visibility, screen_tag) in
            &mut single_screen_entities
        {
            if *app_state == screen_tag.screen {
                if let Some(own_screen_vis) = screen_tag.on_own_screen_visibility {
                    *visibility = own_screen_vis;
                }else{
                    *visibility = Visibility::Visible;
                }
            } else {
                *visibility = Visibility::Hidden;
            }
        }
        'entity_for: for (mut visibility, screen_tags) in
        &mut multiple_screen_entities
        {
            for screen_tag in screen_tags.0.iter() {
                if *app_state == screen_tag.screen {
                    if let Some(own_screen_vis) = screen_tag.on_own_screen_visibility {
                        *visibility = own_screen_vis;
                    } else {
                        *visibility = Visibility::Visible;      
                    }
                    continue 'entity_for;
                }
            }
            *visibility = Visibility::Hidden;
        }
    }
}

fn set_visibility_for_entity(
    mut event_reader: EventReader<SetEntityVisibility>,
    mut query: Query<&mut Visibility>,
) {
    for visibility_set_request in event_reader.read() {
        let entity_to_toggle = visibility_set_request.entity;
        if let Ok(mut entity_visibility) = query.get_mut(entity_to_toggle) {
            *entity_visibility = visibility_set_request.visibility;
        }
    }
}