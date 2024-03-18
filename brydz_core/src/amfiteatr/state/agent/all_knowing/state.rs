use log::debug;
use smallvec::SmallVec;
use karty::cards::{Card, Card2SymTrait};
use karty::hand::{CardSet, HandSuitedTrait, HandTrait};
use karty::register::Register;
use amfiteatr_core::agent::{InformationSet, PresentPossibleActions, EvaluatedInformationSet};
use amfiteatr_core::domain::{DomainParameters, Renew};
use amfiteatr_core::error::AmfiteatrError;

use crate::contract::{Contract, ContractMechanics, ContractParameters};
use crate::deal::{DealDistribution, DescriptionDeckDeal};
use crate::error::{BridgeCoreError, BridgeCoreErrorGen};
use crate::error::ContractErrorGen::CardNotInHand;
use crate::meta::HAND_SIZE;
use crate::player::side::{Side, SideMap};
use crate::amfiteatr::spec::ContractDP;
use crate::amfiteatr::state::{ContractAction, ContractAgentInfoSetAssuming, ContractInfoSet, ContractStateUpdate, StateWithSide};

#[derive(Debug, Clone)]

pub struct ContractAgentInfoSetAllKnowing {
    side: Side,
    deal: SideMap<CardSet>,
    initial_deal: SideMap<CardSet>,
    contract: Contract,
}

impl ContractAgentInfoSetAllKnowing{
    pub fn new(side: Side, deal: SideMap<CardSet>, contract: Contract) -> Self{
        Self{side, deal, contract, initial_deal: deal}
    }
    pub fn side(&self) -> &Side{
        &self.side
    }
    pub fn contract(&self) -> &Contract{
        &self.contract
    }
    pub fn hand(&self) -> &CardSet{
        &self.deal[&self.side]
    }
    pub fn dummy_hand(&self) -> Option<&CardSet>{
        Some(&self.deal[&self.contract.dummy()])
    }
    pub fn initial_deal(&self) -> &SideMap<CardSet>{
        &self.initial_deal
    }



}

impl InformationSet<ContractDP> for ContractAgentInfoSetAllKnowing{
    fn agent_id(&self) -> &<ContractDP as DomainParameters>::AgentId {
        &self.side
    }

    fn is_action_valid(&self, action: &ContractAction) -> bool {
        match action{
            ContractAction::ShowHand(_h) => {
                self.contract.dummy() == self.side
            }
            ContractAction::PlaceCard(c) => match self.hand().contains(c){
                true => match self.contract.current_trick().called_suit(){
                    None => true,
                    Some(s) => {
                        if s == c.suit(){
                            true
                        } else {
                            !self.hand().contains_in_suit(&s)
                        }
                    }
                }
                false => false
            }
        }
    }

    fn update(&mut self, update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        let (side, action) = update.into_tuple();
        match action{
            ContractAction::ShowHand(dhand) => {
                let local_dhand = self.dummy_hand().unwrap();
                if local_dhand != &dhand{
                    todo!()
                    //panic!("Currenly not implemented what to do when dummys showed hand is different than known in infoset")
                }
                Ok(())


            }
            ContractAction::PlaceCard(card) => {
                let actual_side = match self.contract.dummy() == self.contract.current_side(){
                    false => side,
                    true => match side == self.contract.dummy().partner(){
                        true => self.contract.dummy(),
                        false => side
                    }
                };
                debug!("Agent {:?}: actual_side: {:?}", &self.side, &actual_side);
                if !self.deal[&actual_side].contains(&card){
                    //used card known to not be in players hand
                    return Err(BridgeCoreErrorGen::Contract(CardNotInHand(actual_side, card)))
                }
                self.contract.insert_card(actual_side, card)?;
                self.deal[&actual_side].remove_card(&card)?;
                Ok(())

            }
        }
    }
}

impl PresentPossibleActions<ContractDP> for ContractAgentInfoSetAllKnowing{
    type ActionIteratorType = SmallVec<[ContractAction; HAND_SIZE]>;

    fn available_actions(&self) -> Self::ActionIteratorType {
        match self.contract.current_side(){
            dec if dec == self.side => {

                match self.contract.current_trick().called_suit(){
                    None => self.hand().into_iter()
                         .map( ContractAction::PlaceCard).collect(),
                    Some(called) => match self.hand().contains_in_suit(&called){
                        true => self.hand().suit_iterator(&called)
                            .map(ContractAction::PlaceCard).collect(),
                        false => self.hand().into_iter()
                            .map(ContractAction::PlaceCard).collect()
                    }
                }
            },
            dummy if dummy == self.side.partner()  && dummy == self.contract.dummy()=> {

                if let Some(dh) = self.dummy_hand(){
                    match self.contract.current_trick().called_suit(){
                            None => dh.into_iter()
                                 .map(ContractAction::PlaceCard).collect(),
                            Some(called) => match dh.contains_in_suit(&called){
                                true => dh.suit_iterator(&called)
                                     .map(ContractAction::PlaceCard).collect(),
                                false => dh.into_iter()
                                     .map( ContractAction::PlaceCard).collect()
                            }
                        }
                } else {
                    SmallVec::new()
                }

            },
            _ => SmallVec::new()
        }
    }
}

impl EvaluatedInformationSet<ContractDP> for ContractAgentInfoSetAllKnowing {
    type RewardType = i32;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.contract.total_tricks_taken_axis(self.side.axis()) as i32
    }

    fn penalty_for_illegal(&self) -> Self::RewardType {
        -100
    }
}

impl StateWithSide for ContractAgentInfoSetAllKnowing{
    fn id(&self) -> Side {
        self.side
    }
}

impl ContractInfoSet for ContractAgentInfoSetAllKnowing{
    fn side(&self) -> Side {
        self.side
    }

    fn contract_data(&self) -> &Contract {
        &self.contract
    }

    fn dummy_hand(&self) -> Option<&CardSet> {
        self.dummy_hand()
    }

    fn hand(&self) -> &CardSet {
        self.hand()
    }

    fn hint_card_probability_for_player(&self, side: Side, card: &Card) -> f32 {
        if self.contract.card_used().is_registered(card){
            return 0.0;
        }
        match self.deal[&side].contains(card){
            true => 1.0,
            false => 0.0
        }
    }
}

impl From<(Side, ContractParameters, DescriptionDeckDeal,)> for ContractAgentInfoSetAllKnowing{

    fn from(base: (Side, ContractParameters, DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;
        let contract = Contract::new(params);
        Self::new(side, descript.cards , contract)
    }
}
impl From<(&Side, &ContractParameters, &DescriptionDeckDeal,)> for ContractAgentInfoSetAllKnowing{
    fn from(base: (&Side, &ContractParameters, &DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

        let contract = Contract::new(params.clone());
        Self::new(*side, descript.cards, contract)
    }
}
impl Renew<ContractDP, (&Side, &ContractParameters, &DescriptionDeckDeal)> for ContractAgentInfoSetAllKnowing{
    fn renew_from(&mut self, base: (&Side, &ContractParameters, &DescriptionDeckDeal)) -> Result<(), AmfiteatrError<ContractDP>> {
        let (side, params, descript) = base;

        let contract = Contract::new(params.clone());
        self.contract = contract;
        self.side = *side;
        self.initial_deal = descript.cards;
        Ok(())
    }
}
