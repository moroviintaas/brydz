use karty::cards::Card;
use karty::set::CardSetStd;
use amfiteatr_core::agent::{InformationSet};
use crate::contract::{Contract, ContractMechanics};
use crate::deal::BiasedHandDistribution;
use crate::player::side::Side;
use crate::amfiteatr::spec::ContractDP;

pub trait ContractInfoSet{
    fn side(&self) -> Side;
    fn contract_data(&self) -> &Contract;
    fn dummy_hand(&self) -> Option<&CardSetStd>;
    fn dummy_side(&self) -> Side{
        self.contract_data().dummy()
    }
    fn hand(&self) -> &CardSetStd;
    fn hint_card_probability_for_player(&self, side: Side, card: &Card) -> f32;

}

pub trait RenewableContractInfoSet: InformationSet<ContractDP>{
    fn renew(&mut self, hand: CardSetStd, contract: Contract, dummy_hand: Option<CardSetStd>);

}
impl<T: RenewableContractInfoSet> RenewableContractInfoSet for Box<T>{
    fn renew(&mut self, hand: CardSetStd, contract: Contract, dummy_hand: Option<CardSetStd>) {
        self.as_mut().renew(hand, contract, dummy_hand)
    }
}

pub trait CreatedContractInfoSet: InformationSet<ContractDP>{
    fn create_new(side: Side, hand: CardSetStd, contract: Contract, dummy_hand: Option<CardSetStd>, distribution: BiasedHandDistribution) -> Self;
}

impl<T: CreatedContractInfoSet> CreatedContractInfoSet for Box<T>{
    fn create_new(side: Side, hand: CardSetStd, contract: Contract, dummy_hand: Option<CardSetStd>, distribution: BiasedHandDistribution) -> Self {
        Box::new(T::create_new(side, hand, contract, dummy_hand, distribution))
    }
}

//#[cfg(feature = "neuro")]
//pub trait ContractInfoSetFull: CreatedContractInfoSet + RenewableContractInfoSet
/*
pub trait StandardContractInfoSet: CreatedContractInfoSet + RenewableContractInfoSet{}

impl<T: StandardContractInfoSet> StandardContractInfoSet for Box<T>{}

 */