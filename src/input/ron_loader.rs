use std::fs;
use std::path::PathBuf;
use crate::output::error_handler;
use crate::prelude::*;

pub fn domain_board_from_file(folder_to_load_from: FolderToAccess, file_to_load: String)
    -> Result<DomainBoard, error_handler::CustomRonError>
{
    let file_path = PathBuf::from(&folder_to_load_from.to_string())
        .join(format!("{}.txt", file_to_load));
    match fs::read_to_string(&file_path){
        Err(_) => return Err(error_handler::CustomRonError::CouldntParseRon),
        Ok(file_content_as_string) => {
            let ron_parse_result = ron::from_str(&file_content_as_string);
            match ron_parse_result{
                Ok(parsed_ron) => Ok(parsed_ron),
                Err(_) => Err(error_handler::CustomRonError::CouldntParseRon),
            }
        }
    }
}