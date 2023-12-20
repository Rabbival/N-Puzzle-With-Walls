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
            BoardSize::Small=> (35, 50),
            BoardSize::Medium=> (70, 100),
            BoardSize::Large=> (120, 180),
            BoardSize::Giant=> (200, 255),
        }
    }
}