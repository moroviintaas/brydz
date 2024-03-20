use std::fmt::{Display, Formatter};
use std::ops::Sub;
use enum_map::Enum;
//pub use super::role_map::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Enum)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlayRole {
    Whist,
    Declarer,
    Offside,
    Dummy
}

impl PlayRole{

    pub fn index_i(&self) -> i8{
        match self{
            PlayRole::Whist => 1,
            PlayRole::Declarer => 0,
            PlayRole::Offside => 3,
            PlayRole::Dummy => 2,
        }
    }
    pub fn index(&self) -> u8{
        match self{
            PlayRole::Whist => 1,
            PlayRole::Declarer => 0,
            PlayRole::Offside => 3,
            PlayRole::Dummy => 2,
        }
    }

    pub fn next_i(&self, i: u8) -> Self{
        match i{
            0 => *self,
            1 => match self{
                Self::Whist => Self::Dummy,
                Self::Dummy => Self::Offside,
                Self::Offside => Self::Declarer,
                Self::Declarer => Self::Whist,
            },
            2 => match self{
                Self::Whist => Self::Offside,
                Self::Dummy => Self::Declarer,
                Self::Offside => Self::Whist,
                Self::Declarer => Self::Dummy,
            },
            3 => match self{
                Self::Whist => Self::Declarer,
                Self::Dummy => Self::Whist,
                Self::Offside => Self::Dummy,
                Self::Declarer => Self::Offside,
            },
            _ => panic!("Should not happen {self:?}, {i:}")
        }
    }
}

impl Display for PlayRole{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            PlayRole::Whist => "Whist",
            PlayRole::Declarer => "Declarer",
            PlayRole::Offside => "Offside",
            PlayRole::Dummy => "Dummy",
        })
    }
}

impl Sub for PlayRole{
    type Output = PlayRole;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut i = rhs.index_i() - self.index_i();
        if i<0 {
            i += 4;
        }
        match i{
            0 => Self::Declarer,
            1 => Self::Whist,
            2 => Self::Dummy,
            3 => Self::Offside,
            _ => panic!("Should not happen, this is bug")
        }
    }
}