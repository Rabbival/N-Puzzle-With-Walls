use crate::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UnappliedMenuWallCount(pub u8);

pub struct UnappliedMenuWallCountPlugin;

impl Plugin for UnappliedMenuWallCountPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<UnappliedMenuWallCount>();
    }
}