use crate::prelude::*;
use enum_iterator::{all, Sequence};
use std::fmt;

#[derive(Resource, Sequence, Debug)]
pub enum BoardSize{
    Tiny,
    Small,
    Medium,
    Large,
    Giant
}

impl BoardSize{

    pub fn as_strings() -> Vec<String>{
        vec![String::from("quack")]

        // all::<BoardSize>()
        //     .map(|board_size|{
        //         board_size.to_string()
        //     })
        //     .collect::<Vec<String>>()
    }

    pub fn to_grid_side_length(&self) -> u8 {
        match *self{
            BoardSize::Tiny=> 3,
            BoardSize::Small=> 4,
            BoardSize::Medium=> 5,
            BoardSize::Large=> 6,
            BoardSize::Giant=> 10,
        }
    }

    pub fn to_random_turns_range(&self) -> (u8, u8) {
        match *self{
            BoardSize::Tiny=> (10, 20),
            BoardSize::Small=> (35, 50),
            BoardSize::Medium=> (70, 100),
            BoardSize::Large=> (120, 180),
            BoardSize::Giant=> (200, 255),
        }
    }
}


impl fmt::Display for BoardSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self)
    }
}