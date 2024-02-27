use crate::prelude::*;

#[derive(Debug)]
pub enum SystemLog{
    FolderCreated,
    FolderExists,
}

pub fn print_system_log(system_log: SystemLog){
    info!("{:?}", system_log);
}