/// defaultly tiny
#[derive(Default)]
pub enum BoardSize{
    #[default]
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
}