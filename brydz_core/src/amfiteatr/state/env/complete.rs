use std::ops::Index;
use log::{debug, error};
use karty::cards::Card2SymTrait;
use karty::error::{CardSetErrorGen};
use karty::set::{CardSetStd, HandSuitedTrait, CardSet};
use amfiteatr_core::env::{SequentialGameState, GameStateWithPayoffs, GameSummaryGen};
use amfiteatr_core::scheme::{Scheme, Renew};
use amfiteatr_core::error::AmfiteatrError;
use crate::contract::{Contract, ContractMechanics, ContractParameters, ContractParametersGen};
use crate::deal::{ContractGameDescription, DescriptionDeckDeal};
use crate::error::{BridgeCoreError, ContractErrorGen};
use crate::player::side::{Side, SIDES};
use crate::player::side::Side::*;
use crate::amfiteatr::spec::ContractDP;
use crate::amfiteatr::state::{ContractAction, ContractState, ContractStateUpdate};
use crate::amfiteatr::state::ContractAction::{PlaceCard, ShowHand};
use crate::bidding::consts::BID_H3;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ContractEnvStateComplete{
    dummy_hand: CardSetStd,
    declarer_hand: CardSetStd,
    whist_hand: CardSetStd,
    offside_hand: CardSetStd,
    contract: Contract,
    dummy_shown: bool,
    //fault_of_side: Option<Side>,
}

impl Default for ContractEnvStateComplete {
    fn default() -> Self {
        ContractEnvStateComplete{
            dummy_hand:     CardSetStd::from_str("KJT6.T4.K84.AQ94").unwrap(),
            declarer_hand:  CardSetStd::from_str("A3.KQ862.QJT5.K8").unwrap(),
            whist_hand:     CardSetStd::from_str("854.AJ73.932.J52").unwrap(),
            offside_hand:   CardSetStd::from_str("Q792.95.A76.T763").unwrap(),
            contract: Contract::new(ContractParameters::new(Side::South, BID_H3)),
            dummy_shown: false,
            //fault_of_side: None,
        }
    }

}

impl Index<Side> for ContractEnvStateComplete{
    type Output = CardSetStd;

    fn index(&self, index: Side) -> &Self::Output {
        match index - self.contract.declarer(){
            0 => &self.declarer_hand,
            1 => &self.whist_hand,
            2 => &self.dummy_hand,
            3 => &self.offside_hand,
            _ => panic!("No such role")
        }
    }
}



impl ContractEnvStateComplete{
    pub fn new(contract: Contract,
               declarer_hand: CardSetStd, whist_hand: CardSetStd,
               dummy_hand: CardSetStd, offside_hand: CardSetStd)
               -> Self{
        Self{contract, declarer_hand, whist_hand, dummy_hand, offside_hand, dummy_shown: false,
            //fault_of_side: None
            }
    }
    fn _index_mut(&mut self, index: Side) -> &mut CardSetStd {
        match index - self.contract.declarer(){
            0 => &mut self.declarer_hand,
            1 => &mut self.whist_hand,
            2 => &mut self.dummy_hand,
            3 => &mut self.offside_hand,
            _ => panic!("No such role")
        }
    }
}

impl ContractState for ContractEnvStateComplete{
    fn dummy_hand(&self) -> Option<&CardSetStd> {
        Some(&self.dummy_hand)
    }

    fn contract_data(&self) -> &Contract {
        &self.contract
    }
}

impl SequentialGameState<ContractDP> for ContractEnvStateComplete{
    type Updates = [(Side, ContractStateUpdate);4];

    fn current_player(&self) -> Option<Side> {
        match self.contract.is_completed(){
            true => None,
            false => Some(match self.contract.dummy() == self.contract.current_side(){
                true => {
                    if !self.dummy_shown {
                        self.contract.current_side()
                    } else {
                        self.contract.current_side().partner()
                    }
                },/*
                    match self.dummy_hand{
                    None => self.contract.dummy(),
                    Some(_) => self.contract.dummy().partner(),
                }
                */
                false => self.contract.current_side()
            })
        }
    }
    fn is_finished(&self) -> bool {
        self.contract.is_completed()
    }

    /// ```
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::{Contract, ContractParameters, ContractParametersGen};
    /// use brydz_core::player::side::Side::{East, North, South};
    /// use brydz_core::amfiteatr::state::{ContractAction, ContractEnvStateComplete};
    /// use karty::card_set;
    /// use karty::cards::*;
    /// use karty::suits::Suit::Spades;
    /// use amfiteatr_core::env::SequentialGameState;
    /// let hand_north = card_set!(TEN_CLUBS, ACE_DIAMONDS, QUEEN_HEARTS, QUEEN_SPADES);
    /// let hand_east = card_set!(FOUR_CLUBS, THREE_DIAMONDS, SIX_HEARTS, EIGHT_SPADES);
    /// let hand_south = card_set!(NINE_CLUBS, SIX_DIAMONDS, TEN_HEARTS, ACE_SPADES);
    /// let hand_west = card_set!(SIX_CLUBS, EIGHT_DIAMONDS, EIGHT_HEARTS, JACK_SPADES);
    /// let contract_parameters = ContractParameters::new(North, Bid::init(TrumpGen::Colored(Spades), 2).unwrap());
    /// let contract = Contract::new(contract_parameters);
    /// let mut state = ContractEnvStateComplete::new(contract, hand_north, hand_east, hand_south, hand_west);
    /// state.forward(East, ContractAction::PlaceCard(THREE_DIAMONDS)).unwrap();
    /// assert!(state.forward(South, ContractAction::PlaceCard(NINE_CLUBS)).is_err());
    /// ```
    fn forward(&mut self, side: Side, action: ContractAction) -> Result<Self::Updates, BridgeCoreError> {


        debug!("Forwarding environment state by agent {:} using action {}", &side, &action);
        match action{
            ShowHand(dhand) => match side{
                s if s == self.contract.dummy() =>{
                    if dhand == self.dummy_hand{
                        let update =
                            ContractStateUpdate::new(self.dummy_side(), ShowHand(dhand));
                        self.dummy_shown = true;
                        Ok([
                            (North, update),
                            (East, update),
                            (South, update),
                            (West, update)])
                    } else {
                        Err(BridgeCoreError::Contract(ContractErrorGen::DummyCardSetMissmatch))
                    }
                },
                    /*
                    match self.dummy_hand{
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

                     */
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

                if !self[actual_side].contains(&card){
                    return Err(CardSetErrorGen::CardNotInSet(card).into());
                }
                if let Some(called_suit) = self.contract.current_trick().called_suit(){
                    if called_suit != card.suit() && self[actual_side].contains_in_suit(&called_suit){
                        error!("Player {side:} ignored called suit: {called_suit} and played card {card:}");
                        return Err(ContractErrorGen::IgnoredCalledSuit(actual_side, called_suit).into());
                    }
                }


                self.contract.insert_card(actual_side, card)?;

                /*
                if side == self.contract.dummy(){
                    self.dummy_hand.remove_card(&card)?;
                }
                if side == self.contract.declarer(){
                    self.declarer_hand.remove_card(&card)?;
                }
                if side == self.contract.declarer().next(){
                    self.whist_hand.remove_card(&card)?;
                }
                if side == self.contract.dummy().next(){
                    self.offside_hand.remove_card(&card)?;
                }*/

                self._index_mut(actual_side).remove_card(&card)?;
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

impl GameStateWithPayoffs<ContractDP> for ContractEnvStateComplete{


    fn state_payoff_of_player(&self, agent: &Side) -> <ContractDP as Scheme>::UniversalReward {
        self.contract.total_tricks_taken_axis(agent.axis()) as i32
    }

}

impl From<(ContractParameters, DescriptionDeckDeal,)> for ContractEnvStateComplete{

    fn from(base: (ContractParameters, DescriptionDeckDeal,)) -> Self {
        let ( params, descript) = base;


        let declarer = params.declarer();
        let contract = Contract::new(params);
        Self::new( contract,
                   descript.cards[&declarer],
                   descript.cards[&declarer.next_i(1)],
        descript.cards[&declarer.next_i(2)],
        descript.cards[&declarer.next_i(3)])
    }
}

impl From<(&ContractParameters, &DescriptionDeckDeal,)> for ContractEnvStateComplete {
    fn from(base: (&ContractParameters, &DescriptionDeckDeal,)) -> Self {
        let (params, descript) = base;
        let contract = Contract::new(params.clone());
        let declarer = params.declarer();
        Self::new(contract,
                  descript.cards[&declarer],
                  descript.cards[&declarer.next_i(1)],
                  descript.cards[&declarer.next_i(2)],
                  descript.cards[&declarer.next_i(3)])
    }
}

impl Renew<ContractDP, (&ContractParameters, &DescriptionDeckDeal)> for ContractEnvStateComplete{
    fn renew_from(&mut self, base: (&ContractParameters, &DescriptionDeckDeal)) -> Result<(), AmfiteatrError<ContractDP>> {
        let (params, descript) = base;
        self.contract = Contract::new(params.clone());
        let declarer = params.declarer();
        self.declarer_hand = descript.cards[&declarer];
        self.whist_hand = descript.cards[&declarer.next_i(1)];
        self.dummy_hand = descript.cards[&declarer.next_i(2)];
        self.offside_hand = descript.cards[&declarer.next_i(3)];
        self.dummy_shown = false;
        Ok(())
    }
}

impl From<&ContractGameDescription> for ContractEnvStateComplete {
    fn from(base: &ContractGameDescription) -> Self {
        let contract = Contract::new(base.parameters().clone());
        let declarer = contract.declarer();
        Self::new(contract,
                  base.cards()[&declarer],
                  base.cards()[&declarer.next_i(1)],
                  base.cards()[&declarer.next_i(2)],
                  base.cards()[&declarer.next_i(3)])
    }
}

impl Renew<ContractDP, &ContractGameDescription> for ContractEnvStateComplete{
    fn renew_from(&mut self, base: &ContractGameDescription) -> Result<(), AmfiteatrError<ContractDP>> {
        //let (params, descript) = base;
        self.contract = Contract::new(base.parameters().clone());
        let declarer = self.contract.declarer();
        self.declarer_hand = base.cards()[&declarer];
        self.whist_hand = base.cards()[&declarer.next_i(1)];
        self.dummy_hand = base.cards()[&declarer.next_i(2)];
        self.offside_hand = base.cards()[&declarer.next_i(3)];
        self.dummy_shown = false;
        Ok(())
    }
}


impl From<&ContractEnvStateComplete> for GameSummaryGen<ContractDP>{
    fn from(base: &ContractEnvStateComplete) -> Self {
        GameSummaryGen::new(
            SIDES.iter().map(|s| (*s, base.state_payoff_of_player(s))).collect(),
            base.contract.count_completed_tricks() as u64*4
                + base.contract.current_trick().count_cards() as u64,
            None,


        )
    }
}