use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSets {
    InputListening,
    InputHandling,
    ChangesBasedOnInput,
    PostMainChanges,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StateChangeSystemSets {
    StateChangeListening,
    PrepareToHandleStateChange,
    HandleStateChange,
}

pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InputSystemSets::InputListening,
                InputSystemSets::InputHandling,
                InputSystemSets::ChangesBasedOnInput,
                InputSystemSets::PostMainChanges,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                StateChangeSystemSets::StateChangeListening,
                StateChangeSystemSets::PrepareToHandleStateChange,
                StateChangeSystemSets::HandleStateChange,
            )
                .chain(),
        );
    }
}
