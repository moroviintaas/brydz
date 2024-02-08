use brydz_core::karty::cards::Card;
use log::debug;
use smallvec::SmallVec;
use brydz_core::contract::{Contract, ContractMechanics};
use brydz_core::error::{BridgeCoreError, BridgeCoreErrorGen};
use brydz_core::karty::hand::CardSet;
use brydz_core::player::axis::Axis;
use brydz_core::player::axis::Axis::{NorthSouth, EastWest};
use brydz_core::player::side::Side;
use crate::actions::ActionOptimiser;
use crate::error::DoubleDummyError;
use crate::explore::{ExplorerGameState, ExplorerStateUpdate};
use crate::hash::NodeStoreTrait;
use crate::node::TrickNode;
use crate::actions::CardPackVec;
use crate::explore::ExploreOutput::Number;
use crate::explore::track::{CardPackResult, TrackStep};

#[derive(Debug, Clone)]
pub struct BinaryExplorer<G: ActionOptimiser, A: NodeStoreTrait>{
    game_state: ExplorerGameState<G>,
    north_south_target: u8,
    #[allow(dead_code)]
    node_store: A

}

impl<G: ActionOptimiser, A:NodeStoreTrait> BinaryExplorer<G, A>{
    pub fn new_checked(contract: Contract, initial_node: TrickNode, north_south_target: u8) -> Result<Self, BridgeCoreError>{
        Ok(Self{game_state: ExplorerGameState::new_checked(contract, initial_node)?,
            node_store: A::default(), north_south_target
        })
    }
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
    /// use brydz_dd::explore::{BinaryExplorer, ExploreOutput, Explorer};
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
    /// let mut explorer = BinaryExplorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract.clone(), node.clone(), 1).unwrap();
    ///
    /// assert_eq!(explorer.explore_actions(), Ok(true));
    /// let mut explorer2 = BinaryExplorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract.clone(), node.clone(), 2).unwrap();
    /// assert_eq!(explorer2.explore_actions(), Ok(false));
    /// ```
    pub fn explore_actions(&mut self) -> Result<bool, DoubleDummyError>{
        let next_actions = self.state().available_actions();
        debug!("Exploring actions for side: {:?}.\nTricks completed: {:?}.\nCurrent trick: {:#}. Available action groups: {:#}",
            self.current_side(), self.state().contract().count_completed_tricks(),
            self.state().contract().current_trick(), CardPackVec(next_actions.clone().into_iter().collect()));
        if next_actions.and(SmallVec::is_empty){
            debug!("No next actions: current side: {:?}, tricks completed: {:?}, cards in current trick: {:?}", self.current_side(), self.state().contract().count_completed_tricks(), self.state().contract().current_trick().count_cards());
            return Ok((self.state().contract().total_tricks_taken_axis(Axis::NorthSouth) as u8) >= self.north_south_target)
        }
        if self.game_state.contract().current_trick().is_empty(){
            let potential = <CardSet as Into<u64>>::into(self.game_state.actual_node().hands()[&self.current_side()]).count_ones() as u8;
            let actual = self.game_state.contract().total_tricks_taken_axis(NorthSouth);
            if actual as u8 >= self.north_south_target{
                return Ok(true)
            }
            if actual as u8 + potential < self.north_south_target{
                return Ok(false)
            }

        }
        match self.current_side().axis() {
            NorthSouth => {
                for card_pack in next_actions {
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    let v = self.explore_actions()?;
                    self.update(ExplorerStateUpdate::Undo)?;
                    if v {
                        return Ok(true);
                    }
                }
                Ok(false)
            },
            EastWest => {
                for card_pack in next_actions {
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    let v = self.explore_actions()?;
                    self.update(ExplorerStateUpdate::Undo)?;
                    if !v {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
        }

    }
    pub fn hint(&mut self) -> Result<TrackStep, DoubleDummyError>{
        let next_actions = self.state().available_actions();
        debug!("Exploring actions for side: {:?}.\nTricks completed: {:?}.\nCurrent trick: {:#}. Available action groups: {:#}",
            self.current_side(), self.state().contract().count_completed_tricks(),
            self.state().contract().current_trick(), CardPackVec(next_actions.clone().into_iter().collect()));
        if next_actions.and(SmallVec::is_empty){
            debug!("No next actions: current side: {:?}, tricks completed: {:?}, cards in current trick: {:?}", self.current_side(), self.state().contract().count_completed_tricks(), self.state().contract().current_trick().count_cards());
            todo!()
        }
        let mut track_step = TrackStep::new(self.current_side());
        match self.current_side().axis(){
            Axis::NorthSouth => {
                //maximising
                for card_pack in next_actions{
                    //debug!("{:?} placing card {:#}", self.current_side(),card_pack.lowest_card() );
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    let tmp_value = self.explore_actions()?;
                    if tmp_value{
                        track_step.push_and_hint(CardPackResult::new(card_pack, Number(1)));
                    } else {
                        track_step.push(CardPackResult::new(card_pack, Number(0)));
                    }


                    self.update(ExplorerStateUpdate::Undo)?;
                    /*if value > self.north_south_max{
                        break;
                    }*/
                    //self.north_south_min = max(self.north_south_min, value)
                }
                Ok(track_step)

            },
            Axis::EastWest => {
                //minimising
                for card_pack in next_actions{
                    self.update(ExplorerStateUpdate::PlaceCard(card_pack.lowest_card()))?;
                    let tmp_value = self.explore_actions()?;
                    //value = min(value, tmp_value);
                    //track_step.push(CardPackResult::new(card_pack, tmp_value));

                    if !tmp_value{
                        track_step.push_and_hint(CardPackResult::new(card_pack, Number(0)));
                    } else {
                        track_step.push(CardPackResult::new(card_pack, Number(1)));
                    }

                    self.update(ExplorerStateUpdate::Undo)?;

                    /*if value < self.north_south_min{
                        break;
                    }*/
                }
                Ok(track_step)

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
impl<G1: ActionPrepare, G2: ActionPrepare, A: NodeStoreTrait> From<BinaryExplorer<G1,A>> for BinaryExplorer<G2, A>{
    fn from(value: BinaryExplorer<G1,A>) -> Self {
        let mut n = Self{ game_state: value.game_state, north_south_target: value.north_south_target, node_store: value.node_store }
        match n.state().contract().current_trick().is_empty(){
            n.st
        }
    }
} */
/*
impl<G: ActionOptimiser, A: NodeStoreTrait >  StatefulAgent<ContractProtocolSpec> for BinaryExplorer<G, A>{
    type State = ExplorerGameState<G>;
    //type StateUpdate = ExplorerStateUpdate;
    //type Spec = ContractProtocolSpec;

    fn update(&mut self, state_update: ContractStateUpdate) -> Result<(), BridgeCoreErrorGen<Card>> {
        self.game_state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.game_state
    }
}

 */