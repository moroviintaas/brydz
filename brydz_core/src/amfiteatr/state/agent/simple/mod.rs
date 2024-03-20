use smallvec::SmallVec;
use karty::hand::{HandSuitedTrait, HandTrait, CardSet};
use crate::contract::{Contract, ContractMechanics, ContractParameters};
use crate::error::BridgeCoreError;
use crate::meta::HAND_SIZE;
use crate::player::side::Side;
use crate::amfiteatr::state::{ContractAction, ContractInfoSet, ContractStateUpdate, CreatedContractInfoSet, RenewableContractInfoSet, StateWithSide};
use log::debug;
use karty::cards::{Card, Card2SymTrait};
use karty::register::Register;
use crate::deal::{BiasedHandDistribution, ContractGameDescription, DescriptionDeckDeal};
use crate::amfiteatr::spec::ContractDP;

#[cfg(feature = "torch")]
mod state_history_tensor;
#[cfg(feature = "torch")]
mod state_tensor;
//#[cfg(feature = "neuro")]
//pub use state_tensor::*;
use amfiteatr_core::agent::{InformationSet, PresentPossibleActions, EvaluatedInformationSet};
use amfiteatr_core::domain::{DomainParameters, Renew};
use amfiteatr_core::error::AmfiteatrError;

#[derive(Debug, Clone)]
pub struct ContractAgentInfoSetSimple {
    side: Side,
    hand: CardSet,
    dummy_hand: Option<CardSet>,
    contract: Contract
}

impl ContractAgentInfoSetSimple {
    pub fn new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>) -> Self{
        Self{side, hand, dummy_hand, contract}
    }

    /// ```
    /// use brydz_core::bidding::{Bid, Doubling};
    /// use brydz_core::cards::deck::Deck;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::{*};
    /// use brydz_core::player::side::Side::*;
    /// use brydz_core::amfiteatr::state::ContractAgentInfoSetSimple;
    /// use karty::card_set;
    /// use karty::cards::*;
    /// use karty::suits::Suit::*;
    /// let deck = Deck::new_sorted_by_figures();
    /// let mut deal_1 = Contract::new(ContractParametersGen::new_d(North, Bid::init(TrumpGen::Colored(Diamonds), 1).unwrap(), Doubling::None));
    ///
    /// deal_1.insert_card(East, KING_SPADES).unwrap();
    /// deal_1.insert_card(South, QUEEN_SPADES).unwrap();
    /// deal_1.insert_card(West, JACK_SPADES).unwrap();
    /// deal_1.insert_card(North, TWO_HEARTS).unwrap();
    /// let i_s = ContractAgentInfoSetSimple::new(West, card_set![KING_HEARTS], deal_1, Some(card_set![TEN_HEARTS]));
    /// assert!(i_s.possibly_has_card(West, &KING_HEARTS)); //has it
    /// assert!(!i_s.possibly_has_card(West, &QUEEN_HEARTS)); // does not have
    /// assert!(i_s.possibly_has_card(East, &QUEEN_HEARTS)); // hand not known, maybe
    /// assert!(!i_s.possibly_has_card(East, &KING_SPADES)); // hand not known, card used
    /// assert!(!i_s.possibly_has_card(North, &TWO_SPADES)); // hand not known, exhausted suit
    /// assert!(i_s.possibly_has_card(North, &THREE_HEARTS)); //hand not known, maybe
    /// assert!(!i_s.possibly_has_card(North, &KING_HEARTS)); //hand not known, card in own hand
    /// assert!(!i_s.possibly_has_card(North, &TEN_HEARTS)); //hand not known, card in dummy hand
    /// assert!(!i_s.possibly_has_card(South, &TWO_SPADES)); // dummy hand known, does not have
    /// assert!(i_s.possibly_has_card(South, &TEN_HEARTS)); // dummy hand known, has
    /// ```
    pub fn possibly_has_card(&self, side: Side, card: &Card) -> bool{
        if !self.contract.side_possibly_has_card(side, card){
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

impl InformationSet<ContractDP> for ContractAgentInfoSetSimple {
    fn agent_id(&self) -> &<ContractDP as DomainParameters>::AgentId {
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

impl PresentPossibleActions<ContractDP> for ContractAgentInfoSetSimple{
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
impl EvaluatedInformationSet<ContractDP> for ContractAgentInfoSetSimple {
    type RewardType = i32;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.contract.total_tricks_taken_axis(self.side.axis()) as i32
    }

    fn penalty_for_illegal(&self) -> Self::RewardType {
        -100
    }
}

impl RenewableContractInfoSet for ContractAgentInfoSetSimple{
    fn renew(&mut self, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>) {
        self.hand = hand;
        self.contract = contract;
        self.dummy_hand = dummy_hand;
    }
}

impl CreatedContractInfoSet for ContractAgentInfoSetSimple{
    fn create_new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>, _distribution: BiasedHandDistribution) -> Self {
        Self{
            side,
            hand,
            dummy_hand,
            contract,
        }
    }
}

impl StateWithSide for ContractAgentInfoSetSimple{
    fn id(&self) -> Side {
        self.side
    }
}


#[cfg(feature = "torch")]
mod tensor{
    //use tensorflow::{QUInt8, Tensor};
    use crate::amfiteatr::state::ContractAgentInfoSetSimple;
    use karty::cards::{Card2SymTrait, DECK_SIZE, STANDARD_DECK_CDHS};
    use karty::hand::{ HandTrait};
    use karty::register::Register;
    use karty::symbol::CardSymbol;
    use crate::bidding::Doubling;
    use crate::cards::trump::TrumpGen;
    use crate::contract::ContractMechanics;
    //use crate::meta::DECK_SIZE;



    const SURE: u8 = 120;
    const ONE_IN_TWO: u8 = SURE/2;
    const ONE_IN_THREE: u8 = SURE/3;

    const TRICK_STARTER: usize = DECK_SIZE * 4;
    const TRICK_COMPLETION: usize = TRICK_STARTER + 1;
    const OWN_CARD: usize = TRICK_COMPLETION + 1;
    const LEFT_CARD: usize = OWN_CARD + 2;

    const PARTNER_CARD: usize = LEFT_CARD + 2;
    const RIGHT_CARD: usize = PARTNER_CARD + 2;
    const BID_OFFSET: usize = RIGHT_CARD + 2;
    const PLAY_AS_DUMMY: usize = BID_OFFSET + 3;



    const SIMPLE_INFO_SET_LENGTH:usize = PLAY_AS_DUMMY + 1;



    impl From<&ContractAgentInfoSetSimple> for [u8;SIMPLE_INFO_SET_LENGTH] {
        fn from(state: &ContractAgentInfoSetSimple) -> Self {
            let dummy_offset = (state.contract.dummy() - state.side) as usize;

            let unknown_side_1 = state.side.first_unknown_side(state.contract.declarer());
            let unknown_side_2 = state.side.second_unknown_side(state.contract.declarer());
            let unknown_offset_1 = (unknown_side_1 - state.side) as usize;
            let unknown_offset_2 = (unknown_side_2 - state.side) as usize;

            //let mut array:[QUInt8;SIMPLE_INFO_SET_LENGTH] = [QUInt8::zero();SIMPLE_INFO_SET_LENGTH];
            let mut array = [0; SIMPLE_INFO_SET_LENGTH];
            array[TRICK_STARTER] = state.contract.declarer() - state.side;
            array[TRICK_COMPLETION] = state.contract.current_trick().count_cards();
            (array[LEFT_CARD], array[LEFT_CARD+1]) = match state.contract.current_trick()[state.side]{
                None => (0, 0),
                Some(c) => (c.suit().usize_index() as u8 + 1, c.figure().usize_index() as u8 + 1)
            };
            (array[LEFT_CARD], array[LEFT_CARD+1]) = match state.contract.current_trick()[state.side.next()]{
                None => (0, 0),
                Some(c) => (c.suit().usize_index() as u8 + 1, c.figure().usize_index() as u8 + 1)
            };
            (array[PARTNER_CARD], array[PARTNER_CARD+1]) = match state.contract.current_trick()[state.side.partner()]{
                None => (0, 0),
                Some(c) => (c.suit().usize_index() as u8 + 1, c.figure().usize_index() as u8 + 1)
            };
            (array[RIGHT_CARD], array[RIGHT_CARD+1]) = match state.contract.current_trick()[state.side.prev()]{
                None => (0, 0),
                Some(c) => (c.suit().usize_index() as u8 + 1, c.figure().usize_index() as u8 + 1)
            };

            array[BID_OFFSET] = match state.contract.contract_spec().bid().trump(){
                    TrumpGen::Colored(s) => s.usize_index() as u8 + 1,
                    TrumpGen::NoTrump => 0
            };
            array[BID_OFFSET+1] = state.contract.contract_spec().bid().number();
            array[BID_OFFSET+2] = match state.contract.contract_spec().doubling(){
                Doubling::None => 0,
                Doubling::Double => 1,
                Doubling::Redouble => 2
            };
            array[PLAY_AS_DUMMY] = match state.contract.current_side() == state.contract.dummy(){
                true => 1,
                false => 0
            };
            for card in STANDARD_DECK_CDHS{
                if state.hand.contains(&card){
                   array[card.usize_index()] = SURE; //sure
                   /*not needed
                   for i in 1..=3{
                       array[(i*DECK_SIZE)+card.position()] = QUInt8::from(0);
                   }*/
                } else if !state.contract.used_cards().is_registered(&card){
                    match state.dummy_hand{
                        None => {
                            //dummy's hand not shown yet
                            for i in 1..=3{
                                array[(i*DECK_SIZE) + card.usize_index()] = ONE_IN_THREE;
                            }
                        }
                        Some(dhand) => {
                            if dhand.contains(&card){
                                array[(DECK_SIZE*dummy_offset) + card.usize_index()] = SURE;
                            } else {
                                //this is tricky
                                if state.contract.suits_exhausted().is_registered(&(unknown_side_1, card.suit())){
                                    array[(DECK_SIZE*unknown_offset_2) + card.usize_index()] = SURE;
                                }
                                else if state.contract.suits_exhausted().is_registered(&(unknown_side_2, card.suit())){
                                    array[(DECK_SIZE*unknown_offset_1) + card.usize_index()] = SURE;
                                }
                                else{
                                    array[(DECK_SIZE*unknown_offset_1) + card.usize_index()] = ONE_IN_TWO;
                                    array[(DECK_SIZE*unknown_offset_2) + card.usize_index()] = ONE_IN_TWO;
                                }

                            }
                        }
                    }
                    //card was not yet played
                } else {
                    //card was played before
                }
            }
            array


        }
    }
    impl From<&ContractAgentInfoSetSimple> for [f32;SIMPLE_INFO_SET_LENGTH] {
        fn from(state: &ContractAgentInfoSetSimple) -> Self {
            let dummy_offset = (state.contract.dummy() - state.side) as usize;

            let unknown_side_1 = state.side.first_unknown_side(state.contract.declarer());
            let unknown_side_2 = state.side.second_unknown_side(state.contract.declarer());
            let unknown_offset_1 = (unknown_side_1 - state.side) as usize;
            let unknown_offset_2 = (unknown_side_2 - state.side) as usize;

            //let mut array:[QUInt8;SIMPLE_INFO_SET_LENGTH] = [QUInt8::zero();SIMPLE_INFO_SET_LENGTH];
            let mut array = [0.0; SIMPLE_INFO_SET_LENGTH];
            array[TRICK_STARTER] = (state.contract.declarer() - state.side) as f32;
            array[TRICK_COMPLETION] = (state.contract.current_trick().count_cards()) as f32;
            (array[LEFT_CARD], array[LEFT_CARD+1]) = match state.contract.current_trick()[state.side]{
                None => (0.0, 0.0),
                Some(c) => (c.suit().usize_index() as f32 + 1.0, c.figure().usize_index() as f32 + 1.0)
            };
            (array[LEFT_CARD], array[LEFT_CARD+1]) = match state.contract.current_trick()[state.side.next()]{
                None => (0.0, 0.0),
                Some(c) => (c.suit().usize_index() as f32 + 1.0, c.figure().usize_index() as f32 + 1.0)
            };
            (array[PARTNER_CARD], array[PARTNER_CARD+1]) = match state.contract.current_trick()[state.side.partner()]{
                None => (0.0, 0.0),
                Some(c) => (c.suit().usize_index() as f32 + 1.0, c.figure().usize_index() as f32 + 1.0)
            };
            (array[RIGHT_CARD], array[RIGHT_CARD+1]) = match state.contract.current_trick()[state.side.prev()]{
                None => (0.0, 0.0),
                Some(c) => (c.suit().usize_index() as f32 + 1.0, c.figure().usize_index() as f32 + 1.0)
            };

            array[BID_OFFSET] = match state.contract.contract_spec().bid().trump(){
                    TrumpGen::Colored(s) => s.usize_index() as f32 + 1.0,
                    TrumpGen::NoTrump => 0.0
            };
            array[BID_OFFSET+1] = state.contract.contract_spec().bid().number() as f32;
            array[BID_OFFSET+2] = match state.contract.contract_spec().doubling(){
                Doubling::None => 0.0,
                Doubling::Double => 1.0,
                Doubling::Redouble => 2.0
            };
            array[PLAY_AS_DUMMY] = match state.contract.current_side() == state.contract.dummy(){
                true => 1.0,
                false => 0.0
            };
            for card in STANDARD_DECK_CDHS{
                if state.hand.contains(&card){
                   array[card.usize_index()] = 1.0; //sure
                   /*not needed
                   for i in 1..=3{
                       array[(i*DECK_SIZE)+card.position()] = QUInt8::from(0);
                   }*/
                } else if !state.contract.used_cards().is_registered(&card){
                    match state.dummy_hand{
                        None => {
                            //dummy's hand not shown yet
                            for i in 1..=3{
                                array[(i*DECK_SIZE) + card.usize_index()] = 1.0/3.0;
                            }
                        }
                        Some(dhand) => {
                            if dhand.contains(&card){
                                array[(DECK_SIZE*dummy_offset) + card.usize_index()] = 1.0;
                            } else {
                                //this is tricky
                                if state.contract.suits_exhausted().is_registered(&(unknown_side_1, card.suit())){
                                    array[(DECK_SIZE*unknown_offset_2) + card.usize_index()] = 1.0;
                                }
                                else if state.contract.suits_exhausted().is_registered(&(unknown_side_2, card.suit())){
                                    array[(DECK_SIZE*unknown_offset_1) + card.usize_index()] = 1.0;
                                }
                                else{
                                    array[(DECK_SIZE*unknown_offset_1) + card.usize_index()] = 0.5;
                                    array[(DECK_SIZE*unknown_offset_2) + card.usize_index()] = 0.5;
                                }

                            }
                        }
                    }
                    //card was not yet played
                } else {
                    //card was played before
                }
            }
            array


        }
    }


    impl From<&ContractAgentInfoSetSimple> for amfiteatr_rl::tch::Tensor{
        fn from(value: &ContractAgentInfoSetSimple) -> Self {
            amfiteatr_rl::tch::Tensor::from_slice(&Into::<[f32;SIMPLE_INFO_SET_LENGTH]>::into(value))
        }
    }

}

impl From<(Side, ContractParameters, DescriptionDeckDeal,)> for ContractAgentInfoSetSimple{

    fn from(base: (Side, ContractParameters, DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

        let contract = Contract::new(params);
        Self::new(side, descript.cards[&side] , contract, None)
    }
}
impl From<(&Side, &ContractParameters, &DescriptionDeckDeal)> for ContractAgentInfoSetSimple{
    fn from(base: (&Side, &ContractParameters, &DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

        let contract = Contract::new(params.clone());
        Self::new(*side, descript.cards[side] , contract, None)
    }
}

impl Renew<ContractDP, (&Side, &ContractParameters, &DescriptionDeckDeal)> for ContractAgentInfoSetSimple{
    fn renew_from(&mut self, base: (&Side, &ContractParameters, &DescriptionDeckDeal)) -> Result<(), AmfiteatrError<ContractDP>> {
        let (side, params, descript) = base;

        let contract = Contract::new(params.clone());
        self.dummy_hand = None;
        self.contract = contract;
        self.side = *side;
        self.hand = descript.cards[side];
        Ok(())
    }
}

impl From<(&Side, &ContractGameDescription)> for ContractAgentInfoSetSimple{
    fn from(base: (&Side, &ContractGameDescription)) -> Self {
        let (side, description) = base;

        let contract = Contract::new(description.parameters().clone());
        Self::new(*side, description.cards()[side] , contract, None)
    }
}

impl Renew<ContractDP, (&Side, &ContractGameDescription)> for ContractAgentInfoSetSimple{
    fn renew_from(&mut self, base: (&Side, &ContractGameDescription)) -> Result<(), AmfiteatrError<ContractDP>> {
        let (side, description) = base;

        let contract = Contract::new(description.parameters().clone());
        self.dummy_hand = None;
        self.contract = contract;
        self.side = *side;
        self.hand = description.cards()[side];
        Ok(())
    }
}


impl ContractInfoSet for ContractAgentInfoSetSimple{
    fn side(&self) -> Side {
        self.side
    }

    fn contract_data(&self) -> &Contract {
        &self.contract
    }

    fn dummy_hand(&self) -> Option<&CardSet> {
        self.dummy_hand.as_ref()
    }

    fn hand(&self) -> &CardSet {
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
            return match self.hand.contains(card){
                true => 1.0,
                false => 0.0
            };
        } else {
            if self.hand.contains(card){
                return 0.0; //this player has, other cant
            }
            if let Some(d) = self.dummy_hand{
                //dummy shown
                if side == self.contract.dummy(){
                    //check dummys card
                    match d.contains(card){
                        true => 1.0,
                        false => 0.0
                    }
                } else {
                    match d.contains(card){
                        true => 0.0,
                        false => 0.5
                    }
                }

                //neither self nor dummy, card not marked as used
                //0.5

            } else {
                //dummy not shown, anyone except this can have
                return 1.0/3.0;
            }
        }

    }
}


#[cfg(test)]
mod tests{
    use std::str::FromStr;
    use karty::cards::{*};
    use karty::hand::CardSet;
    use karty::suits::Suit::Hearts;
    use amfiteatr_core::agent::InformationSet;
    use crate::bidding::Bid;
    use crate::cards::trump::TrumpGen;
    use crate::contract::{Contract, ContractParametersGen};
    use crate::player::side::Side::{*};
    use crate::amfiteatr::state::{ContractAgentInfoSetSimple, ContractStateUpdate};
    use crate::amfiteatr::state::ContractAction::{PlaceCard, ShowHand};

    #[cfg(feature = "torch")]
    #[test]
    fn convert_simple_info_set_to_bytes(){
        let contract = Contract::new(
            ContractParametersGen::new(
                East,
                Bid::init(TrumpGen::Colored(Hearts), 2).unwrap() ));
        let mut info_set = ContractAgentInfoSetSimple::new(North,
                                                       CardSet::from_str("AT86.KJT93.4T.2A").unwrap(),
                                                       contract, None);

        let state_as_vec:[u8; 222] = (&info_set).into();
        assert_eq!(Vec::from(state_as_vec),
                   vec![120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120,//north
                        0, 0, 120, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0,
                        0, 120, 0, 0, 0, 0, 0, 120, 120, 120, 0, 120, 0,
                        0, 0, 0, 0, 120, 0, 120, 0, 120, 0, 0, 0, 120,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//east (declarer)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 40,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//south (partner)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 40,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//west (dummy)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 40,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0

                   ]
        );

        info_set.update(ContractStateUpdate::new(South, PlaceCard(ACE_DIAMONDS))).unwrap();
        let state_as_vec:[u8;222] = (&info_set).into();
        assert_eq!(Vec::from(state_as_vec),
                   vec![120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120,//north
                        0, 0, 120, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0,
                        0, 120, 0, 0, 0, 0, 0, 120, 120, 120, 0, 120, 0,
                        0, 0, 0, 0, 120, 0, 120, 0, 120, 0, 0, 0, 120,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//east (declarer)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 0,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//south (partner)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 0,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//west (dummy)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 0,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        1, 1, 0, 0, 0, 0, 2, 13, 0, 0 , 3, 2, 0, 1

                   ]
        );
        //AT86.KJT93.4T.2A
        info_set.update(ContractStateUpdate::new(West,
                                                 ShowHand(CardSet::from_str("QJ3.8764.A95.T96").unwrap()))).unwrap();
        info_set.update(ContractStateUpdate::new(West,
                                                 PlaceCard(FIVE_DIAMONDS))).unwrap();

        let state_as_vec:[u8;222] = (&info_set).into();
        assert_eq!(Vec::from(state_as_vec),
                   vec![120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120,//north
                        0, 0, 120, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0,
                        0, 120, 0, 0, 0, 0, 0, 120, 120, 120, 0, 120, 0,
                        0, 0, 0, 0, 120, 0, 120, 0, 120, 0, 0, 0, 120,
                        0, 60, 60, 60, 0, 60, 60, 0, 0, 60, 60, 60, 0,//east (declarer)
                        60, 60, 0, 0, 60, 60, 60, 0, 0, 60, 60, 60, 0,
                        60, 0, 0, 60, 0, 0, 0, 0, 0, 0, 60, 0, 60,
                        60, 0, 60, 60, 0, 60, 0, 60, 0, 0, 0, 60, 0,
                        0, 60, 60, 60, 0, 60, 60, 0, 0, 60, 60, 60, 0,//south (partner)
                        60, 60, 0, 0, 60, 60, 60, 0, 0, 60, 60, 60, 0,
                        60, 0, 0, 60, 0, 0, 0, 0, 0, 0, 60, 0, 60,
                        60, 0, 60, 60, 0, 60, 0, 60, 0, 0, 0, 60, 0,
                        0, 0, 0, 0, 120, 0, 0, 120, 120, 0, 0, 0, 0,//west (dummy)
                        0, 0, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0, 0,
                        0, 0, 120, 0, 120, 120, 120, 0, 0, 0, 0, 0, 0,
                        0, 120, 0, 0, 0, 0, 0, 0, 0, 120, 120, 0, 0,
                        1, 2, 0, 0, 0, 0, 2, 13, 2, 4 , 3, 2, 0, 0

                   ]
        );




    }
}

