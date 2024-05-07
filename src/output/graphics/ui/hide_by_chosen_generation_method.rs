use crate::prelude::*;

#[derive(Component)]
pub struct HideByChosenGenerationMethod {
    pub generation_methods_when_should_hide: Vec<BoardGenerationMethod>,
    pub visibility_otherwise: Option<Visibility>
}

impl HideByChosenGenerationMethod{
    pub fn new(generation_methods_when_should_hide: Vec<BoardGenerationMethod>) -> Self{
        Self{
            generation_methods_when_should_hide,
            visibility_otherwise: None
        }
    }
}