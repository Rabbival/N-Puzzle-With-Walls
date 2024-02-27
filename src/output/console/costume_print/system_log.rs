use crate::prelude::*;

#[derive(Debug)]
pub enum SystemLog{
    FolderCreated(String),
    FolderExists(String),
}

pub fn print_system_log(system_log: SystemLog){
    info!("{:?}", system_log);
}