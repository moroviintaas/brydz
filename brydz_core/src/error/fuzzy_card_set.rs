use thiserror;
use karty::error::CardSetErrorGen;
use karty::symbol::CardSymbol;
use crate::player::side::Side;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum FuzzyCardSetErrorGen<Crd: CardSymbol>{
    #[error("Requested card is not in hand")]
    CardNotInHand(Crd),
    #[error("Parsing FuzzyCardSet")]
    Parse,
    #[error("Bad probabilities sum, expected: {expected:}, found {found:}")]
    BadProbabilitiesSum{
        expected: f32,
        found: f32},
    #[error("Downscale factor {0} is forbidden")]
    ForbiddenDownscale(f32),
    #[error("Probability is bad: {0} (over 1.0)")]
    ProbabilityOverOne(f32),
    #[error("Probability is bad: {0} (below 0.0)")]
    ProbabilityBelowZero(f32),
    #[error("Probability is bad: {0} (unspecified)")]
    BadProbability(f32),
    #[error("Card error: {0}")]
    CardSet(CardSetErrorGen<Crd>),
    #[error("Every side has 0 effective probability of getting this card")]
    ImpossibleSideSelection,
    #[error("Too few uncertain cards field to allocate for side: {0}")]
    OutOfUncertainCardsForSide(Side)



}

impl<Crd: CardSymbol> From<CardSetErrorGen<Crd>> for FuzzyCardSetErrorGen<Crd>{
    fn from(value: CardSetErrorGen<Crd>) -> Self {
        Self::CardSet(value)
    }
}