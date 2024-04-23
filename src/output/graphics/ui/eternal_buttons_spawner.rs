use crate::prelude::*;

/// buttons that are always on the screen
#[derive(Component)]
pub struct EternalButton;

#[derive(Component)]
pub struct MenuToggleButton;

pub struct EternalButtonsSpawnerPlugin;

impl Plugin for EternalButtonsSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_exit_button, spawn_menu_toggling_button)
        );
    }
}

fn spawn_exit_button(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut commands: Commands,
) {
    for spawn_request in spawn_event_reader.read() {
        let button_style = &spawn_request.thin_button_style;
        let button_text_style = &spawn_request.big_text_style;

        commands
            .spawn(build_node_bundle_with_full_percentage_style(
                AlignItems::Start,
                JustifyContent::End,
                Visibility::Visible,
                None
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
                                EternalButton,
                                EternalButtonAction::EndGame,
                            ))
                            .with_children(|parent| {
                                parent.spawn((TextBundle::from_section(
                                    "X",
                                    button_text_style.clone(),
                                ),));
                            });
                    });
            });
    }
}

fn spawn_menu_toggling_button(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut commands: Commands,
) {
    for spawn_request in spawn_event_reader.read() {
        let button_style = &spawn_request.thin_button_style;
        let button_text_style = &spawn_request.big_text_style;

        commands
            .spawn(build_node_bundle_with_full_percentage_style(
                AlignItems::Start,
                JustifyContent::Start,
                Visibility::Visible,
                None
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
                                EternalButton,
                                EternalButtonAction::ToggleMenu,
                                MenuToggleButton,
                            ))
                            .with_children(|parent| {
                                parent.spawn((TextBundle::from_section(
                                    "M",
                                    button_text_style.clone(),
                                ),));
                            });
                    });
            });
    }
}
