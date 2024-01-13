use crate::{costume_event::ui_spawn_event, prelude::*};

use super::{menu_spawner, ui_graphics};

/// buttons that are always on the screen
#[derive(Component)]
pub struct EternalButton;

#[derive(Component)]
pub struct MenuToggleButton;

pub struct EternalButtonsSpanwerPlugin;

impl Plugin for EternalButtonsSpanwerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_exit_button, spawn_menu_toggling_button).after(menu_spawner::menu_setup),
        );
    }
}

fn spawn_exit_button(
    mut eternal_buttons_event_reader: EventReader<ui_spawn_event::SpawnEternalButtons>,
    mut commands: Commands,
) {
    for eternal_button_event in eternal_buttons_event_reader.read() {
        let button_style = &eternal_button_event.thin_button_style;
        let button_text_style = &eternal_button_event.button_text_style;

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::End,
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            })
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
                                    background_color: ui_graphics::NORMAL_BUTTON.into(),
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
    mut eternal_buttons_event_reader: EventReader<ui_spawn_event::SpawnEternalButtons>,
    mut commands: Commands,
) {
    for eternal_button_event in eternal_buttons_event_reader.read() {
        let button_style = &eternal_button_event.thin_button_style;
        let button_text_style = &eternal_button_event.button_text_style;

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            })
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
                                    background_color: ui_graphics::NORMAL_BUTTON.into(),
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
