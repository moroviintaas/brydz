use std::fmt::{Debug, Display, Formatter};
use std::ops::{Sub};
use rand::{Rng, RngCore};
use karty::random::RandomSymbol;
use crate::player::axis::Axis;
use crate::player::axis::Axis::{EastWest, NorthSouth};
use crate::player::side::Side::{East, North, South, West};
pub use super::side_map::*;

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

/// ```
/// use brydz_core::player::side::{Side};
/// use karty::cards::Card;
/// assert_eq!(std::mem::size_of::<Side>(), 1)
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Side{
    East,
    South,
    West,
    North
}

pub const SIDES : [Side;4] = [North, East, South, West];
impl Side{
    pub fn axis(&self) -> Axis{
        match self{
            Self::East | Self::West=> EastWest,
            Self::North | Self::South => NorthSouth
        }
    }
    pub fn next(&self) -> Self{
        match self{
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East
        }
    }
    pub fn prev(&self) -> Self{
        match self{
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West
        }
    }
    pub fn partner(&self) -> Self{
        match self{
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South
        }
    }
    pub fn next_i(&self, index: u8) -> Self{
        match index & 0x03{
            0 => self.to_owned(),
            1 => self.next(),
            2 => match self{
                North => South,
                East => West,
                South => North,
                West => East
            },
            3 => self.prev(),
            i => {panic!("Next_i {i} shouldn't happen")}
        }

    }
    /// ```
    /// use brydz_core::player::side::Side;
    /// use brydz_core::player::side::Side::{East, North, West};
    /// assert_eq!(Side::difference(North, East), 1);
    /// assert_eq!(Side::difference(East, North), 3);
    /// assert_eq!(Side::difference(East, West), 2);
    /// ```
    pub fn difference(first: Self, second: Self) -> u8{
        if first == second{
            0
        } else if first.next_i(1) == second{
            1
        } else if first.next_i(2) == second{
            2
        } else{
            3
        }
    }

    pub fn index(&self) -> u8{
        match self{
            North => 0,
            East => 1,
            South => 2,
            West => 3
        }
    }
    pub(crate) fn first_unknown_side(self, declarer: Side) -> Side{
        match self - declarer{
            1 => self.partner(),
            _ => self.next()
        }
    }
    pub(crate) fn second_unknown_side(self, declarer: Side) -> Side{
        match self - declarer{
            0 | 1 => self.prev(),
            _ => self.partner()
        }
    }
}

impl Sub for Side{
    type Output = u8;
    /// ```
    /// use brydz_core::player::side::Side::{East, North, South};
    /// assert_eq!(North-East, 3);
    /// assert_eq!(North-South, 2);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Self::difference(rhs, self)
    }
}

impl Display for Side{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            East => "East",
            South => "South",
            West => "West",
            North => "North"
        })
    }
}
/*
impl Distribution<Side> for rand::distributions::Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Side {
        SIDES[rng.gen_range(0..4)]
    }
}*/

impl<R: RngCore> RandomSymbol<R> for Side{
    fn random(rng: &mut R) -> Self {
        match rng.random_range(0..4) {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => panic!("Unfallable")
        }
    }
}


/*
impl Display for Option<Side>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            None => "None",
            Some(s) => s.fmt(f)
        })
    }
}*/



/*
impl<'a, T> IntoIterator for SideAssociated<T>{
    type Item = Side;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.north, &self.east, &self.south, &self.west].iter()
    }
}*/

impl<T: Clone> SideMap<T>{
    pub fn clone_element(&self, side: &Side) -> T{
        self[side].clone()
    }
}



impl<T1, T2> SideMap<(T1, T2)>{
    pub fn split(self) -> (SideMap<T1>, SideMap<T2>){
        (SideMap {north: self.north.0, east: self.east.0, west: self.west.0, south: self.south.0},
         SideMap {north: self.north.1, east: self.east.1, west: self.west.1, south: self.south.1})
    }
}





impl<T:Copy> Copy for SideMap<T>{}
/*
impl<T> Default for SideAssociated<Option<T>>{
    fn default() -> Self {
        Self{east: None, south: None, west: None, north: None}
    }
}*/
impl<T:Default> Default for SideMap<T>{
    fn default() -> Self {
        Self{east: T::default(), south: T::default(), west: T::default(), north: T::default()}
    }
}

#[cfg(test)]
mod tests{
    use crate::player::side::Side::*;

    #[test]
    fn first_unknown(){
        let declarer = North;
        assert_eq!(North.first_unknown_side(declarer), East);
        assert_eq!(East.first_unknown_side(declarer), West);
        assert_eq!(South.first_unknown_side(declarer), West);
        assert_eq!(West.first_unknown_side(declarer), North);

    }

    #[test]
    fn second_unknown(){
        let declarer = North;
        assert_eq!(North.second_unknown_side(declarer), West);
        assert_eq!(East.second_unknown_side(declarer), North);
        assert_eq!(South.second_unknown_side(declarer), North);
        assert_eq!(West.second_unknown_side(declarer), East);

    }
}