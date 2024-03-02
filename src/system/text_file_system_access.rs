use std::fs;
use crate::prelude::*;
use std::path::PathBuf;

pub fn write_to_file(
    folder_to_put_file_in: FolderToAccess,
    file_name: String,
    file_content: String
) -> std::io::Result<()>
{
    create_folder_if_none_exists_yet(folder_to_put_file_in);
    let full_file_name = format!("{}.txt", file_name);
    let file_path = PathBuf::from(&folder_to_put_file_in.to_string())
        .join(full_file_name.clone());
    fs::write(file_path, file_content)?;
    print_system_log(SystemLog::WroteToFile(full_file_name));
    Ok(())
}

pub fn delete_text_file(
    folder_to_put_file_in: FolderToAccess,
    file_name: String,
) -> std::io::Result<()>
{
    let full_file_name = format!("{}.txt", file_name);
    let file_path = PathBuf::from(&folder_to_put_file_in.to_string())
        .join(full_file_name.clone());
    fs::remove_file(file_path)?;
    print_system_log(SystemLog::FileDeleted(full_file_name));
    Ok(())
}

pub fn get_all_valid_text_file_names_in_folder(
    folder_to_put_file_in: FolderToAccess,
) -> impl Iterator<Item = String>
{
    let saved_layouts_directory_iterator
        = fs::read_dir(folder_to_put_file_in.to_string()).unwrap();
    
    saved_layouts_directory_iterator
        .filter(|file_result|{ file_result.is_ok() })
        .map(|valid_file|{
            valid_file.unwrap().file_name().into_string().unwrap()
        })
        .filter(|file_name|{
            let file_name_postfix =
                &file_name[(file_name.len()-4)..file_name.len()];
            file_name_postfix == ".txt"
        })
}