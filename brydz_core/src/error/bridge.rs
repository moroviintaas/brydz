use std::fmt::{Display, Formatter};
use karty::cards::{Card2SymTrait, Card};
use karty::suits::{SuitTrait};

use crate::error::bidding::BiddingErrorGen;

#[cfg(feature= "amfiteatr")]
use amfiteatr_core::error::{AmfiteatrError};
#[cfg(feature= "amfiteatr")]
use crate::amfiteatr::spec::ContractDP;


use crate::error::contract::ContractErrorGen;
use crate::error::{DistributionError, CardSetErrorGen, ScoreError, TrickErrorGen};

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use crate::error::FormatError;



#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct Mismatch<T>{
    pub expected: T,
    pub found: T
}
impl<T: Copy> Copy for Mismatch<T>{}



impl<S: SuitTrait> Display for BiddingErrorGen<S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BridgeCoreErrorGen<Card: Card2SymTrait>{
    Contract(ContractErrorGen<Card>),
    Bidding(BiddingErrorGen<Card::Suit>),
    Score(ScoreError),
    Trick(TrickErrorGen<Card>),
    Distribution(DistributionError),
    Hand(CardSetErrorGen<Card>),
    Format(FormatError),
    Custom(String),


}

impl<Card: Card2SymTrait> Display for BridgeCoreErrorGen<Card> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            BridgeCoreErrorGen::Contract(deal_error)=> match f.alternate(){
                true => write!(f, "BridgeError::DealError {{ {deal_error:#} }} " ),
                false => write!(f, "BridgeError::DealError {{ {deal_error} }} " ),
            }
            _ => {write!(f, "error : {self:?}, //todo implement as thiserror")},
        }

    }
}

impl<Card: Card2SymTrait> std::error::Error for BridgeCoreErrorGen<Card>{}





pub type BridgeCoreError = BridgeCoreErrorGen<Card>;

#[cfg(feature = "amfiteatr")]
impl From<BridgeCoreError> for AmfiteatrError<ContractDP>{
    fn from(value: BridgeCoreError) -> Self {
        Self::Game(value)
    }
}
