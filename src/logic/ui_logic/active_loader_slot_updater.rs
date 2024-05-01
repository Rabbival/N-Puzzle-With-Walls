use crate::prelude::*;

pub struct ActiveLoaderSlotUpdaterPlugin;

impl Plugin for ActiveLoaderSlotUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, (
                show_slot,
                set_slot_text,
                set_slot_layout_preview
            ).after(show_currently_displayed_saved_layouts_screen)
        );
    }
}

fn show_slot(
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    mut loader_screen_actions_query: Query<(&LoaderScreenAction, &mut CustomOnScreenTag)>,
){
    for slot_set_request in event_reader.read(){
        for (loader_action, mut layout_slot_on_screen_tag)
            in loader_screen_actions_query.iter_mut()
        {
            if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action {
                if layout_slot == slot_set_request.slot_to_set {
                    layout_slot_on_screen_tag.on_own_screen_visibility = Some(Visibility::Visible);
                }
            }
        }
    }
}

fn set_slot_text(
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &Children)>,
    mut layout_slot_text_query: Query<&mut Text>,
    domain_board_query: Query<&DomainBoard>,
){
    for loader_slot_set_request in event_reader.read(){
        match domain_board_query.get(loader_slot_set_request.layout_entity){
            Ok(domain_board_to_set_text_to) => {
                if let Err(entity_error) = set_slot_text_inner(
                    domain_board_to_set_text_to,
                    loader_slot_set_request.slot_to_set,
                    &loader_screen_actions_query,
                    &mut layout_slot_text_query,
                ){
                    print_entity_related_error(entity_error);
                }
            },
            Err(_query_entity_error) => print_entity_related_error(EntityRelatedCostumeError::EntityNotInQuery)
        };
    }
}

fn set_slot_text_inner(
    domain_board_to_set_text_to: &DomainBoard,
    slot_to_set: LoaderScreenSlot,
    loader_screen_actions_query: &Query<(&LoaderScreenAction, &Children)>,
    layout_slot_text_query: &mut Query<&mut Text>,
) -> Result<(), EntityRelatedCostumeError> 
{
    for (loader_action, children) in loader_screen_actions_query {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action {
            if layout_slot == slot_to_set {
                for child_entity in children.iter() {
                    let layout_slot_text_result =
                        layout_slot_text_query.get_mut(*child_entity);
                    if let Ok(mut slot_text) = layout_slot_text_result {
                        slot_text.sections[0].value = domain_board_to_set_text_to.to_string_for_button();
                        return Ok(());
                    }
                }
                return Err(EntityRelatedCostumeError::EntityNotInQuery)
            }
        }
    }
    Ok(())
}

fn set_slot_layout_preview(
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    loader_screen_actions_query: Query<(&LoaderScreenAction, &Children)>,
    tile_board_query: Query<&TileBoard>,
    layout_preview_parent_node_query: Query<Entity, With<LayoutPreviewParentNode>>,
    mut commands: Commands
){
    for loader_slot_set_request in event_reader.read(){
        match tile_board_query.get(loader_slot_set_request.layout_entity){
            Ok(tile_board_to_preview) => {
                if let Err(entity_error) = set_slot_layout_preview_inner(
                    tile_board_to_preview,
                    loader_slot_set_request.slot_to_set,
                    &loader_screen_actions_query,
                    &layout_preview_parent_node_query,
                    &mut commands
                ){
                    print_entity_related_error(entity_error);
                }
            },
            Err(_query_entity_error) => print_entity_related_error(EntityRelatedCostumeError::EntityNotInQuery)
        };
    }
}

fn set_slot_layout_preview_inner(
    tile_board_to_preview: &TileBoard,
    slot_to_set: LoaderScreenSlot,
    loader_screen_actions_query: &Query<(&LoaderScreenAction, &Children)>,
    layout_preview_parent_node_query: &Query<Entity, With<LayoutPreviewParentNode>>,
    commands: &mut Commands
) -> Result<(), EntityRelatedCostumeError>
{
    for (loader_action, children) in loader_screen_actions_query {
        if let LoaderScreenAction::ChooseLayoutInSlot(layout_slot) = *loader_action {
            if layout_slot == slot_to_set {
                for child_entity in children.iter() {
                    let layout_slot_preview_node_result =
                        layout_preview_parent_node_query.get(*child_entity);
                    if let Ok(preview_parent_node) = layout_slot_preview_node_result {
                        construct_ui_layout_preview(
                            tile_board_to_preview,
                            preview_parent_node,
                            commands
                        );
                        return Ok(());
                    }
                }
                return Err(EntityRelatedCostumeError::EntityNotInQuery)
            }
        }
    }
    Ok(())
}

fn construct_ui_layout_preview(
    tile_board_to_preview: &TileBoard,
    preview_parent_node: Entity,
    commands: &mut Commands
){
    let mut preview_parent_entity = commands.get_entity(preview_parent_node).unwrap();
    preview_parent_entity.clear_children();
    let tile_board_grid = &tile_board_to_preview.grid;
    let grid_side_length = *tile_board_grid.get_side_length();
    for r in 0..grid_side_length{
        preview_parent_entity.with_children(|parent| {
            let mut row_node = parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });
            for c in 0..grid_side_length{
                let cell_get_result =
                    tile_board_grid.get(&GridLocation::new(r as i32, c as i32));
                let mut background_color : BackgroundColor = Color::NONE.into();
                if let Ok(optional_tile_in_cell) = cell_get_result{
                    if optional_tile_in_cell.is_some(){
                        background_color = Color::INDIGO.into();
                    }
                };
                row_node.with_children(|parent|{
                    parent.spawn((
                        NodeBundle{
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color,
                            ..default()
                        },
                    ));
                });
            }
        });
    }
}