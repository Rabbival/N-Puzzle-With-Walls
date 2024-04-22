use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum FolderToAccess {
    SavedLayouts
}

impl Display for FolderToAccess{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(
            match self{
                FolderToAccess::SavedLayouts => {
                    "saved_layouts"
                }
            })?;
        Ok(())
    }
}