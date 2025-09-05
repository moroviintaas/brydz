use karty::cards::Card2SymTrait;
use karty::suits::{SuitTrait, Suit};
use crate::bidding::Bid;
use crate::error::bridge::Mismatch;
use crate::error::BridgeCoreErrorGen;
use crate::player::side::Side;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BiddingErrorGen<SU: SuitTrait>{
    DoubleAfterDouble,
    DoubleAfterReDouble,
    ReDoubleWithoutDouble,
    ReDoubleAfterReDouble,
    DoubleOnVoidCall,
    ReDoubleOnVoidCall,
    IllegalBidNumber(u8),
    ViolatedOrder(Mismatch<Side>),
    BidTooLow(Mismatch<Bid<SU>>),
    DoubleOnSameAxis,
    ReDoubleOnSameAxis

}

pub type BiddingError = BiddingErrorGen<Suit>;

impl<Card: Card2SymTrait> From<BiddingErrorGen<Card::Suit>> for BridgeCoreErrorGen<Card>{
    fn from(e: BiddingErrorGen<Card::Suit>) -> Self {
        Self::Bidding(e)
    }
}