use std::cmp::Ordering;
use std::ops::{Mul, MulAssign};
use karty::cards::Card;
use crate::error::{FuzzyCardSetErrorGen};
use crate::error::FuzzyCardSetErrorGen::{BadProbability, ProbabilityBelowZero, ProbabilityOverOne};

#[derive(Clone, Copy, Debug, Default)]
pub enum FProbability{
    One,
    #[default]
    Zero,
    Uncertain(f32),
    Bad(f32)
}

impl FProbability{
    pub fn is_uncertain(&self) -> bool{
        matches!(self, Self::Uncertain(_))
    }
    pub fn is_zero(&self) -> bool{
        match self{
            FProbability::One => false,
            FProbability::Zero => true,
            FProbability::Uncertain(a) => a == &0.0,
            FProbability::Bad(_) => false
        }
    }
    pub fn is_one(&self) -> bool{
        match self{
            FProbability::One => true,
            FProbability::Zero => false,
            FProbability::Uncertain(a) => a == &1.0,
            FProbability::Bad(_) => false
        }
    }
}

impl Mul<f32> for FProbability{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        match self{
            FProbability::One => {
                if rhs > 1.0{
                    FProbability::Bad(rhs)
                } else if rhs == 0.0 {
                    FProbability::Zero
                } else if rhs < 0.0{
                    FProbability::Bad(rhs)
                } else {
                    FProbability::Uncertain(rhs)
                }
            }
            FProbability::Zero => FProbability::Zero,
            FProbability::Uncertain(p) => {
                let new_p = p*rhs;
                if !(0.0..=1.0).contains(&new_p){
                    FProbability::Bad(new_p)
                }  else {
                    FProbability::Uncertain(new_p)
                }
            }
            FProbability::Bad(p) => FProbability::Bad(p*rhs)
        }
    }
}

impl MulAssign<f32> for FProbability{
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}


impl PartialEq<Self> for FProbability {
    fn eq(&self, other: &Self) -> bool {
        let left_asf32 = match self{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => *f,
            FProbability::Bad(b) => *b
        };
        let right_asf32 = match other{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => *f,
            FProbability::Bad(b) => *b
        };
        left_asf32.eq(&right_asf32)
    }
}

impl PartialOrd for FProbability{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left_asf32 = match self{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => *f,
            FProbability::Bad(b) => *b
        };
        let right_asf32 = match other{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => *f,
            FProbability::Bad(b) => *b
        };

        left_asf32.partial_cmp(&right_asf32)
    }
}
/*
impl Into<f32> for FProbability{
    fn into(self) -> f32 {
        match self{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => f,
            FProbability::Bad(b) => b
        }
    }
}
*/
impl From<FProbability> for f32{
    fn from(value: FProbability) -> Self {
         match value{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => f,
            FProbability::Bad(b) => panic!("Bad number as probability: {b}")
        }
    }
}

impl From<FProbability> for f64{
    fn from(value: FProbability) -> Self {
         match value{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => f.into(),
            FProbability::Bad(b) => panic!("Bad number as probability: {b}")
        }
    }
}



impl TryFrom<f32> for FProbability{
    type Error = FuzzyCardSetErrorGen<Card>;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value > 0.0 && value < 1.0{
            Ok(FProbability::Uncertain(value))
        } else if value == 0.0{
            Ok(FProbability::Zero)
        } else if value == 1.0{
            Ok(FProbability::One)
        } else if value < 0.0 {
            Err(ProbabilityBelowZero(value))
        } else if value > 1.0{
            Err(ProbabilityOverOne(value))
        } else {
            Err(BadProbability(value))
        }
    }
}