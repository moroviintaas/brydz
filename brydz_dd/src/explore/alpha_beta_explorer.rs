use std::cmp::{max, min};

use brydz_core::contract::{Contract, ContractMechanics};
use brydz_core::error::{BridgeCoreErrorGen};
use brydz_core::karty::cards::Card;
use brydz_core::player::axis::Axis;
use brydz_core::player::side::Side;
use log::debug;
use crate::error::{DoubleDummyError};
use crate::node::TrickNode;
use std::fmt::{Debug};
use std::sync::mpsc::channel;
use std::thread;
use smallvec::SmallVec;
use brydz_core::karty::hand::CardSet;
use brydz_core::player::axis::Axis::NorthSouth;
use crate::actions::ActionOptimiser;
use crate::explore::{ExploreOutput, ExplorerGameState, ExplorerStateUpdate};
use crate::explore::track::{CardPackResult, GameTrack, TrackStep};
use crate::actions::CardPackVec;
use crate::hash::{NodeStoreTrait};


#[derive(Debug, Clone)]
pub struct Explorer<G: ActionOptimiser, A: NodeStoreTrait > {
    game_state: ExplorerGameState<G>,
    //north_south_max: ExploreOutput,
    //north_south_min: ExploreOutput,
    //initial_north_south_max: ExploreOutput,
    //initial_north_south_min: ExploreOutput,
    node_store: A



}


impl<G: ActionOptimiser, A: NodeStoreTrait > Explorer<G, A>{
    /*fn check_current_trick(&self) -> Result<(), TrickError>{


    }*/

    pub fn new_checked(contract: Contract, initial_node: TrickNode) -> Result<Self, DoubleDummyError>{
        //let mut v = Vec::with_capacity(64);
        //v.push(initial_node);
        //let explorer = Self{contract,  node_stack: v, north_south_max: ns_max, north_south_min: ns_min, grouper};

        //explorer.check_valid().map(|()| explorer)
        Ok(Self{game_state: ExplorerGameState::new_checked(contract, initial_node)?, 
            node_store: A::default(),
        })
    }

    /*fn update_state(&mut self, update: ExplorerStateUpdate) -> Result<(), BridgeCoreError>{
        self.game_state.update(update)
    }*/

    /*pub fn reset_alpha_beta(&mut self){
        self.north_south_min = self.initial_north_south_min;
        self.north_south_max = self.initial_north_south_max;
    }*/

    pub fn current_side(&self) -> Side{
        self.state().contract().current_side()
    }


    /// ```
    /// use brydz_dd::{
    ///     node::{
    ///         TrickNode
    ///     },
    /// };
    /// use brydz_core::{
    ///     player::side::{SideMap, Side::*},
    ///     cards::trump::Trump,
    ///     bidding::Bid,
    ///     contract::{Contract, ContractParametersGen, ContractMechanics},
    ///     karty::{
    ///         suits::Suit::*,
    ///         cards::*,
    ///         hand::CardSet,
    ///         card_set
    ///     }
    /// };
    /// use brydz_dd::actions::DistinctCardGrouper;
    /// use brydz_dd::explore::{ExploreOutput, Explorer};
    /// use brydz_dd::hash::DummyNodeStore;
    ///
    /// let contract = Contract::new(
    /// ContractParametersGen::new(West, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    /// let hands = SideMap::new(
    ///     card_set![ACE_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS],
    ///     card_set!(ACE_HEARTS, KING_DIAMONDS, KING_HEARTS, JACK_DIAMONDS),
    ///     card_set![ACE_DIAMONDS, ACE_CLUBS, QUEEN_HEARTS, QUEEN_CLUBS],
    ///     card_set![KING_SPADES, QUEEN_DIAMONDS, JACK_SPADES, JACK_HEARTS ]);
    /// let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    /// let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    /// assert_eq!(explorer.explore_actions(ExploreOutput::MinusInfinity, ExploreOutput::Infinity), Ok(1.into()));
    /// ```
    pub fn explore_actions(&mut self, mut north_south_min: ExploreOutput, mut north_south_max: ExploreOutput) -> Result<ExploreOutput, DoubleDummyError>{
        let next_actions = self.state().available_actions();
        debug!("Exploring actions for side: {:?}.\nTricks completed: {:?}.\nCurrent trick: {:#}. Available action groups: {:#}",
            self.current_side(), self.state().contract().count_completed_tricks(),
            self.state().contract().current_trick(), CardPackVec(next_actions.clone().into_iter().collect()));
            //self.state().contract().current_trick(), next_actions.clone());
        if next_actions.and(SmallVec::is_empty){
            debug!("No next actions: current side: {:?}, tricks completed: {:?}, cards in current trick: {:?}", self.current_side(), self.state().contract().count_completed_tricks(), self.state().contract().current_trick().count_cards());
            return Ok((self.state().contract().total_tricks_taken_axis(Axis::NorthSouth) as u8).into())
        }

        if self.game_state.contract().current_trick().is_empty(){
            if let Some(hit_node) = self.node_store.get_value(self.game_state.actual_node()){
                return Ok(ExploreOutput::Number(hit_node + self.state().contract().total_tricks_taken_axis(NorthSouth) as u8))
            }

            if let ExploreOutput::Number(min) = north_south_min{
                let potential = <CardSet as Into<u64>>::into(self.game_state.actual_node().hands()[&self.current_side()]).count_ones() as u8
                + self.game_state.contract().total_tricks_taken_axis(NorthSouth) as u8;
                if potential < min{
                    return Ok(ExploreOutput::Number(potential))
                }
                //return if north south is doomed not to achieve his/her minimum
            }

            if let ExploreOutput::Number(max) = north_south_max{
                let current = self.game_state.contract().total_tricks_taken_axis(NorthSouth) as u8;
                if  current> max{
                    return Ok(ExploreOutput::Number(current))
                }
                //return uf north south scored over their maximum
            }
        }

        match self.current_side().axis(){
            Axis::NorthSouth => {
                //maximising
                let mut value = ExploreOutput::MinusInfinity;
                for card_pack in next_actions{
                    //debug!("{:?} placing card {:#}", self.current_side(),card_pack.lowest_card() );
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    value = max(value, self.explore_actions(north_south_min, north_south_max)?);
                    self.update(ExplorerStateUpdate::Undo)?;
                    if value > north_south_max{
                        break;
                    }
                    north_south_min = max(north_south_min, value)
                }
                if self.game_state.contract().current_trick().is_empty(){
                    if let ExploreOutput::Number(v) = value{
                        self.node_store.store_value(self.game_state.actual_node(),
                    v - self.game_state.contract().total_tricks_taken_axis(NorthSouth) as u8);
                    }
                }
                Ok(value)

            },
            Axis::EastWest => {
                //minimising
                let mut value = ExploreOutput::Infinity;
                for card_pack in next_actions{
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    value = min(value, self.explore_actions(north_south_min, north_south_max)?);
                    self.update(ExplorerStateUpdate::Undo)?;
                    if value < north_south_min{
                        break;
                    }
                    north_south_max = min(north_south_max, value)
                }
                if self.game_state.contract().current_trick().is_empty(){
                    if let ExploreOutput::Number(v) = value{
                        self.node_store.store_value(self.game_state.actual_node(),
                    v - self.game_state.contract().total_tricks_taken_axis(NorthSouth) as u8);
                    }
                }
                Ok(value)

            }
        }
    }

    pub fn hint(&mut self) -> Result<TrackStep, DoubleDummyError>{
        //self.reset_alpha_beta();
        let next_actions = self.state().available_actions();
        debug!("Exploring actions for side: {:?}.\nTricks completed: {:?}.\nCurrent trick: {:#}. Available action groups: {:?}",
            self.current_side(), self.state().contract().count_completed_tricks(),
            self.state().contract().current_trick(), next_actions);
        if next_actions.and(SmallVec::is_empty){
            debug!("No next actions: current side: {:?}, tricks completed: {:?}, cards in current trick: {:?}", self.current_side(), self.state().contract().count_completed_tricks(), self.state().contract().current_trick().count_cards());
            //return Ok((self.state().contract().total_tricks_taken_axis(Axis::NorthSouth) as u8).into())
            todo!()
        }
        let mut track_step = TrackStep::new(self.current_side());
        

        match self.current_side().axis(){
            Axis::NorthSouth => {
                //maximising
                let mut north_south_min = ExploreOutput::MinusInfinity;
                let mut value = ExploreOutput::MinusInfinity;
                for card_pack in next_actions{
                    //debug!("{:?} placing card {:#}", self.current_side(),card_pack.lowest_card() );
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    let tmp_value = self.explore_actions(north_south_min, ExploreOutput::Infinity)?;
                    if tmp_value > value{
                        track_step.push_and_hint(CardPackResult::new(card_pack, tmp_value));
                        value = tmp_value
                    } else {
                        track_step.push(CardPackResult::new(card_pack, tmp_value));
                    }


                    self.update(ExplorerStateUpdate::Undo)?;
                    /*if value > self.north_south_max{
                        break;
                    }*/
                    north_south_min = max(north_south_min, value)
                }
                Ok(track_step)

            },
            Axis::EastWest => {
                //minimising
                let mut north_south_max = ExploreOutput::Infinity;
                let mut value = ExploreOutput::Infinity;
                for card_pack in next_actions{
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    let tmp_value = self.explore_actions(ExploreOutput::MinusInfinity, north_south_max)?;
                    //value = min(value, tmp_value);
                    //track_step.push(CardPackResult::new(card_pack, tmp_value));

                    if tmp_value < value{
                        track_step.push_and_hint(CardPackResult::new(card_pack, tmp_value));
                        value = tmp_value
                    } else {
                        track_step.push(CardPackResult::new(card_pack, tmp_value));
                    }

                    self.update(ExplorerStateUpdate::Undo)?;

                    /*if value < self.north_south_min{
                        break;
                    }*/
                    north_south_max = min(north_south_max, value)
                }
                Ok(track_step)

            }
        }

    }
    pub fn hint_concurrent(&mut self) -> Result<TrackStep, DoubleDummyError>
    where Self: Send, G: 'static, A: Clone + 'static {
        //self.reset_alpha_beta();
        let next_actions = self.state().available_actions();
        debug!("Exploring actions for side: {:?}.\nTricks completed: {:?}.\nCurrent trick: {:#}. Available action groups: {:#}",
            self.current_side(), self.state().contract().count_completed_tricks(),
            self.state().contract().current_trick(), CardPackVec(next_actions.clone().into_iter().collect()));
        if next_actions.and(SmallVec::is_empty){
            debug!("No next actions: current side: {:?}, tricks completed: {:?}, cards in current trick: {:?}", self.current_side(), self.state().contract().count_completed_tricks(), self.state().contract().current_trick().count_cards());
            //return Ok((self.state().contract().total_tricks_taken_axis(Axis::NorthSouth) as u8).into())
            todo!()
        }
        let mut track_step = TrackStep::new(self.current_side());
        let mut thread_control = Vec::with_capacity(16);

        for card_pack in next_actions{
            let mut explorer_clone = self.clone();
            let (tx, rx) = channel();
            let card = card_pack.lowest_card();
            let handler = thread::spawn(move || {
                explorer_clone.update(ExplorerStateUpdate::PlaceCard(card)).unwrap();
                let tmp_value = explorer_clone.explore_actions(ExploreOutput::MinusInfinity, ExploreOutput::Infinity).unwrap();
                tx.send(tmp_value).unwrap();


            });
            thread_control.push((handler, rx, card_pack));

        }
        /*for control in thread_control.into_iter(){
        }*/
        /*
        let values: Vec<CardPackResult> = thread_control.into_iter().map(|(_, rx, card_pack)|
           //(rx.recv().unwrap(), card_pack)
           CardPackResult::new(card_pack, rx.recv().unwrap())
        ).collect();
        for (handle,_, _) in thread_control.into_iter(){
            handle.join().unwrap()
        }*/

        let  values: Vec<CardPackResult> = thread_control.into_iter().map(|(handle, rx, card_pack)|{
            let result = CardPackResult::new(card_pack, rx.recv().unwrap());
            handle.join().unwrap();
            result
        }).collect();

        match self.current_side().axis(){
            Axis::NorthSouth => {
                let mut value = ExploreOutput::MinusInfinity;
                for res in values{
                    if res.raw_value() > value{
                        value = res.raw_value();
                        track_step.push_and_hint(res);
                        //value = res.raw_value()
                    }
                    else {
                        track_step.push(res);
                    }
                }
                Ok(track_step)
            }
            Axis::EastWest => {
                let mut value = ExploreOutput::Infinity;
                for res in values{
                    if res.raw_value() < value{
                        value = res.raw_value();
                        track_step.push_and_hint(res);
                        //value = res.raw_value()
                    }
                    else {
                        track_step.push(res);
                    }
                }
                Ok(track_step)
            }
        }



        /*
        match self.current_side().axis(){
            Axis::NorthSouth => {
                //maximising
                let mut value = ExploreOutput::MinusInfinity;
                for card_pack in next_actions.into_iter(){
                    //debug!("{:?} placing card {:#}", self.current_side(),card_pack.lowest_card() );
                    self.update_state(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    let tmp_value = self.explore_actions()?;
                    if tmp_value > value{
                        track_step.push_and_hint(CardPackResult::new(card_pack, tmp_value));
                        value = tmp_value
                    } else {
                        track_step.push(CardPackResult::new(card_pack, tmp_value));
                    }


                    self.update_state(ExplorerStateUpdate::Undo)?;
                    /*if value > self.north_south_max{
                        break;
                    }*/
                    self.north_south_min = max(self.north_south_min, value)
                }
                Ok(track_step)

            },
            Axis::EastWest => {
                //minimising
                let mut value = ExploreOutput::Infinity;
                for card_pack in next_actions.into_iter(){
                    self.update_state(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    let tmp_value = self.explore_actions()?;
                    //value = min(value, tmp_value);
                    //track_step.push(CardPackResult::new(card_pack, tmp_value));

                    if tmp_value < value{
                        track_step.push_and_hint(CardPackResult::new(card_pack, tmp_value));
                        value = tmp_value
                    } else {
                        track_step.push(CardPackResult::new(card_pack, tmp_value));
                    }

                    self.update_state(ExplorerStateUpdate::Undo)?;

                    /*if value < self.north_south_min{
                        break;
                    }*/
                    self.north_south_max = min(self.north_south_max, value)
                }
                Ok(track_step)

            }
        }*/

    }

    pub fn explore_and_track_actions(&mut self, mut north_south_min: ExploreOutput, mut north_south_max: ExploreOutput) -> Result<GameTrack, DoubleDummyError>{


        let next_actions = self.state().available_actions();
        debug!("Exploring actions for side: {:?}.\nTricks completed: {:?}.\nCurrent trick: {:#}. Available action groups: {:#}",
            self.current_side(), self.state().contract().count_completed_tricks(),
            self.state().contract().current_trick(), CardPackVec(next_actions.clone().into_iter().collect()));
        if next_actions.and(SmallVec::is_empty){
            debug!("No next actions: current side: {:?}, tricks completed: {:?}, cards in current trick: {:?}", self.current_side(), self.state().contract().count_completed_tricks(), self.state().contract().current_trick().count_cards());
            return Ok(
                GameTrack::new((self.state().contract().total_tricks_taken_axis(Axis::NorthSouth) as u8).into()))
        }
        let mut track_step = TrackStep::new(self.current_side());
        match self.current_side().axis(){
            Axis::NorthSouth => {
                //maximising
                let mut value = GameTrack::new(ExploreOutput::MinusInfinity);

                for card_pack in next_actions{
                    //debug!("{:?} placing card {:#}", self.current_side(),card_pack.lowest_card() );
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    //track_step.push(card_pack);
                    let tmp_value = self.explore_and_track_actions(north_south_min, north_south_max)?;


                    if tmp_value > value{
                        track_step.push_and_hint(CardPackResult::new(card_pack, tmp_value.leaf_value()));
                        value = tmp_value;

                    } else {
                        track_step.push(CardPackResult::new(card_pack, tmp_value.leaf_value()));
                    }
                    //value = max(value, );
                    self.update(ExplorerStateUpdate::Undo)?;
                    if value.leaf_value() > north_south_max{
                        break;
                    }
                    north_south_min = max(north_south_min, value.leaf_value())
                }
                value.push(track_step);
                Ok(value)

            },
            Axis::EastWest => {
                //minimising
                let mut value = GameTrack::new(ExploreOutput::Infinity);

                for card_pack in next_actions{
                    //debug!("{:?} placing card {:#}", self.current_side(),card_pack.lowest_card() );
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    //track_step.push(card_pack);
                    let tmp_value = self.explore_and_track_actions(north_south_min, north_south_max)?;


                    if tmp_value < value{
                        track_step.push_and_hint(CardPackResult::new(card_pack, tmp_value.leaf_value()));
                        value = tmp_value;
                    } else {
                        track_step.push(CardPackResult::new(card_pack, tmp_value.leaf_value()));
                    }
                    //value = max(value, );
                    self.update(ExplorerStateUpdate::Undo)?;
                    if value.leaf_value() < north_south_min{
                        break;
                    }
                    north_south_max = min(north_south_max, value.leaf_value())
                }
                value.push(track_step);
                Ok(value)

            }
        }
    }

    pub fn update(&mut self, state_update: ExplorerStateUpdate) -> Result<(), BridgeCoreErrorGen<Card>> {
        self.game_state.update(state_update)
    }

    pub fn state(&self) -> &ExplorerGameState<G> {
        &self.game_state
    }

}
/*
impl<G: ActionOptimiser, A: NodeStoreTrait >  StatefulAgent<ContractProtocolSpec> for Explorer<G, A>{
    type State = ExplorerGameState<G>;


}

*/



#[cfg(test)]
mod tests{
    use brydz_core::bidding::Bid;
    use brydz_core::cards::trump::TrumpGen;
    use brydz_core::contract::{Contract, ContractMechanics, ContractParametersGen};
    use brydz_core::deal::fair_bridge_partial_deal;
    use brydz_core::error::{BridgeCoreError};
    use brydz_core::error::ContractErrorGen::CurrentSidePresume;
    use brydz_core::karty::cards::{Card, Card2SymTrait};
    use brydz_core::karty::error::CardSetError;
    use brydz_core::karty::figures::{Ace, King, Queen};
    use brydz_core::karty::hand::{HandTrait, CardSet};
    use brydz_core::karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    use brydz_core::player::side::Side::{East, North, South, West};
    use crate::actions::DistinctCardGrouper;
    use crate::explore::{Explorer};
    use crate::explore::ExplorerStateUpdate::PlaceCard;
    use crate::hash::DummyNodeStore;
    use crate::node::TrickNode;

    #[test]
    fn explorer_validation(){
        let card_supply: Vec<Card> = Card::card_subset(
            vec![Ace, King, Queen],
            vec![Spades, Hearts, Diamonds, Clubs]).collect();
        let hands = fair_bridge_partial_deal::<CardSet>(card_supply, North);
        let cards_north = hands[&North].to_vec();
        let cards_east = hands[&East].to_vec();

        let contract = Contract::new(
            ContractParametersGen::new(West,
                                       Bid::init(TrumpGen::Colored(Spades), 1).unwrap()));
        let node = TrickNode::new_checked(hands, North).unwrap();
        let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
        /*explorer.place_card(&cards_north[0]).unwrap();
        explorer.check_valid().unwrap();
        explorer.place_card(&cards_east[0]).unwrap();*/
        explorer.update(PlaceCard(cards_north[0])).unwrap();
        explorer.state().check_valid().unwrap();
        explorer.update(PlaceCard(cards_east[0])).unwrap();

        assert_eq!(explorer.state().contract().current_side(), South);
        assert!(explorer.state().contract().current_trick().contains(&cards_north[0]));
    }

    #[test]
    fn explorer_validation_bad_lengths(){
        let mut card_supply: Vec<Card> = Card::card_subset(
            vec![Ace, King, Queen],
            vec![Spades, Hearts, Diamonds, Clubs]).collect();
        card_supply.pop();
        let hands = fair_bridge_partial_deal::<CardSet>(card_supply, North);

        let result_node = TrickNode::new_checked(hands, North);
        assert_eq!(result_node, Err(CardSetError::DifferentLengths(2, 3)));

    }
    #[test]
    fn explorer_validation_mismatch_side(){
        let card_supply: Vec<Card> = Card::card_subset(
            vec![Ace, King, Queen],
            vec![Spades, Hearts, Diamonds, Clubs]).collect();
        let hands = fair_bridge_partial_deal::<CardSet>(card_supply, North);
        //let mut cards_north = hands[&North].to_vec();
        //let mut cards_east = hands[&East].to_vec();

        let contract = Contract::new(
            ContractParametersGen::new(East,
                                       Bid::init(TrumpGen::Colored(Spades), 1).unwrap()));
        let node = TrickNode::new_checked(hands, North).unwrap();
        let result_explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node);
        assert_eq!(result_explorer.unwrap_err(), BridgeCoreError::Contract(CurrentSidePresume(South, North)).into());
    }
}
/*
 match self.current_side().axis(){
            Axis::NorthSouth => {
                //maximising
                let mut value = GameTrack::new(ExploreOutput::MinusInfinity);

                for card_pack in next_actions.into_iter(){
                    //debug!("{:?} placing card {:#}", self.current_side(),card_pack.lowest_card() );
                    self.update_state(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    //track_step.push()
                    let tmp_value = self.explore_and_track_actions()?;
                    if tmp_value > value{
                        value = tmp_value
                    }
                    //value = max(value, );
                    self.update_state(ExplorerStateUpdate::Undo)?;
                    if value.leaf_value() > self.north_south_max.into(){
                        break;
                    }
                    self.north_south_min = max(self.north_south_min, value.leaf_value().)
                }
                Ok(value)

            },
            Axis::EastWest => {
                //minimising
                /*let mut value = GameTrack::new(ExploreOutput::Infinity);
                for card_pack in next_actions.into_iter(){
                    self.update_state(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    value = min(value, self.explore_actions()?);
                    self.update_state(ExplorerStateUpdate::Undo)?;
                    if value < self.north_south_min{
                        break;
                    }
                    self.north_south_max = min(self.north_south_max, value)
                }
                Ok(value)

                 */

            }
        }
 */