use crate::{prelude::*, costume_event::ui_event, logic::board_building::board_properties};

use super::menu_graphics;

/// which option is applied to the current board,
/// intended for when changing but not applying and reopening the menu
#[derive(Component)]
pub struct AppliedOptionTag;

/// which option is currently selected
#[derive(Component)]
pub struct SelectedOptionTag;

#[derive(Component)]
pub struct ApplyButtonTag;

#[derive(Component)]
pub struct WallCountTextTag;

pub struct MenuSpanwerPlugin;

impl Plugin for MenuSpanwerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, menu_setup)
            .add_systems(Startup, (
                    spawn_generate_button,
                    spawn_size_options,
                    spawn_generation_options,
                    spawn_tile_counter
                )
                .after(menu_setup)
            );
    }
}


fn menu_setup(
    mut button_event_writer: EventWriter<ui_event::SpawnButtons>,
    mut big_button_event_writer: EventWriter<ui_event::SpawnBigButtons>,
    mut tile_count_buttons_event_writer: EventWriter<ui_event::SpawnTileCountButtons>
) {
    let button_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0 ,
        ..default()
    };

    let big_button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(80.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let big_button_text_style = TextStyle {
        font_size: 60.0 ,
        ..default()
    };

    let thin_button_style = Style {
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    button_event_writer.send(ui_event::SpawnButtons{
        button_style: button_style.clone(),
        button_text_style: button_text_style.clone()
    });
    big_button_event_writer.send(ui_event::SpawnBigButtons{
        big_button_style,
        big_button_text_style
    });
    tile_count_buttons_event_writer.send(ui_event::SpawnTileCountButtons{
        regular_button_style: button_style,
        thin_button_style,
        button_text_style
    });
}

fn spawn_generate_button(
    mut big_button_event_reader: EventReader<ui_event::SpawnBigButtons>,
    mut commands: Commands
){
    for big_button_event in big_button_event_reader.read(){
        let button_style=&big_button_event.big_button_style;
        let button_text_style=&big_button_event.big_button_text_style;

        commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            OnScreenTag::Menu,
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
            }).with_children(|parent| {
                parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: menu_graphics::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::GenerateBoard
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Generate",
                        button_text_style.clone(),
                    ));
                });
            });
        });
    }
}

fn spawn_generation_options(
    mut button_event_reader: EventReader<ui_event::SpawnButtons>,
    mut commands: Commands
){
    for button_event in button_event_reader.read(){
        let button_style=&button_event.button_style;
        let button_text_style=&button_event.button_text_style;

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                OnScreenTag::Menu,
            ))
            .with_children(|parent| {
                //title
                parent.spawn((
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
                    OnScreenTag::Menu,
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
                            for generation_method in BoardGenerationMethod::as_list(){
                                let mut button_entity = parent
                                    .spawn((
                                        ButtonBundle {
                                            style: button_style.clone(),
                                            background_color: menu_graphics::NORMAL_BUTTON.into(),
                                            ..default()
                                        },
                                        MenuButtonAction::ChangeGenerationMethod(generation_method)
                                    ));    
                                    button_entity.with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            generation_method.to_string(),
                                            button_text_style.clone(),
                                        ));
                                    });
                                    if generation_method == BoardGenerationMethod::default() {
                                        button_entity.insert(SelectedOptionTag);
                                    }
                            }
                        });
                });
            });
    }
}

fn spawn_size_options(
    mut button_event_reader: EventReader<ui_event::SpawnButtons>,
    mut commands: Commands
){
    for button_event in button_event_reader.read(){
        let button_style=&button_event.button_style;
        let button_text_style=&button_event.button_text_style;

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                OnScreenTag::Menu,
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
                        for board_size in BoardSize::as_list(){
                            let mut button_entity = parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: menu_graphics::NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    MenuButtonAction::ChangeSize(board_size)
                                ));    
                                button_entity.with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        board_size.to_string(),
                                        button_text_style.clone(),
                                    ));
                                });
                                if board_size == BoardSize::default() {
                                    button_entity.insert(SelectedOptionTag);
                                }
                        }
                });
            });
    }
}

fn spawn_tile_counter(
    mut button_event_reader: EventReader<ui_event::SpawnTileCountButtons>,
    mut commands: Commands
){
    for button_event in button_event_reader.read(){
        let regular_button_style= &button_event.regular_button_style;
        let thin_button_style = &button_event.thin_button_style;
        let button_text_style = &button_event.button_text_style;

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                OnScreenTag::Menu,
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
                        //title - empty
                        parent.spawn(TextBundle::from_section(
                            String::from("Empty Tiles"),
                            button_text_style.clone(),
                        ));
                        //buttons - empty
                        for (action, text) in [
                            (MenuButtonAction::ChangeEmptyTilesCount(1), "1"),
                            (MenuButtonAction::ChangeEmptyTilesCount(2), "2"),
                        ]{
                            let mut empty_tiles_count=0;
                            if let MenuButtonAction::ChangeEmptyTilesCount(number) = action {
                                empty_tiles_count=number;
                            }
                            let mut button_entity = parent
                                .spawn((
                                    ButtonBundle {
                                        style: thin_button_style.clone(),
                                        background_color: menu_graphics::NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    action
                                ));    
                                button_entity.with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        text,
                                        button_text_style.clone(),
                                    ));
                                });
                                if empty_tiles_count == board_properties::DEFAULT_EMPTY_COUNT {
                                    button_entity.insert(SelectedOptionTag);
                                }
                        }

                        //title - walls
                        parent.spawn(TextBundle::from_section(
                            String::from("Wall Tiles"),
                            button_text_style.clone(),
                        ));
                        //buttons - walls
                        parent.spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for (action, text) in [
                                (Some(MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Decrease)), "<"),
                                (None, " _ "),
                                (Some(MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Increase)), ">"),
                            ]{
                                if action.is_none() {
                                    parent.spawn((TextBundle::from_section(
                                            text,
                                            button_text_style.clone(),
                                        ),
                                        WallCountTextTag
                                    ));  
                                }else{
                                    let mut arrow_button_entity = parent
                                    .spawn((
                                        ButtonBundle {
                                            style: thin_button_style.clone(),
                                            background_color: menu_graphics::NORMAL_BUTTON.into(),
                                            ..default()
                                        },
                                        action.unwrap()
                                    ));    
                                    arrow_button_entity.with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            text,
                                            button_text_style.clone(),
                                        ));
                                    });
                                }
                            }
                        });
                        //apply button
                        let mut apply_button_entity = 
                            parent.spawn((
                                ButtonBundle {
                                    style: regular_button_style.clone(),
                                    background_color: menu_graphics::NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Apply),                                
                                SelectedOptionTag,
                                ApplyButtonTag
                            ));    
                        apply_button_entity.with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Apply",
                                button_text_style.clone(),
                            ));
                        });
                });
            });
    }
}