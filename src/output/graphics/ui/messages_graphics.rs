use crate::prelude::*;

#[derive(Component)]
pub struct VictoryAnnouncementTag;


pub struct MessagesGraphicsPlugin;

impl Plugin for MessagesGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(Startup, spawn_victory_message)
			.add_systems(
            Update,
			toggle_victory_message_visibilities
						.in_set(InputSystemSets::PostMainChanges)
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
			   Visibility::Hidden
		   ),
			CustomOnScreenTag::Game,
			VictoryAnnouncementTag,
			OnOwnScreenVisibility(Visibility::Hidden),
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
								background_color: super::NORMAL_BUTTON_COLOR.into(),
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
	mut victory_listener: EventReader<ToggleVictoryMessage>,
	mut victory_message_query: Query<
		(&mut Visibility, &mut OnOwnScreenVisibility),
		With<VictoryAnnouncementTag>,
	>,
){
	for _victory_announcment in victory_listener.read(){
		let (mut victory_message_vis, mut victory_message_on_own_screen_vis) =
			victory_message_query.single_mut();
		if *victory_message_vis == Visibility::Visible {
			*victory_message_vis = Visibility::Hidden;
			victory_message_on_own_screen_vis.0 = Visibility::Hidden;
		} else {
			*victory_message_vis = Visibility::Visible;
			victory_message_on_own_screen_vis.0 = Visibility::Visible;
		}
	}
}
