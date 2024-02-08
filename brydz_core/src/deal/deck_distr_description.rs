use karty::hand::CardSet;
use crate::deal::{DealDistribution};
use crate::player::side::SideMap;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct DescriptionDeckDeal{
    pub probabilities: DealDistribution,
    pub cards: SideMap<CardSet>
}
