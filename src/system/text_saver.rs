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
    let file_path = PathBuf::from(&folder_to_put_file_in.to_string())
        .join(format!("{}.txt", file_name));
    fs::write(&file_path, file_content)?;
    Ok(())
}