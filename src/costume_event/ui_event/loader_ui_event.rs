use crate::prelude::*;

#[derive(Event)]
pub struct LoaderScreenActionInitiated {
    pub action: LoaderScreenAction,
}

pub struct LoaderUiEventPlugin;

impl Plugin for LoaderUiEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoaderScreenActionInitiated>();
    }
}
