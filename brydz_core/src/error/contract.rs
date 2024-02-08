use std::fmt::{Display, Formatter};
use karty::cards::{Card2SymTrait, Card};
use crate::error::{BridgeCoreErrorGen, TrickErrorGen};
use crate::player::side::Side;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ContractErrorGen<Card: Card2SymTrait>{
    ContractFull,
    DealIncomplete,
    DuplicateCard(Card),
    BadTrick(TrickErrorGen<Card>),
    IndexedOverCurrentTrick(usize),
    DummyReplaceAttempt,
    DummyNotPlaced,
    DummyCardSetMissmatch,
    CurrentSidePresume(Side, Side),
    UndoOnEmptyContract,
    UsedExhaustedSuit(Side, Card::Suit),
    IgnoredCalledSuit(Side, Card::Suit),
    CardNotInHand(Side, Card),

}
impl<Card: Card2SymTrait>Display for ContractErrorGen<Card>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub type ContractError = ContractErrorGen<Card>;

impl<Card: Card2SymTrait> From<ContractErrorGen<Card>> for BridgeCoreErrorGen<Card>{
    fn from(e: ContractErrorGen<Card>) -> Self {
        Self::Contract(e)
    }
}