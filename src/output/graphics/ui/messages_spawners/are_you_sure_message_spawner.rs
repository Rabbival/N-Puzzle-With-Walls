use crate::output::graphics::ui::NORMAL_BUTTON_COLOR;
use crate::prelude::*;

#[derive(Component)]
pub struct AreYouSureMessageTextTag;


pub struct AreYouSureMessageSpawnerPlugin;

impl Plugin for AreYouSureMessageSpawnerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                spawn_are_you_sure_message
            );
    }
}

fn spawn_are_you_sure_message(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut commands: Commands,
) {
    for spawn_request in spawn_event_reader.read() {
        let text_style = &spawn_request.medium_text_style;
        let button_style = &spawn_request.big_button_style;
        commands
            .spawn((
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::Center,
                    Visibility::Hidden,
                    Some(FlexDirection::Column)
                ),
                AreYouSureMessageType::DeleteAllBoards,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(85.0),
                            height: Val::Percent(50.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        z_index: ZIndex::Global(1),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(95.0),
                                    height: Val::Percent(95.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::BLACK.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((TextBundle::from_section(
                                        AreYouSureMessageType::DeleteAllBoards.to_string(),
                                        text_style.clone(),
                                    ).with_text_justify(JustifyText::Center),
                                  ButtonText,
                                  AreYouSureMessageTextTag
                                ));
                                parent.spawn(are_you_sure_message_ui_gap());
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(95.0),
                                        height: Val::Percent(30.0),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: Color::INDIGO.into(),
                                    ..default()
                                }).with_children(|parent| {
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: button_style.clone(),
                                                background_color: NORMAL_BUTTON_COLOR.into(),
                                                ..default()
                                            },
                                            AreYouSureMessageButtonAction::Confirm
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Confirm",
                                                text_style.clone(),
                                            ));
                                        });
                                    parent.spawn(are_you_sure_message_ui_gap());
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: button_style.clone(),
                                                background_color: NORMAL_BUTTON_COLOR.into(),
                                                ..default()
                                            },
                                            AreYouSureMessageButtonAction::Cancel
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Cancel",
                                                text_style.clone(),
                                            ));
                                        });
                                });
                            });
                        });
                parent.spawn(are_you_sure_message_ui_gap());
            });
    }
}

fn are_you_sure_message_ui_gap() -> NodeBundle{
    NodeBundle {
        style: Style {
            width: Val::Percent(40.0),
            height: Val::Percent(20.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }
}