use crate::output::graphics::ui::NORMAL_BUTTON_COLOR;
use crate::prelude::*;

#[derive(Component)]
pub struct AreYouSureMessageTag;

pub struct AreYouSureMessagePlugin;

impl Plugin for AreYouSureMessagePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                spawn_are_you_sure_message
            )
            .add_systems(
                Update,
                listen_for_are_you_sure_message_requests
            );
    }
}

fn spawn_are_you_sure_message(
    mut spawn_event_reader: EventReader<SpawnBigButtons>,
    mut commands: Commands,
) {
    for spawn_request in spawn_event_reader.read() {
        let text_style = TextStyle {
            font_size: 42.0,
            ..default()
        };
        let button_style = &spawn_request.big_button_style;
        commands
            .spawn((
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::Center,
                    Visibility::Hidden,
                    Some(FlexDirection::Column)
                ),
                AreYouSureMessageTag,
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
                                    ).with_text_alignment(TextAlignment::Center),
                                  ButtonText,
                                  SaveWallsLayoutTextTag
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
                                            //TODO: put button action here
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
                                            //TODO: put button action here
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

fn listen_for_are_you_sure_message_requests(

){

}