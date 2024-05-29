use crate::prelude::*;

const GAME_SESSION_LOG_FILE_NAME: &str = "latest_n_puzzle_session_log";

pub struct GameSessionLogPlugin;

impl Plugin for GameSessionLogPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_new_log_file);
    }
}

fn create_new_log_file(){
    if create_file(FolderToAccess::GameLogs, String::from(GAME_SESSION_LOG_FILE_NAME)).is_err(){
        print_system_access_error(SystemAccessError::BadFolderPath(FolderToAccess::GameLogs));
    }
}

pub fn append_to_game_session_log_file(string_to_append: String){
    let string_to_append_with_newline = string_to_append + "\n";
    if append_to_file(
        FolderToAccess::GameLogs,
        String::from(GAME_SESSION_LOG_FILE_NAME),
        string_to_append_with_newline
    ).is_err(){
        print_system_access_error(
            SystemAccessError::CouldntFindFile(SystemFileName::from_name(
                String::from(GAME_SESSION_LOG_FILE_NAME),
                SystemFileType::TextFile
            ))
        );
    }
}