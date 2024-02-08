use std::fmt::{Display, Formatter};
use karty::{cards::Card2SymTrait};
use crate::error::BridgeCoreErrorGen;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ScoreError{
    NegativeTrickNumber
}

impl Display for ScoreError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<Card: Card2SymTrait> From<ScoreError> for BridgeCoreErrorGen<Card>{
    fn from(e: ScoreError) -> Self {
        Self::Score(e)
    }
}