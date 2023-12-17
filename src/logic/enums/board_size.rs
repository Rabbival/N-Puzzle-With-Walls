use crate::prelude::*;

#[derive(Resource)]
pub enum BoardSize{
    Tiny,
    Small,
    Medium,
    Large,
    Giant
}

impl BoardSize{
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
            BoardSize::Small=> (30, 40),
            BoardSize::Medium=> (60, 80),
            BoardSize::Large=> (100, 120),
            BoardSize::Giant=> (180, 200),
        }
    }
}