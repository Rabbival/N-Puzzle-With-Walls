#[derive(Debug, Clone, Copy)]
pub enum FolderToAccess {
    SavedLayouts
}

impl FolderToAccess{
    pub fn to_string(&self) -> String{
        match self{
            FolderToAccess::SavedLayouts => {
                String::from("saved_layouts")
            }
        }
    }
}