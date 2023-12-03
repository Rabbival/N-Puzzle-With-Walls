use enum_iterator::{all, Sequence};

pub const BASIC_DIRECTION_COUNT:u8=4;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd)]
pub enum BasicDirection{
    Up,
    Right,
    Down,
    Left
}

impl BasicDirection {
    pub fn dir_to_index(&self) -> u8 {
        *self as u8
    }

    pub fn opposite_direction(&self) -> Self{
        Self::index_to_dir(self.opposite_direction_index())
    }
    
    pub fn opposite_direction_index(&self) -> u8{
        let mut index=self.dir_to_index();
        (index+2)%BASIC_DIRECTION_COUNT
    }
}

//static functions
impl BasicDirection{
    pub fn index_to_dir(index: u8) -> Self{
        match index{
            0=>BasicDirection::Up,
            1=>BasicDirection::Right,
            2=>BasicDirection::Down,
            3=>BasicDirection::Left
        }
    }

    pub fn get_directions_as_vec() -> Vec<BasicDirection>{
        all::<BasicDirection>().collect::<Vec<_>>()
    }
}