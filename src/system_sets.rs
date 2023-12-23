use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CostumeSystemSets{
    InputListening,
    InputHandling,
    ChangesBasedOnInput,
    PostChangesBasedOnInput
}

pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update,(
                CostumeSystemSets::InputListening,
                CostumeSystemSets::InputHandling,
                CostumeSystemSets::ChangesBasedOnInput,
                CostumeSystemSets::PostChangesBasedOnInput
                ).chain())
            ;
    }
}