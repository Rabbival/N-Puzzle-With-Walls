use crate::prelude::*;

#[derive(Debug)]
pub struct FileName(pub String);

#[derive(Debug)]
pub enum SystemAccessError {
    RequestedFileDoesntExist(FileName),
    CouldntParseFile(FileName),
    MismatchingPostfix(MismatchError)
}

pub fn print_system_access_error(system_access_error: SystemAccessError) {
    match system_access_error {
        SystemAccessError::RequestedFileDoesntExist(file_name) => {
            error!("couldn't find file {}",file_name.0);
        },
        SystemAccessError::CouldntParseFile(file_name) => {
            error!("couldn't parse {}",file_name.0);
        },
        SystemAccessError::MismatchingPostfix(mismatch_error) => {
            error!
            (
                "expected {} but found {}",
                mismatch_error.expected,
                mismatch_error.found
            );
        }
    }
}