use std::fmt::{Display, Formatter};
use karty::cards::{Card, Card2SymTrait};
use crate::error::{BridgeCoreErrorGen, Mismatch};
use crate::player::side::Side;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Eq, PartialEq,  Clone)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum TrickErrorGen<Card: Card2SymTrait>{
    MissingCard(Side),
    CardSlotAlreadyUsed(Side),
    DuplicateCard(Card),
    ImposibleUndo,
    ViolatedOrder(Mismatch<Side>),
    UsedPreviouslyExhaustedSuit(Card::Suit),
    TrickFull,
    TrickNotEmpty,
    TrickEmpty
}
impl<Card: Card2SymTrait> Display for TrickErrorGen<Card> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

pub type TrickError = TrickErrorGen<Card>;

impl<Card: Card2SymTrait> From<TrickErrorGen<Card>> for BridgeCoreErrorGen<Card>{
    fn from(e: TrickErrorGen<Card>) -> Self {
        Self::Trick(e)
    }
}