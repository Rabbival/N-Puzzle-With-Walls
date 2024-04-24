use crate::prelude::*;

#[derive(Component)]
pub struct SaveWallsLayoutButton;

#[derive(Component)]
pub struct TextAboveSaveButton;

#[derive(Component)]
pub struct SaveWallsLayoutTextTag;

pub struct GameScreenButtonSpawnerPlugin;

impl Plugin for GameScreenButtonSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_save_walls_layout_button);
    }
}

fn spawn_save_walls_layout_button(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut commands: Commands,
) {
    for spawn_request in spawn_event_reader.read() {
        let button_style = &spawn_request.space_bar_looking_button_style;
        let button_text_style = &spawn_request.medium_text_style;
        let tiny_red_text_style = &spawn_request.tiny_red_text_style;
        commands
            .spawn(
                (build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::End,
                    Visibility::Hidden,
                    Some(FlexDirection::ColumnReverse)
                ),
                 simple_on_screen_tag(AppState::Game)
                ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: super::NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                SaveWallsLayoutButton,
                            ))
                            .with_children(|parent| {
                                parent.spawn((TextBundle::from_section(
                                    "Save Walls Layout",
                                    button_text_style.clone(),
                                ), 
                                    ButtonText,
                                    SaveWallsLayoutTextTag
                                ));
                            });
                    });
                parent
                    .spawn((
                        TextBundle::from_section(
                            "",
                            tiny_red_text_style.clone()
                        ),
                        TextAboveSaveButton
                    ));
            });
    }
}