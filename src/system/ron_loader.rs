use std::fs;
use std::path::PathBuf;
use crate::prelude::*;

pub fn domain_board_from_file(folder_to_load_from: FolderToAccess, file_to_load: SystemFileName)
    -> Result<DomainBoard, SystemAccessError>
{
    let file_path = PathBuf::from(&folder_to_load_from.to_string())
        .join(file_to_load.name_with_postfix.clone());
    match fs::read_to_string(file_path){
        Err(_) => Err(SystemAccessError::CouldntFindFile(file_to_load.clone())),
        Ok(file_content_as_string) => {
            let ron_parse_result = ron::from_str(&file_content_as_string);
            match ron_parse_result{
                Ok(parsed_ron) => Ok(parsed_ron),
                Err(_) => Err(SystemAccessError::CouldntParseFile(file_to_load.clone())),
            }
        }
    }
}