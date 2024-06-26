use crate::prelude::{DomainBoardName, Entity, SavedLayoutIndexInDifficultyVec};

pub struct LoaderChosenLayoutConfig{
    pub chosen_layout_button_text: String,
    pub layout_name: DomainBoardName,
    pub optional_layout_entity: Option<Entity>,
    pub optional_index: Option<SavedLayoutIndexInDifficultyVec>,
    pub optional_page_number: Option<usize>
}

impl Default for LoaderChosenLayoutConfig{
    fn default() -> Self {
        Self{
            chosen_layout_button_text: String::from("no chosen board"),
            layout_name: DomainBoardName(String::new()),
            optional_layout_entity: None,
            optional_index: None,
            optional_page_number: None
        }
    }
}