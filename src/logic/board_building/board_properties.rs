use crate::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BoardProperties{
    pub size: BoardSize,
    pub wall_count: u8,
    pub empty_count: u8,
    pub generation_method: BoardGenerationMethod,
}

/// intended to keep track of the numbers not yet applied
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UnappliedMenuWallCount(pub u8);


pub struct BoardPropertiesPlugin;

impl Plugin for BoardPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BoardProperties>()
            .init_resource::<UnappliedMenuWallCount>()
            .add_systems(Update, set_menu_wall_count_to_current)
            ;
    }
}

/// resets the number in the menu to the current (previously chosen) number
fn set_menu_wall_count_to_current(
    mut event_listener: EventReader<SetMenuElementsToFitCurrent>,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
    board_prop_res: Res<BoardProperties>,
){
    for _event in event_listener.read(){
        unapplied_menu_wall_count.0=board_prop_res.wall_count;
    }
}