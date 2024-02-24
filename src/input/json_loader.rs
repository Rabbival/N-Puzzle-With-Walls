use std::fs;
use std::path::PathBuf;
use crate::output::error_handler;
use crate::prelude::*;

// pub fn read_from_file(folder_to_load_from: FolderToAccess, file_to_load: String)
//     -> Result<JsonValue, error_handler::CustomJsonError>
// {
//     let file_path = PathBuf::from(&folder_to_load_from.to_string())
//         .join(format!("{}.txt", file_to_load));
//     match fs::read_to_string(&file_path){
//         Err(_) => return Err(error_handler::CustomJsonError::CouldntParseJson),
//         Ok(file_content_as_string) => {
//             let json_parse_result = json::parse(&file_content_as_string);
//             match json_parse_result{
//                 Ok(parsed_json) => Ok(parsed_json),
//                 Err(_) => Err(error_handler::CustomJsonError::CouldntParseJson),
//             }
//         }
//     }
// }