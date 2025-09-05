use std::ops::{Deref};
use log::debug;
use smallvec::SmallVec;
use karty::cards::{Card, Card2SymTrait};
use karty::set::{CardSetStd, HandSuitedTrait, CardSet};
use karty::register::Register;
use amfiteatr_core::agent::{InformationSet, PresentPossibleActions, EvaluatedInformationSet};
use amfiteatr_core::scheme::{Scheme, Renew};
use amfiteatr_core::error::AmfiteatrError;
use crate::contract::{Contract, ContractMechanics, ContractParameters};
use crate::deal::{BiasedHandDistribution, ContractGameDescription, DealDistribution, DescriptionDeckDeal};
use crate::error::BridgeCoreError;
use crate::meta::HAND_SIZE;
use crate::player::side::Side;
use crate::amfiteatr::spec::ContractDP;
use crate::amfiteatr::state::{ContractAction, ContractInfoSet, ContractStateUpdate, CreatedContractInfoSet, RenewableContractInfoSet, StateWithSide};

#[derive(Debug, Clone)]
pub struct ContractAgentInfoSetAssuming {
    side: Side,
    hand: CardSetStd,
    dummy_hand: Option<CardSetStd>,
    contract: Contract,
    card_distribution: BiasedHandDistribution,
}

impl ContractAgentInfoSetAssuming{
    #[allow(dead_code)]
    pub fn new(side: Side, hand: CardSetStd, contract: Contract, dummy_hand: Option<CardSetStd>, card_distribution: BiasedHandDistribution) -> Self{
        Self{side, hand, dummy_hand, contract, card_distribution}
    }
    #[allow(dead_code)]
    pub fn new_fair(side: Side, hand: CardSetStd, contract: Contract, dummy_hand: Option<CardSetStd>) -> Self{
        Self{side, hand, dummy_hand, contract, card_distribution: Default::default()}
    }

    pub fn side(&self) -> &Side{
        &self.side
    }
    pub fn contract(&self) -> &Contract{
        &self.contract
    }
    pub fn hand(&self) -> &CardSetStd {
        &self.hand
    }
    pub fn dummy_hand(&self) -> Option<&CardSetStd>{
        self.dummy_hand.as_ref()
    }
    pub fn distribution_assumption(&self) -> &BiasedHandDistribution{
        &self.card_distribution
    }

    pub fn possibly_has_card(&self, side: Side, card: &Card) -> bool{
        if !self.contract.side_possibly_has_card(side, card){
            return false;
        }
        if self.card_distribution[side][card].is_zero(){
            return false;
        }
        if side == self.side && !self.hand.contains(card){
            return false;
        }
        if side != self.side && self.hand.contains(card){
            return false;
        }
        if let Some(d) = self.dummy_hand {
            if side == self.contract.dummy() && !d.contains(card){
                return false;
            }
            if side != self.contract.dummy() && d.contains(card){
                return false;
            }
        }
        true

    }
    pub fn surely_has_card(&self, side: Side, card: &Card) -> bool{
        if side == self.side && self.hand.contains(card){
            return true;
        }
        if let Some(d) = self.dummy_hand {
            if side == self.contract.dummy() && d.contains(card){
                return true;
            }

        }
        false
    }



}



impl InformationSet<ContractDP> for ContractAgentInfoSetAssuming {

    fn agent_id(&self) -> &<ContractDP as Scheme>::AgentId {
        &self.side
    }

    fn is_action_valid(&self, action: &ContractAction) -> bool {
        match action{
            ContractAction::ShowHand(_h) => {
                self.contract.dummy() == self.side
            }
            ContractAction::PlaceCard(c) => match self.hand.contains(c){
                true => match self.contract.current_trick().called_suit(){
                    None => true,
                    Some(s) => {
                        if s == c.suit(){
                            true
                        } else {
                            !self.hand.contains_in_suit(&s)
                        }
                    }
                }
                false => false
            }
        }
    }

    fn update(&mut self, update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        //debug!("Agent {} received state update: {:?}", self.side, &update);
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
                debug!("Agent {:?}: actual_side: {:?}", &self.side, &actual_side);
                self.contract.insert_card(actual_side, card)?;
                if actual_side == self.side{
                    self.hand.remove_card(&card)?
                }
                if actual_side == self.contract.dummy(){
                    if let Some(ref mut dh) = self.dummy_hand{
                        dh.remove_card(&card)?
                    }
                }
                Ok(())

            }
        }
    }


}

impl PresentPossibleActions<ContractDP> for ContractAgentInfoSetAssuming{
    type ActionIteratorType = SmallVec<[ContractAction; HAND_SIZE]>;


    fn available_actions(&self) -> Self::ActionIteratorType {
        match self.contract.current_side(){
            dec if dec == self.side => {

                match self.contract.current_trick().called_suit(){
                    None => self.hand.into_iter()
                         .map( ContractAction::PlaceCard).collect(),
                    Some(called) => match self.hand.contains_in_suit(&called){
                        true => self.hand.suit_iterator(&called)
                            .map(ContractAction::PlaceCard).collect(),
                        false => self.hand.into_iter()
                            .map(ContractAction::PlaceCard).collect()
                    }
                }
            },
            dummy if dummy == self.side.partner()  && dummy == self.contract.dummy()=> {

                if let Some(dh) = self.dummy_hand{
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

impl EvaluatedInformationSet<ContractDP, i32> for ContractAgentInfoSetAssuming {

    fn current_assessment(&self) -> i32{
        self.contract.total_tricks_taken_axis(self.side.axis()) as i32
    }

    fn penalty_for_illegal(&self) -> i32 {
        -100
    }
}

impl RenewableContractInfoSet for ContractAgentInfoSetAssuming{
    fn renew(&mut self, hand: CardSetStd, contract: Contract, dummy_hand: Option<CardSetStd>) {
        self.hand = hand;
        self.contract = contract;
        self.dummy_hand = dummy_hand;
    }
}

impl CreatedContractInfoSet for ContractAgentInfoSetAssuming{
    fn create_new(side: Side, hand: CardSetStd, contract: Contract, dummy_hand: Option<CardSetStd>, distribution: BiasedHandDistribution) -> Self {
        Self{
            side,
            hand,
            dummy_hand,
            contract,
            card_distribution: distribution
        }
    }
}

impl StateWithSide for ContractAgentInfoSetAssuming{
    fn id(&self) -> Side {
        self.side
    }
}

impl ContractInfoSet for ContractAgentInfoSetAssuming{
    fn side(&self) -> Side {
        self.side
    }

    fn contract_data(&self) -> &Contract {
        &self.contract
    }

    fn dummy_hand(&self) -> Option<&CardSetStd> {
        self.dummy_hand.as_ref()
    }

    fn hand(&self) -> &CardSetStd {
        &self.hand
    }

    fn hint_card_probability_for_player(&self, side: Side, card: &Card) -> f32 {
        if self.contract.card_used().is_registered(card){
            return 0.0;
        }
        if self.contract.suits_exhausted().is_registered(&(side, card.suit())){
            return 0.0;
        }
        if self.side == side{
            match self.hand.contains(card){
                true => 1.0,
                false => 0.0
            }
        } else {
            if self.hand.contains(card){
                return 0.0; //this player has, other cant
            }
            if let Some(d) = self.dummy_hand{
                //dummy shown
                if side == self.contract.dummy(){
                    //check dummys card
                    return match d.contains(card){
                        true => 1.0,
                        false => 0.0
                    }
                }
                if d.contains(card){
                    return 0.0;
                }

                //neither self nor dummy, card not marked as used
                let initial_proba_this: f32 = self.card_distribution[self.side][card].into();
                let initial_proba_dummy: f32 = self.card_distribution[self.contract.dummy()][card].into();
                let remaining_proba = 1.0 - initial_proba_dummy - initial_proba_this;
                assert!(remaining_proba >= 0.0);
                let c_proba: f32 = self.card_distribution[side][card].into();
                c_proba / remaining_proba


            } else {
                //dummy not shown, anyone except this can have
                let initial_proba_this: f32 = self.card_distribution[self.side][card].into();
                let remaining_proba = 1.0 -  initial_proba_this;
                let c_proba: f32 = self.card_distribution[side][card].into();
                c_proba / remaining_proba
            }
        }
    }
}
impl From<(Side, ContractParameters, DescriptionDeckDeal,)> for ContractAgentInfoSetAssuming{

    fn from(base: (Side, ContractParameters, DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

         let distr = match descript.probabilities{
            DealDistribution::Fair => Default::default(),
            DealDistribution::Biased(biased) => biased.deref().clone()
        };

        let contract = Contract::new(params);
        Self::new(side, descript.cards[&side] , contract, None, distr)
    }
}
impl From<(&Side, &ContractParameters, &DescriptionDeckDeal,)> for ContractAgentInfoSetAssuming{
    fn from(base: (&Side, &ContractParameters, &DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

        let distr = match &descript.probabilities{
            DealDistribution::Fair => Default::default(),
            DealDistribution::Biased(biased) => biased.deref().clone()
        };

        let contract = Contract::new(params.clone());
        Self::new(*side, descript.cards[side], contract, None, distr)
    }
}

impl Renew<ContractDP, (&Side, &ContractParameters, &DescriptionDeckDeal)> for ContractAgentInfoSetAssuming{
    fn renew_from(&mut self, base: (&Side, &ContractParameters, &DescriptionDeckDeal)) -> Result<(), AmfiteatrError<ContractDP>> {
        let (side, params, descript) = base;

        let contract = Contract::new(params.clone());
        let distr = match &descript.probabilities{
            DealDistribution::Fair => Default::default(),
            DealDistribution::Biased(biased) => biased.deref().clone()
        };
        self.dummy_hand = None;
        self.contract = contract;
        self.side = *side;
        self.hand = descript.cards[side];
        self.card_distribution = distr;
        Ok(())
    }
}

impl From<(&Side, &ContractGameDescription)> for ContractAgentInfoSetAssuming{
    fn from(base: (&Side, &ContractGameDescription)) -> Self {
        let (side, description) = base;

        let distr = match &description.distribution(){
            DealDistribution::Fair => Default::default(),
            DealDistribution::Biased(biased) => biased.deref().clone()
        };

        let contract = Contract::new(description.parameters().clone());
        Self::new(*side, description.cards()[side], contract, None, distr)
    }
}

impl Renew<ContractDP, (&Side, &ContractGameDescription)> for ContractAgentInfoSetAssuming{
    fn renew_from(&mut self, base: (&Side, &ContractGameDescription)) -> Result<(), AmfiteatrError<ContractDP>> {
        let (side, description) = base;

        let contract = Contract::new(description.parameters().clone());
        let distr = match &description.distribution(){
            DealDistribution::Fair => Default::default(),
            DealDistribution::Biased(biased) => biased.deref().clone()
        };
        self.dummy_hand = None;
        self.contract = contract;
        self.side = *side;
        self.hand = description.cards()[side];
        self.card_distribution = distr;
        Ok(())
    }
}