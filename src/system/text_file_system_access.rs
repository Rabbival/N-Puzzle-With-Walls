use crate::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::io::Write;

pub fn create_file(
    folder_to_create_in: FolderToAccess,
    file_name: String
) -> std::io::Result<()>
{
    create_folder_if_none_exists_yet(folder_to_create_in);
    let full_file_name = SystemFileName::from_name(file_name, SystemFileType::TextFile);
    let file_path = PathBuf::from(&folder_to_create_in.to_string())
        .join(full_file_name.name_with_postfix.clone());
    File::create(file_path)?;
    print_system_log(SystemLog::FileCreated(full_file_name.name_with_postfix));
    Ok(())
}

pub fn write_to_file(
    folder_to_put_file_in: FolderToAccess,
    file_name: String,
    file_content: String
) -> std::io::Result<()>
{
    create_folder_if_none_exists_yet(folder_to_put_file_in);
    let full_file_name = SystemFileName::from_name(file_name, SystemFileType::TextFile);
    let file_path = PathBuf::from(&folder_to_put_file_in.to_string())
        .join(full_file_name.name_with_postfix.clone());
    fs::write(file_path, file_content)?;
    print_system_log(SystemLog::WroteToFile(full_file_name.name_with_postfix));
    Ok(())
}

pub fn append_to_file(
    folder_where_the_file_is: FolderToAccess,
    file_name: String,
    string_to_append: String
) -> std::io::Result<()>
{
    create_folder_if_none_exists_yet(folder_where_the_file_is);
    let full_file_name = SystemFileName::from_name(file_name, SystemFileType::TextFile);
    let file_path = PathBuf::from(&folder_where_the_file_is.to_string())
        .join(full_file_name.name_with_postfix.clone());
    let mut data_file = OpenOptions::new().append(true).open(file_path)?;
    data_file.write_all(string_to_append.as_bytes())?;
    print_system_log(SystemLog::AppendedToFile(full_file_name.name_with_postfix));
    Ok(())
}

pub fn delete_text_file(
    folder_to_delete_from: FolderToAccess,
    file_name: String,
) -> std::io::Result<()>
{
    let full_file_name = SystemFileName::from_name(file_name, SystemFileType::TextFile);
    let file_path = PathBuf::from(&folder_to_delete_from.to_string())
        .join(full_file_name.name_with_postfix.clone());
    fs::remove_file(file_path)?;
    print_system_log(SystemLog::FileDeleted(full_file_name.name_with_postfix));
    Ok(())
}

pub fn get_all_valid_text_file_names_in_folder(
    folder_to_put_file_in: FolderToAccess,
) -> impl Iterator<Item = SystemFileName>
{
    let saved_layouts_directory_iterator
        = fs::read_dir(folder_to_put_file_in.to_string()).unwrap();
    
    saved_layouts_directory_iterator
        .filter(|file_result|{ file_result.is_ok() })
        .map(|valid_file|{
            valid_file.unwrap().file_name()
        })
        .filter_map(|os_string|{
            let system_file_name = SystemFileName::try_from_os_string(os_string)?;
            if system_file_name.file_type == SystemFileType::TextFile{
                Some(system_file_name)
            }else{
                None
            }
        })
}