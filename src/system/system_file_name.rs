use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use crate::prelude::SystemFileType;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct SystemFileName{
    pub name_with_postfix: String,
    pub file_type: SystemFileType
}

impl SystemFileName{
    pub fn from_name(name: String, requested_postfix_type: SystemFileType) -> SystemFileName{
        let name_with_postfix = name + &requested_postfix_type.to_postfix();
        SystemFileName { 
            name_with_postfix,
            file_type: requested_postfix_type
        }
    }
    
    pub fn try_from_os_string(os_string: OsString) -> Option<SystemFileName>{
        if let Ok(parsed_string) = os_string.into_string(){
            let postfix = String::from(&parsed_string[(parsed_string.len()-4)..]);
            for system_file_type in SystemFileType::collect_all(){
                if postfix == system_file_type.to_postfix(){
                    return Some(SystemFileName{
                        name_with_postfix: parsed_string,
                        file_type: system_file_type
                    });
                }
            }
        }
        None
    }
    
    pub fn to_name(&self) -> String{
        let inner_string = &self.name_with_postfix;
        String::from(&inner_string[..(inner_string.len()-4)])
    }
}

impl Display for SystemFileName{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.name_with_postfix)?;
        Ok(())
    }
}