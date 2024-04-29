use crate::output::graphics::ui::NORMAL_BUTTON_COLOR;
use crate::prelude::*;

#[derive(Component)]
pub struct VictoryAnnouncementTag;

pub struct VictoryMessagePlugin;

impl Plugin for VictoryMessagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_victory_message)
            .add_systems(
                OnEnter(GameState::Victory),
                toggle_victory_message_visibilities,
            )
            .add_systems(
                OnExit(GameState::Victory),
                toggle_victory_message_visibilities,
            );
    }
}

fn spawn_victory_message(
    mut commands: Commands
){
    let button_style = Style {
        width: Val::Px(600.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 50.0,
        ..default()
    };

    commands
        .spawn((
            build_node_bundle_with_full_percentage_style(
                AlignItems::End,
                JustifyContent::Center,
                Visibility::Hidden,
                None
            ),
            CustomOnScreenTag{
                screen: AppState::Game,
                on_own_screen_visibility: Some(Visibility::Hidden)
            },
            VictoryAnnouncementTag,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            VictoryButtonAction::ResetBoard,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Puzzle Solved!  (Reset)",
                                    button_text_style.clone()
                                ),
                                ButtonText,
                            ));
                        });
                });
        });
}

/// toggles both actual visibility and on_own_screen one
fn toggle_victory_message_visibilities(
    mut victory_message_query: Query<
        (&mut Visibility, &mut CustomOnScreenTag),
        With<VictoryAnnouncementTag>,
    >,
){
    let (mut victory_message_vis, mut custom_on_screen_tag) =
        victory_message_query.single_mut();
    if let Some(own_screen_vis_for_toggle) = &mut custom_on_screen_tag.on_own_screen_visibility{
        if *victory_message_vis == Visibility::Visible {
            *victory_message_vis = Visibility::Hidden;
            *own_screen_vis_for_toggle = Visibility::Hidden;
        } else {
            *victory_message_vis = Visibility::Visible;
            *own_screen_vis_for_toggle = Visibility::Visible;
        }
    }
}