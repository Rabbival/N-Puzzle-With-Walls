use crate::prelude::*;

/// which option is applied to the current board,
/// intended for when changing but not applying and reopening the
#[derive(Component)]
pub struct AppliedOptionTag;

/// which option is currently selected
#[derive(Component)]
pub struct SelectedOptionTag;

#[derive(Component)]
pub struct ApplyButtonTag;

#[derive(Component)]
pub struct WallCountTextTag;

#[derive(Component)]
pub struct BoardGenerationTextTag;

#[derive(Component)]
pub struct TreeGenerationOptionsTag;

#[derive(Component)]
pub struct ButtonText;

pub struct MenuSpawnerPlugin;

impl Plugin for MenuSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
                (
                    spawn_generate_button,
                    spawn_size_options,
                    spawn_generation_options,
                    spawn_tile_counter,
                )
            );
    }
}

fn spawn_generate_button(
    mut big_button_event_reader: EventReader<SpawnBigButtons>,
    mut commands: Commands,
) {
    for big_button_event in big_button_event_reader.read() {
        let button_style = &big_button_event.big_button_style;
        let button_text_style = &big_button_event.big_button_text_style;
        let tiny_red_text_style = &big_button_event.tiny_red_text_style;

        commands
            .spawn((
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::End,
                    Visibility::Hidden,
                    Some(FlexDirection::ColumnReverse)
                ),
                MultipleOnScreenTags(vec![
                    CustomOnScreenTag{
                        screen: AppState::Menu,
                        on_own_screen_visibility: None
                    },
                    CustomOnScreenTag{
                        screen: AppState::Builder,
                        on_own_screen_visibility: None
                    },
                    CustomOnScreenTag{
                        screen: AppState::Loader,
                        on_own_screen_visibility: None
                    },
                ]),
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::PURPLE.into(),
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
                                MenuButtonAction::MainButtonPressed,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        BoardGenerationMethod::default().to_generation_button_text(), 
                                        button_text_style.clone()
                                    ),
                                    BoardGenerationTextTag,
                                    ButtonText,
                                ));
                            });
                    });
                //unapplied changes warning text
                parent
                    .spawn((
                       TextBundle::from_section(
                           "",
                           tiny_red_text_style.clone()
                       ),
                       TextAboveStartButton
                    ));
            });
    }
}

fn spawn_generation_options(
    mut button_event_reader: EventReader<SpawnButtons>,
    mut commands: Commands,
) {
    for button_event in button_event_reader.read() {
        let button_style = &button_event.button_style;
        let button_text_style = &button_event.button_text_style;

        commands
            .spawn((
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::Start,
                    Visibility::Hidden,
                    Some(FlexDirection::Column)
                ),
                CustomOnScreenTag{
                    screen: AppState::Menu,
                    on_own_screen_visibility: None
                },
            ))
            .with_children(|parent| {
                //title
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            background_color: Color::INDIGO.into(),
                            ..default()
                        },
                        CustomOnScreenTag{
                            screen: AppState::Menu,
                            on_own_screen_visibility: None
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            String::from("Generation Method"),
                            button_text_style.clone(),
                        ));
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                //buttons
                                for generation_method in BoardGenerationMethod::as_list() {
                                    let mut button_entity = parent.spawn((
                                        ButtonBundle {
                                            style: button_style.clone(),
                                            background_color: super::NORMAL_BUTTON_COLOR.into(),
                                            ..default()
                                        },
                                        MenuButtonAction::ChangeGenerationMethod(generation_method),
                                    ));
                                    button_entity.with_children(|parent| {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                generation_method.to_string(),
                                                button_text_style.clone(),
                                            ),
                                            ButtonText,
                                        ));
                                    });
                                    if generation_method == BoardGenerationMethod::default() {
                                        button_entity.insert(SelectedOptionTag);
                                        button_entity.insert(AppliedOptionTag);
                                    }
                                }
                            });
                    });
            });
    }
}

fn spawn_size_options(
    mut button_event_reader: EventReader<SpawnButtons>,
    mut commands: Commands,
) {
    for button_event in button_event_reader.read() {
        let button_style = &button_event.button_style;
        let button_text_style = &button_event.button_text_style;

        commands
            .spawn((
               build_node_bundle_with_full_percentage_style(
                   AlignItems::Center,
                   JustifyContent::Start,
                   Visibility::Hidden,
                   None
               ),
               CustomOnScreenTag{
                   screen: AppState::Menu,
                   on_own_screen_visibility: None
               },
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::INDIGO.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        //title
                        parent.spawn(TextBundle::from_section(
                            String::from("Board Size"),
                            button_text_style.clone(),
                        ));
                        //buttons
                        for board_size in BoardSize::as_list() {
                            let mut button_entity = parent.spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: super::NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                MenuButtonAction::ChangeSize(board_size),
                            ));
                            button_entity.with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        board_size.to_string(),
                                        button_text_style.clone(),
                                    ),
                                    ButtonText,
                                ));
                            });
                            if board_size == BoardSize::default() {
                                button_entity.insert(SelectedOptionTag);
                                button_entity.insert(AppliedOptionTag);
                            }
                        }
                    });
            });
    }
}

fn spawn_tile_counter(
    mut button_event_reader: EventReader<SpawnTileCountButtons>,
    mut commands: Commands,
) {
    for button_event in button_event_reader.read() {
        let regular_button_style = &button_event.regular_button_style;
        let thin_button_style = &button_event.thin_button_style;
        let button_text_style = &button_event.button_text_style;
        let small_text_style = &button_event.small_text_style;

        commands
            .spawn((
               build_node_bundle_with_full_percentage_style(
                   AlignItems::Center,
                   JustifyContent::End,
                   Visibility::Hidden,
                   None
               ),
               CustomOnScreenTag{
                   screen: AppState::Menu,
                   on_own_screen_visibility: None
               },
            ))
            .with_children(|parent| {
                parent
                    // tree generation options
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,
                                ..default()
                            },
                            background_color: Color::INDIGO.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        CustomOnScreenTag{
                            screen: AppState::Menu,
                            on_own_screen_visibility: Some(Visibility::Hidden)
                        },
                        TreeGenerationOptionsTag,
                    ))
                    .with_children(|parent| {
                        //title
                        parent.spawn(TextBundle::from_section(
                            " Default Wall \n   Placing ",
                            small_text_style.clone(),
                        ));
                        //buttons
                        for traveller_type in GridTravellerType::as_list() {
                            let mut button_entity = parent.spawn((
                                ButtonBundle {
                                    style: regular_button_style.clone(),
                                    background_color: super::NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                MenuButtonAction::ChangeSpanningTreeGeneration(traveller_type),
                            ));
                            button_entity.with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        traveller_type.to_button_option_text(),
                                        small_text_style.clone(),
                                    ),
                                    ButtonText,
                                ));
                            });
                            if traveller_type == GridTravellerType::default() {
                                button_entity.insert(SelectedOptionTag);
                                button_entity.insert(AppliedOptionTag);
                            }
                        }
                    });

                //actual tile counters
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::INDIGO.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        //title - empty
                        parent.spawn(TextBundle::from_section(
                            String::from("Empty Tiles"),
                            button_text_style.clone(),
                        ));
                        //buttons - empty
                        for (action, text) in [
                            (MenuButtonAction::ChangeEmptyTilesCount(1), "1"),
                            (MenuButtonAction::ChangeEmptyTilesCount(2), "2"),
                        ] {
                            let mut empty_tiles_count = 0;
                            if let MenuButtonAction::ChangeEmptyTilesCount(number) = action {
                                empty_tiles_count = number;
                            }
                            let mut button_entity = parent.spawn((
                                ButtonBundle {
                                    style: thin_button_style.clone(),
                                    background_color: super::NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                action,
                            ));
                            button_entity.with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(text, button_text_style.clone()),
                                    ButtonText,
                                ));
                            });
                            if empty_tiles_count == DEFAULT_EMPTY_COUNT {
                                button_entity.insert(SelectedOptionTag);
                                button_entity.insert(AppliedOptionTag);
                            }
                        }

                        //title - walls
                        parent.spawn(TextBundle::from_section(
                            String::from("Wall Tiles"),
                            button_text_style.clone(),
                        ));
                        //buttons - walls
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                for (action, text) in [
                                    (
                                        Some(MenuButtonAction::ChangeWallTilesCount(
                                            WallTilesChange::Decrease,
                                        )),
                                        "<",
                                    ),
                                    (None, " _ "),
                                    (
                                        Some(MenuButtonAction::ChangeWallTilesCount(
                                            WallTilesChange::Increase,
                                        )),
                                        ">",
                                    ),
                                ] {
                                    if action.is_none() {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                text,
                                                button_text_style.clone(),
                                            ),
                                            WallCountTextTag,
                                        ));
                                    } else {
                                        let mut arrow_button_entity = parent.spawn((
                                            ButtonBundle {
                                                style: thin_button_style.clone(),
                                                background_color: super::NORMAL_BUTTON_COLOR.into(),
                                                ..default()
                                            },
                                            action.unwrap(),
                                        ));
                                        arrow_button_entity.with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    text,
                                                    button_text_style.clone(),
                                                ),
                                                ButtonText,
                                            ));
                                        });
                                    }
                                }
                            });
                        //apply button
                        let mut apply_button_entity = parent.spawn((
                            ButtonBundle {
                                style: regular_button_style.clone(),
                                background_color: super::NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Apply),
                            SelectedOptionTag,
                            ApplyButtonTag,
                        ));
                        apply_button_entity.with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section("Apply", button_text_style.clone()),
                                ButtonText,
                            ));
                        });
                    });
            });
    }
}