use enum_iterator::{all, Sequence};
use std::fmt;

#[derive(Sequence, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BoardSize{
    Tiny,
    #[default]
    Small,
    Medium,
    Large,
    Giant
}

impl BoardSize{
    pub fn as_list() -> Vec<BoardSize>{
        all::<BoardSize>().collect::<Vec<BoardSize>>()
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

    pub fn wall_count_upper_bound(&self) -> u8 {
        match *self{
            BoardSize::Tiny=> 1,
            BoardSize::Small=> 3,
            BoardSize::Medium=> 5,
            BoardSize::Large=> 8,
            BoardSize::Giant=> 30,
        }
    }
}

impl fmt::Display for BoardSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}