use crate::prelude::*;

pub struct VictoryMessageLogicPlugin;

impl Plugin for VictoryMessageLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                toggle_victory_message_visibilities
                    .in_set(InputSystemSets::PostMainChanges)
            );
    }
}


/// toggles both actual visibility and on_own_screen one
fn toggle_victory_message_visibilities(
    mut victory_listener: EventReader<ToggleVictoryMessage>,
    mut victory_message_query: Query<
        (&mut Visibility, &mut CustomOnScreenTag),
        With<VictoryAnnouncementTag>,
    >,
){
    for _victory_announcement in victory_listener.read(){
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
}