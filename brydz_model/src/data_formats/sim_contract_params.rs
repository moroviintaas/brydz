use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use brydz_core::bidding::Bid;
use brydz_core::cards::trump::TrumpGen;
use brydz_core::contract::ContractParameters;
use brydz_core::deal::{DealDistribution, DescriptionDeckDeal};
use brydz_core::player::side::Side::North;
use brydz_core::player::side::SideMap;
use karty::hand::CardSet;


#[derive(serde::Serialize, serde::Deserialize, Clone,) ]
pub struct SimContractParams {
    parameters: ContractParameters,
    //info_sets: SideMap<DistributionTemplate>
    deal_distribution: DealDistribution,
    cards: SideMap<CardSet>

}

impl SimContractParams{
    pub fn new(parameters: ContractParameters,
               deal_distribution: DealDistribution,
               cards: SideMap<CardSet>) -> Self{
        Self{parameters, deal_distribution, cards}
    }

    pub fn cards(&self) -> &SideMap<CardSet>{
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

impl From<SimContractParams> for DescriptionDeckDeal{
    fn from(value: SimContractParams) -> Self {
        DescriptionDeckDeal{
            probabilities: value.deal_distribution,
            cards: value.cards,
        }
    }
}