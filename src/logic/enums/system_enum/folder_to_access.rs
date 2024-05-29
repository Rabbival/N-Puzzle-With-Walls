use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum FolderToAccess {
    SavedLayouts,
    GameLogs
}

impl Display for FolderToAccess{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(
            match self{
                Self::SavedLayouts => {
                    "saved_layouts"
                },
                Self::GameLogs => {
                    "game_logs"
                },
            })?;
        Ok(())
    }
}