use crate::prelude::*;

#[derive(Component, Default, PartialEq, Eq, Debug)]
pub struct MultipleOnScreenTags(pub Vec<CustomOnScreenTag>);