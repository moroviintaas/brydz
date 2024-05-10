use karty::set::{CardSet, CardSetStd};
use crate::contract::{Contract, ContractMechanics, ContractParameters};
use crate::error::BridgeCoreError;
use crate::amfiteatr::state::{ContractAction, ContractState, ContractStateUpdate};
use log::{debug};
use amfiteatr_core::env::{SequentialGameState, GameStateWithPayoffs};
use amfiteatr_core::domain::{DomainParameters};
use crate::deal::DescriptionDeckDeal;
use crate::player::side::{Side};
use crate::player::side::Side::*;
use crate::amfiteatr::spec::ContractDP;
use crate::amfiteatr::state::ContractAction::{PlaceCard, ShowHand};

#[derive(Clone, Debug)]
pub struct ContractEnvStateMin{
    dummy_hand: Option<CardSetStd>,
    contract: Contract,
}

impl ContractEnvStateMin{

    pub fn new(contract: Contract, dummy_hand: Option<CardSetStd>) -> Self{
        Self{dummy_hand, contract }
    }

    pub fn dummy_hand(&self) -> Option<&CardSetStd>{
        self.dummy_hand.as_ref()
    }

    pub fn contract(&self) -> &Contract{
        &self.contract
    }
    pub fn replace_contract(&mut self, contract: Contract){
        self.contract = contract
    }

}

impl ContractState for ContractEnvStateMin{
    fn dummy_hand(&self) -> Option<&CardSetStd> {
        self.dummy_hand.as_ref()
    }

    fn contract_data(&self) -> &Contract {
        &self.contract
    }
}
/*
impl State<ContractProtocolSpec> for ContractEnvStateMin{

    fn update(&mut self, update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        debug!("Updating environment with {:?}", &update);
        let (side, action) = update.into_tuple();
        match action{
            ContractAction::ShowHand(dhand) => match side{
                s if s == self.contract.dummy() => match self.dummy_hand{
                    Some(_) => panic!("Behavior when dummy shows hand second time"),
                    None => {
                        self.dummy_hand = Some(dhand);
                        Ok(())
                    }

                }
                _ => panic!("Non defined behaviour when non dummy shows hand.")

            }
            ContractAction::PlaceCard(card) => {
                let actual_side = match self.contract.dummy() == self.contract.current_side(){
                    false => side,
                    true => match side == self.contract.dummy().partner(){
                        true => self.contract.dummy(),
                        false => side
                    }
                };
                self.contract.insert_card(actual_side, card)?;
                if side == self.contract.dummy(){
                    if let Some(ref mut dh) = self.dummy_hand{
                        dh.remove_card(&card)?
                    }
                }
                Ok(())

            }
        }
    }

}
*/
impl SequentialGameState<ContractDP> for ContractEnvStateMin{
    type Updates = [(Side, ContractStateUpdate);4];

    fn current_player(&self) -> Option<Side> {
        match self.contract.is_completed(){
            true => None,
            false => Some(match self.contract.dummy() == self.contract.current_side(){
                true => match self.dummy_hand{
                    None => self.contract.dummy(),
                    Some(_) => self.contract.dummy().partner(),
                }
                false => self.contract.current_side()
            })
        }
    }
    fn is_finished(&self) -> bool {
        self.contract.is_completed()
    }

    fn forward(&mut self, side: Side, action: ContractAction) -> Result<Self::Updates, BridgeCoreError> {


        debug!("Translating environment state by agent {:} using action {:?}", &side, &action);
        match action{
            ShowHand(dhand) => match side{
                s if s == self.contract.dummy() => match self.dummy_hand{
                    Some(_) => panic!("Behavior when dummy shows hand second time"),
                    None => {
                        self.dummy_hand = Some(dhand);
                        let update =
                            ContractStateUpdate::new(self.dummy_side(), ShowHand(dhand));
                        Ok([
                            (North, update),
                            (East, update),
                            (South, update),
                            (West, update)])
                    }

                }
                _ => panic!("Non defined behaviour when non dummy shows hand.")

            }
            PlaceCard(card) => {
                let actual_side = match self.contract.dummy() == self.contract.current_side(){
                    false => side,
                    true => match side == self.contract.dummy().partner(){
                        true => self.contract.dummy(),
                        false => side
                    }
                };
                self.contract.insert_card(actual_side, card)?;
                if side == self.contract.dummy(){
                    if let Some(ref mut dh) = self.dummy_hand{
                        dh.remove_card(&card)?
                    }
                }
                let update = ContractStateUpdate::new(actual_side, PlaceCard(card));
                Ok([
                            (North, update),
                            (East, update),
                            (South, update),
                            (West, update)])

            }
        }



    }
}

impl GameStateWithPayoffs<ContractDP> for ContractEnvStateMin{


    fn state_payoff_of_player(&self, agent: &Side) -> <ContractDP as DomainParameters>::UniversalReward {
        self.contract.total_tricks_taken_axis(agent.axis()) as i32
    }

}

impl From<(ContractParameters, DescriptionDeckDeal,)> for ContractEnvStateMin{

    fn from(base: (ContractParameters, DescriptionDeckDeal,)) -> Self {
        let ( params, _descript) = base;

        let contract = Contract::new(params);
        Self::new(contract, None)
    }
}

impl From<(&ContractParameters, &DescriptionDeckDeal)> for ContractEnvStateMin{
    fn from(base: (&ContractParameters, &DescriptionDeckDeal,)) -> Self {
        let (params, _descript) = base;
        let contract = Contract::new(params.clone());
        Self::new(contract, None)
    }
}