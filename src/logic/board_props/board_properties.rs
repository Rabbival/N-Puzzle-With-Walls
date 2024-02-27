use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

pub const DEFAULT_EMPTY_COUNT: u8 = 1;
pub const DEFAULT_WALL_COUNT: u8 = 0;

#[derive(Component)]
pub struct AppliedBoardProperties;
#[derive(Component)]
pub struct PlannedBoardProperties;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct BoardProperties {
    pub size: BoardSize,
    pub wall_count: u8,
    pub empty_count: u8,
    pub generation_method: BoardGenerationMethod,
    /// determines the graph travelling that will be used to generate
    /// the MST used for efficient wall spawning
    pub tree_traveller_type: GridTravellerType,
}

/// intended to keep track of the numbers not yet applied
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UnappliedMenuWallCount(pub u8);

pub struct BoardPropertiesPlugin;

impl Plugin for BoardPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnappliedMenuWallCount>()
            .add_systems(PreStartup, create_current_and_planned_board_properties)
            .add_systems(
                Update,
                set_planned_props_to_fit_current
                    .in_set(StateChangeSystemSets::PrepareToHandleStateChange),
            );
    }
}

fn create_current_and_planned_board_properties(mut commands: Commands) {
    commands.spawn((BoardProperties::default(), AppliedBoardProperties));
    commands.spawn((BoardProperties::default(), PlannedBoardProperties));
}

/// sets the one that appears in the menu to fit the current configuration
fn set_planned_props_to_fit_current(
    mut event_writer: EventWriter<SetMenuElementsToFitCurrent>,
    mut event_listener: EventReader<SetPlannedPropertiesToFitCurrent>,
    mut unapplied_menu_wall_count: ResMut<UnappliedMenuWallCount>,
    applied_board_prop_query: Query<
        &BoardProperties,
        (
            With<AppliedBoardProperties>,
            Without<PlannedBoardProperties>,
        ),
    >,
    mut planned_board_prop_query: Query<
        &mut BoardProperties,
        (
            With<PlannedBoardProperties>,
            Without<AppliedBoardProperties>,
        ),
    >,
) {
    for _event in event_listener.read() {
        let current_props = applied_board_prop_query.single();
        let mut planned_props = planned_board_prop_query.single_mut();
        unapplied_menu_wall_count.0 = current_props.wall_count;
        *planned_props = *current_props;
        event_writer.send(SetMenuElementsToFitCurrent);
    }
}

impl Default for BoardProperties {
    fn default() -> Self {
        Self {
            size: BoardSize::default(),
            wall_count: DEFAULT_WALL_COUNT,
            empty_count: DEFAULT_EMPTY_COUNT,
            generation_method: BoardGenerationMethod::default(),
            tree_traveller_type: GridTravellerType::default(),
        }
    }
}

impl fmt::Display for BoardProperties{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("BoardProperties(size: ")?;
        fmt.write_str(&self.size.to_string())?;
        fmt.write_str(", wall_count: ")?;
        fmt.write_str(&self.wall_count.to_string())?;
        fmt.write_str(", empty_count: ")?;
        fmt.write_str(&self.empty_count.to_string())?;
        fmt.write_str(", generation_method: ")?;
        fmt.write_str(&self.generation_method.to_string())?;
        fmt.write_str(", tree_traveller_type: ")?;
        fmt.write_str(&self.tree_traveller_type.to_string())?;
        fmt.write_str(")")?;
        Ok(())
    }
}