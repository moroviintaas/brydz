

use karty::cards::Card2SymTrait;

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};


use super::BridgeCoreErrorGen;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum FormatError{
    SerializeError,
    DeserializeError

}

impl<Card: Card2SymTrait>  From<FormatError> for BridgeCoreErrorGen<Card>{
    fn from(e: FormatError) -> Self {
        Self::Format(e)
    }
}