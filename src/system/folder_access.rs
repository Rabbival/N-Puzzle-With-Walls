use crate::prelude::*;
use std::fs;

pub fn create_folder_if_none_exists_yet(
    folder_to_put_file_in: FolderToAccess
){
    if fs::create_dir(folder_to_put_file_in.to_string()).is_ok(){
        print_system_log(SystemLog::FolderCreated(folder_to_put_file_in.to_string()));
    }
}