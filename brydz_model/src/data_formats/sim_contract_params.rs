use brydz_core::contract::ContractParameters;
use brydz_core::deal::DealDistribution;
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
}