pub mod game_log;
pub mod system_log;
pub mod solution_printer;


use std::fmt::Display;

use crate::prelude::*;

pub enum BevyPrintType {
    Info,
    Warn,
    Error,
}

pub fn print_display_deriver_vec<T: Display>(to_print: &Vec<T>, print_type: BevyPrintType) {
    let mut to_print_str = String::from("[");
    for item in to_print {
        to_print_str += &(String::from(" ") + &item.to_string());
    }
    to_print_str += " ]";
    match print_type {
        BevyPrintType::Info => {
            info!("{}", to_print_str)
        }
        BevyPrintType::Warn => {
            warn!("{}", to_print_str)
        }
        BevyPrintType::Error => {
            error!("{}", to_print_str)
        }
    }
}