/*
use karty::cards::Card2SymTrait;

use crate::error::BridgeCoreErrorGen;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum HandError{
    CardNotInHand,
    EmptyHand,
    HandFull,
    CardDuplicated,
    HandNotInitialised,
}

impl<Card: Card2SymTrait> From<HandError> for BridgeCoreErrorGen<Card>{
    fn from(e: HandError) -> Self {
        Self::Hand(e)
    }
}

 */

use karty::cards::Card2SymTrait;
use karty::error::CardSetErrorGen;
use crate::error::BridgeCoreErrorGen;

impl<Card: Card2SymTrait> From<CardSetErrorGen<Card>> for BridgeCoreErrorGen<Card>{
    fn from(e: CardSetErrorGen<Card>) -> Self {
        Self::Hand(e)
    }
}