use rand::prelude::ThreadRng;
use karty::set::CardSetStd;
use crate::bidding::Bid;
use crate::cards::trump::TrumpGen;
use crate::contract::ContractParameters;
use crate::deal::{DealDistribution};
use crate::player::side::Side::North;
use crate::player::side::{Side, SideMap};
use rand_distr::Distribution as RandDistribution;
use amfiteatr_core::scheme::Renew;
use crate::amfiteatr::spec::ContractDP;
use crate::amfiteatr::state::ContractAgentInfoSetSimple;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct DescriptionDeckDeal{
    pub probabilities: DealDistribution,
    pub cards: SideMap<CardSetStd>
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct ContractGameDescription {
    parameters: ContractParameters,
    //info_sets: SideMap<DistributionTemplate>
    deal_distribution: DealDistribution,
    cards: SideMap<CardSetStd>

}

impl ContractGameDescription {
    pub fn new(parameters: ContractParameters,
               deal_distribution: DealDistribution,
               cards: SideMap<CardSetStd>) -> Self{
        Self{parameters, deal_distribution, cards}
    }

    pub fn cards(&self) -> &SideMap<CardSetStd>{
        &self.cards
    }
    pub fn parameters(&self) -> &ContractParameters{
        &self.parameters
    }
    pub fn distribution(&self) -> &DealDistribution{
        &self.deal_distribution
    }

    pub fn new_fair_random(rng: &mut ThreadRng) -> Self{
        let dd = DealDistribution::Fair;
        let params = ContractParameters::new(North, Bid::init(TrumpGen::NoTrump, 1).unwrap());
        let cards = dd.sample(rng);
        Self{
            parameters: params,
            deal_distribution: dd,
            cards
        }
    }
}

impl From<ContractGameDescription> for DescriptionDeckDeal{
    fn from(value: ContractGameDescription) -> Self {
        DescriptionDeckDeal{
            probabilities: value.deal_distribution,
            cards: value.cards,
        }
    }
}

