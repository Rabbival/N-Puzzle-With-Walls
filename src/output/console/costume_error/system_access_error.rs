use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum SystemAccessError {
    CouldntFindFile(SystemFileName),
    CouldntParseFile(SystemFileName),
    MismatchingPostfix(MismatchError),
}

pub fn print_system_access_error(system_access_error: SystemAccessError) {
    match system_access_error {
        SystemAccessError::CouldntFindFile(system_file_name) => {
            error!("couldn't find file {}", system_file_name.name_with_postfix);
        },
        SystemAccessError::CouldntParseFile(system_file_name) => {
            error!("couldn't parse {}", system_file_name.name_with_postfix);
        },
        SystemAccessError::MismatchingPostfix(mismatch_error) => {
            error!
            (
                "expected {} but found {}",
                mismatch_error.expected,
                mismatch_error.found
            );
        },
    }
}