use crate::prelude::*;

/// allows optionally visible components to save their original visibility
#[derive(Component, Default)]
pub struct OnOwnScreenVisibility(pub Visibility);