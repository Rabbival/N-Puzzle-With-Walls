use crate::prelude::AppState;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSets {
    InputListening,
    InputHandlingPreparations,
    InputHandling,
    InitialChanges,
    PostInitialChanges,
    MainChanges,
    PostMainChanges,
    LateChanges,
    LatestChanges,
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
                InputSystemSets::InputHandlingPreparations,
                InputSystemSets::InputHandling,
                InputSystemSets::InitialChanges,
                InputSystemSets::PostInitialChanges,
                InputSystemSets::MainChanges,
                InputSystemSets::PostMainChanges,
                InputSystemSets::LateChanges,
                InputSystemSets::LatestChanges,
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
        )
        .configure_sets(
            OnEnter(AppState::Loader),
            (
                StateChangeSystemSets::StateChangeListening,
                StateChangeSystemSets::PrepareToHandleStateChange,
                StateChangeSystemSets::HandleStateChange,
            )
                .chain(),
        );
    }
}
