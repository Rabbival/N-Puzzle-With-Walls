use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use crate::output::graphics::ui::NORMAL_BUTTON_COLOR;
use crate::prelude::*;

const LAYOUT_MARGINS_RECT: UiRect = UiRect {
    top: Val::Px(0.0),
    right: Val::Px(10.0),
    bottom: Val::Px(20.0),
    left: Val::Px(10.0)
};

#[derive(Component)]
pub struct ChosenLayoutTextTag;

#[derive(Component)]
pub struct ScreenChangeArrowTag;

#[derive(Component)]
pub struct LayoutPreviewNode(pub LoaderScreenSlot);

pub struct LoaderScreenSpawnerPlugin;

impl Plugin for LoaderScreenSpawnerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_bottom_line,
                spawn_layout_slots_to_choose_from,
                spawn_delete_all_layouts_button,
                spawn_load_screen_arrows
            )
        );
    }
}

fn spawn_bottom_line(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut commands: Commands,
){
    for spawn_request in spawn_event_reader.read() {
        let text_style = &spawn_request.medium_text_style;
        let button_style = &spawn_request.common_button_style;
        let chosen_slot_text_style = &spawn_request.small_text_style;
        let chosen_slot_button_style = &spawn_request.space_bar_looking_button_style;
        
        commands
            .spawn((
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::End,
                    Visibility::Hidden,
                    Some(FlexDirection::Column)
                ),
                CustomOnScreenTag{
                    screen: AppState::Loader,
                    on_own_screen_visibility: Some(Visibility::Visible)
                },
            )).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: INDIGO_TEXT_COLOR.into(),
                ..default()
            }).with_children(|parent| {
                //chosen
                parent
                    .spawn((
                        ButtonBundle {
                            style: chosen_slot_button_style.clone(),
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        LoaderScreenAction::JumpToChosenLayoutScreen(None, BoardDifficulty::default())
                    ))
                    .with_children(|parent| {
                        parent.spawn((TextBundle::from_section(
                            "no chosen board",
                                chosen_slot_text_style.clone(),
                            ),
                            ChosenLayoutTextTag
                         ));
                    });
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Start,
                            ..default()
                        },
                        ..default()
                    },
                )).with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            LoaderScreenAction::GenerateBoard(None),
                            CustomOnScreenTag{
                                screen: AppState::Loader,
                                on_own_screen_visibility: Some(Visibility::Hidden)
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Load",
                                text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            LoaderScreenAction::WarnBeforeDeletion(PopUpMessageType::DeleteBoard(None))
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Delete",
                                text_style.clone(),
                            ));
                        });
                });
            });
        });
    }
}

fn spawn_layout_slots_to_choose_from(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
){
    for spawn_request in spawn_event_reader.read() {
        commands
            .spawn(
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::Center,
                    Visibility::Hidden,
                    Some(FlexDirection::Column)
                )
            ).with_children(|parent| {
            //first row
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Start,
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|parent| {
                spawn_layout_entity(
                    parent,
                    image_handle_to_render_to(&mut images),
                    spawn_request,
                    LoaderScreenSlot::TopLeft,
                );
                spawn_layout_entity(
                    parent,
                    image_handle_to_render_to(&mut images),
                    spawn_request,
                    LoaderScreenSlot::TopRight,
                );
            });
            //second row
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                spawn_layout_entity(
                    parent,
                    image_handle_to_render_to(&mut images),
                    spawn_request,
                    LoaderScreenSlot::BottomLeft,
                );
                spawn_layout_entity(
                    parent,
                    image_handle_to_render_to(&mut images),
                    spawn_request,
                    LoaderScreenSlot::BottomRight,
                );
            });
            //gap
            parent.spawn(
                 NodeBundle {
                     style: Style {
                         width: Val::Percent(100.0),
                         height: Val::Percent(8.0),
                         ..default()
                     },
                     ..default()
                 });
        });
    }
}

fn spawn_layout_entity(
    parent: &mut ChildBuilder,
    image_handle: Handle<Image>,
    spawn_request: &SpawnTextsAndButtons,
    loader_screen_slot: LoaderScreenSlot,
){
    parent.spawn(NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            margin: LAYOUT_MARGINS_RECT,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        let mut layout_button_entity = parent.spawn((
            ButtonBundle {
                style: spawn_request.board_props_button_style.clone(),
                background_color: super::NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            LoaderScreenAction::ChooseLayoutInSlot(loader_screen_slot),
            CustomOnScreenTag{
                screen: AppState::Loader,
                on_own_screen_visibility: Some(Visibility::Hidden)
            }
        ));
        layout_button_entity.with_children(|parent| {
            parent.spawn((
                TextBundle::from_section("", spawn_request.tiny_text_style.clone()),
                ButtonText,
            ));
        });
        layout_button_entity.with_children(|parent|{
           parent.spawn((
                NodeBundle{
                    style: Style {
                        width: Val::Percent(85.0),
                        height: Val::Percent(85.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(image_handle),
                LayoutPreviewNode(loader_screen_slot),
            )); 
        });
        //tiny gap node
        layout_button_entity.with_children(|parent|{
            parent.spawn((
                NodeBundle{
                    style: Style {
                        width: Val::Percent(85.0),
                        height: Val::Percent(2.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
    });
}

fn image_handle_to_render_to(images: &mut Assets<Image>)-> Handle<Image> {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };
    
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    images.add(image)
}

fn spawn_delete_all_layouts_button(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut commands: Commands,
){
    for spawn_request in spawn_event_reader.read() {
        commands
            .spawn((
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::Start,
                    Visibility::Hidden,
                    Some(FlexDirection::Column)
                ),
                simple_on_screen_tag(AppState::Loader),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            background_color: INDIGO_TEXT_COLOR.into(),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            ButtonBundle {
                                style: spawn_request.space_bar_looking_button_style.clone(),
                                background_color: super::NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            LoaderScreenAction::WarnBeforeDeletion(PopUpMessageType::DeleteAllBoards)
                        ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        "Delete All Boards",
                                        spawn_request.medium_text_style.clone()
                                    ),
                                    ButtonText,
                                ));
                            });
                    });
            });
    }
}

fn spawn_load_screen_arrows(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut commands: Commands,
) {
    for spawn_request in spawn_event_reader.read() {
        spawn_load_screen_arrow(
            spawn_request,
            JustifyContent::End,
            ">",
            LoaderScreenAction::ChangeScreen(ScreenChangeRequestType::Next),
            &mut commands
        );
        spawn_load_screen_arrow(
            spawn_request,
            JustifyContent::Start,
            "<",
            LoaderScreenAction::ChangeScreen(ScreenChangeRequestType::Previous),
            &mut commands
        );
    }
}

fn spawn_load_screen_arrow(
    spawn_request: &SpawnTextsAndButtons,
    content_side: JustifyContent,
    text_value: impl Into<String>,
    loader_screen_action: LoaderScreenAction,
    commands: &mut Commands
){
    let thin_button_style = &spawn_request.thin_button_style;
    let medium_text_style = &spawn_request.medium_text_style;

    commands
        .spawn((
            build_node_bundle_with_full_percentage_style(
                AlignItems::Center,
                content_side,
                Visibility::Hidden,
                None
            ),
            CustomOnScreenTag{
                screen: AppState::Loader,
                on_own_screen_visibility: Some(Visibility::Hidden)
            },
            ScreenChangeArrowTag
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: INDIGO_TEXT_COLOR.into(),
                ..default()
            })
                .with_children(|parent| {
                    let mut arrow_button_entity = parent.spawn((
                        ButtonBundle {
                            style: thin_button_style.clone(),
                            background_color: super::NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        loader_screen_action
                    ));
                    arrow_button_entity.with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                text_value,
                                medium_text_style.clone(),
                            ),
                            ButtonText,
                        ));
                    });
                });
        });

}