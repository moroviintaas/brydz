use std::fmt::{Debug, Display, Formatter};
use brydz_core::contract::{Contract, ContractMechanics};
use brydz_core::error::{BridgeCoreError, ContractError, DistributionError, CardSetErrorGen};
use brydz_core::error::ContractErrorGen::{BadTrick, CurrentSidePresume};
use brydz_core::error::TrickErrorGen::DuplicateCard;
use brydz_core::karty::cards::Card;
use brydz_core::karty::set::{CardSet, CardSetStd};
use brydz_core::meta::{CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND,  CONTRACT_ACTION_STACK_SIZE_BOUND};
use brydz_core::player::side::{Side, SideMap, SIDES};
use brydz_core::player::side::Side::{East, North, South, West};
use log::debug;
use smallvec::SmallVec;
use brydz_core::karty::suits::SuitMap;
//use brydz_core::sztorm::re_export::state::StateUpdate;
use crate::actions::{CardPack, ActionOptimiser};
use crate::explore::ExplorerStateUpdate::PlaceCard;
use crate::node::TrickNode;

#[derive(Debug, Clone)]
pub struct ExplorerGameState<G: ActionOptimiser>{
    contract: Contract,
    //initial_node: TrickNode,
    node_stack: SmallVec<[TrickNode; CONTRACT_ACTION_STACK_SIZE_BOUND]>,
    //_grouper: PhantomData<G>,
    action_optimiser: G,
    current_analysis_depth: u8,
    //cached_trick_actions: SideMap<SuitMap<SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]>>>
    //cached_raw_actions: SideMap<SuitMap<SmallVec<[Card; CONTRACT_ACTION_SPACE_BOUND]>>>
}

impl<G: ActionOptimiser> ExplorerGameState<G>{
    pub fn new_checked(contract: Contract, initial_node: TrickNode) -> Result<Self, BridgeCoreError>{
        let mut v = SmallVec::new();
        v.push(initial_node);
        //let mut explorer_state = Self{contract, node_stack: v, _grouper: PhantomData::default(), current_analysis_depth:0, cached_raw_actions: SideMap::default() };
        let mut explorer_state = Self{contract, node_stack: v, current_analysis_depth:0, action_optimiser: G::default() };
        /*if explorer_state.contract.current_trick().is_empty(){
            explorer_state.generate_cached_action_variants()
        }*/
        match explorer_state.contract().current_trick().is_empty(){
            true =>{
                explorer_state.update_cache_new_trick()?;
            },
            false => {
                explorer_state.update_cache_partial_trick()?;
            }
        }


        explorer_state.check_valid().map(|()| explorer_state)

    }
    /*fn generate_cached_action_variants(&mut self){
        self.cached_raw_actions = SideMap::new_with_fn(|side|
            SuitMap::new_from_f(|suit|
            /*G::group_in_context(self.hands(), side, &suit)*/
            self.actual_node().hands()[&side].suit_iterator(&suit).collect()
        ))


    }*/


    pub fn convert_optimiser<NG: ActionOptimiser>(self) -> Result<ExplorerGameState<NG>, BridgeCoreError>{
        let n = ExplorerGameState::<NG>{
            contract: self.contract,
            node_stack: self.node_stack,
            action_optimiser: NG::default(),
            current_analysis_depth: self.current_analysis_depth,
        };
        let ap = match n.contract().current_trick().is_empty(){
                true => {
                    let mut opt = NG::default();
                    opt.cache_on_trick_new(&n)?;
                    opt
                }
                false => {
                    let mut opt = NG::default();
                    opt.cache_on_partial_trick(&n)?;
                    opt
                }
            };
        Ok(ExplorerGameState::<NG>{
            contract: n.contract,
            node_stack:  n.node_stack,
            action_optimiser: ap,
            current_analysis_depth: n.current_analysis_depth,
        })
    }

    pub fn hands(&self) -> &SideMap<CardSetStd>{
        self.node_stack.last().unwrap().hands()
    }
    pub fn hand(&self, side: Side) -> &CardSetStd {
        &self.hands()[&side]
    }

    fn check_current_side(&self) -> Result<Side, ContractError>{
        let node_side = self.node_stack.last().unwrap().current_side();
        /*match self.contract.current_side(){
            Some(contract_side) => match contract_side == node_side {
                true => Ok(contract_side),
                false => Err(CurrentSidePresume(contract_side, node_side))
            }
            None => Err(ContractError::ContractFull)
        }*/
        match self.contract.current_side() == node_side {
            true => Ok(node_side),
            false => Err(CurrentSidePresume(self.contract.current_side() ,
            node_side))
        }
    }

    //fn group_cards_in_hand(&self, suit: Suit) -> <Self as BridgeContractAgentState>::Aggregator {
        /*#[allow(dead_code)]
        fn group_cards_in_hand(&self, suit: Suit) -> VCardPack {    
        //G::group(&self.actual_node().hands()[&self.contract().current_side()], &suit)
        G::group_in_context(self.actual_node().hands(), self.contract().current_side(), &suit)
    }*/

    pub fn contract(&self) -> &Contract{
        &self.contract
    }

    pub fn check_valid(&self) -> Result<(), BridgeCoreError>{
        self.check_current_side()?;
        let all_hands = self.node_stack.last().unwrap().hands();
        let hands_sum = all_hands[&North].union(&all_hands[&East])
            .union(&all_hands[&South]).union(&all_hands[&West]);
        let trick = self.contract.current_trick();
        //let mut card_numbers = SideMap::<usize>::default();
        let mut card_numbers = all_hands.transform(|hand| hand.len());

        //We check if card that is in trick is not in one of hands
        for side in SIDES{
            if let Some(card) = trick[side]{
                if hands_sum.contains(&card){
                    return Err(BadTrick(DuplicateCard(card)).into());
                }
                //we mark that one of agent's card is in trick
                //card_numbers[&side] = all_hands[&side].len() + 1;
                card_numbers[&side] += 1;
            } else{

                //card_numbers[&side] = all_hands[&side].len();
            }
        }
        //we check if every agent has the same ammount (including added in tricks)
        match card_numbers.are_all_equal(){
            true => Ok(()),
            false => Err(DistributionError::NotEqualCardNumbers(card_numbers).into())
        }

    }

    fn place_card(&mut self, card: &Card) -> Result<(), BridgeCoreError>{
        //let result_node = self.
        let mut node = *self.actual_node();
        let side = self.check_current_side()?;
        let completed_tricks = self.contract.count_completed_tricks();
        let cards_in_trick = self.contract.current_trick().count_cards();
        let called_suit = self.contract.current_trick().called_suit().map(|st| st.to_owned());
        match node.hands()[&side].contains(card){
            true => match self.contract.insert_card(side, *card){
                Ok(s) => {
                    match node.remove_card_current_side(card){
                        Ok(()) => {
                            debug!("{:?} placed card {:#} on table with {:?} completed tricks and {:?} cards in current trick (called suit: {:?}).",
                                side, card, completed_tricks, cards_in_trick, called_suit);
                            node.set_current_side(s);
                            self.node_stack.push(node);

                            Ok(())
                        }
                        Err(e) => {
                            self.contract.undo()?;
                            Err(e.into())
                        }
                    }

                }
                Err(e) => Err(e.into())
            }
            false => Err(CardSetErrorGen::CardNotInSet(*card).into())
        }

    }
    pub fn actual_node(&self) -> &TrickNode{
        self.node_stack.last().unwrap()
    }

    pub fn current_hand(&self) -> CardSetStd {
        self.hands()[&self.contract().current_side()]
    }
    pub fn current_side(&self) -> Side{
        self.contract.current_side()
    }

    fn update_cache_new_trick(&mut self) -> Result<(), BridgeCoreError>{
        let mut preparer = std::mem::take(&mut self.action_optimiser);
        preparer.cache_on_trick_new(self)?;
        self.action_optimiser = preparer;
        Ok(())
    }

    fn update_cache_dropped_trick(&mut self) -> Result<(), BridgeCoreError>{
        let mut preparer = std::mem::take(&mut self.action_optimiser);
        preparer.cache_on_trick_drop(self)?;
        self.action_optimiser = preparer;
        Ok(())
    }

    fn update_cache_partial_trick(&mut self) -> Result<(), BridgeCoreError>{
        let mut preparer = std::mem::take(&mut self.action_optimiser);
        preparer.cache_on_partial_trick(self)?;
        self.action_optimiser = preparer;
        Ok(())
    }

    pub(crate) fn update(&mut self, update: ExplorerStateUpdate) -> Result<(), BridgeCoreError> {
        match update{
            PlaceCard(card) => {

                self.place_card(&card)?;
                self.current_analysis_depth+=1;
                if self.contract().current_trick().is_empty(){
                   // self.action_preparer = G::cache_on_trick_new(&self)?;
                    self.update_cache_new_trick()?;

                }
                Ok(())

            },
            ExplorerStateUpdate::Undo => {
                let completed_tricks = self.contract.count_completed_tricks();
                let cards_in_trick = self.contract.current_trick().count_cards();
                debug!("Undoing on table with {:?} tricks completed and {:?} cards in current trick.",
                    completed_tricks, cards_in_trick);
                let update_cards_cache = self.contract().current_trick().is_empty();

                match self.contract.undo(){
                    Ok(_) => {
                        self.node_stack.pop().unwrap();
                        if update_cards_cache{
                            //self.action_preparer = G::cache_on_trick_drop(&self)?;
                            self.update_cache_dropped_trick()?;
                        }
                        self.current_analysis_depth -= 1;
                        Ok(())
                    }
                    Err(e) => Err(e.into())
                }
            }
        }
    }

    pub fn available_actions(&self) -> SuitMap<SmallVec<[CardPack;CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND]>> {
        /*let iter = BridgeLegalCards::new(&self.hands()[&self.contract.current_side()], self.contract.current_trick().called_suit())
            .map(|c| CardPack::group_single_card(&c)).collect();
        iter*/
        self.action_optimiser.prepare_vec(self)


    }



}

#[derive(Debug, Clone)]
pub enum ExplorerStateUpdate{
    PlaceCard(Card),
    Undo
}


impl Display for ExplorerStateUpdate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            PlaceCard(c) => write!(f, "Update [ Card placed: {} ]", c),
            ExplorerStateUpdate::Undo => write!(f, "Update [ Undo previous update ]")
        }

    }
}
/*
impl StateUpdate for ExplorerStateUpdate{

}

 */
#[derive(Clone, Debug)]
pub struct DoubleDummyExplorerDomain {

}
/*
impl Into<AmfiError<DoubleDummyExplorerDomain>> for BridgeCoreError {
    fn into(self) -> AmfiError<DoubleDummyExplorerDomain> {
        AmfiError::GameError(self)
    }
}

impl InternalGameError<DoubleDummyExplorerDomain> for BridgeCoreError{

}

impl ProtocolSpecification for DoubleDummyExplorerDomain {
    type ActionType = CardPack;
    type GameErrorType = BridgeCoreError;
    type UpdateType = ExplorerStateUpdate;
    type AgentId = Side;
}*/
/*
impl<G: ActionOptimiser> State<ContractProtocolSpec> for ExplorerGameState<G>{
    //type UpdateType = ExplorerStateUpdate;
    //type Error = BridgeCoreError;



    fn is_finished(&self) ->bool {
        self.contract.is_completed()
    }
}

 */
/* 
impl<G: ActionOptimiser> BridgeContractState for ExplorerGameState<G> {
    type HandType = StackHand;
    type ContractType = Contract;
    
    fn dummy_hand(&self) -> Option<&Self::HandType> {
        None
    }

    fn contract(&self) -> &Self::ContractType {
        &self.contract
    }
}

*/
/*
impl<G: ActionOptimiser> InformationSet<ContractProtocolSpec> for ExplorerGameState<G>{
    //type ActionType = CardPack;
    //type Aggregator = SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]>;
    type ActionIteratorType = SuitMap<SmallVec<[CardPack;CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND]>>;

    //const  ActionSpaceBound: usize = CONTRACT_ACTION_SPACE_BOUND;




    //type Id = Side;

    fn id(&self) -> &Side {
        todo!()
    }

    fn is_action_valid(&self, _action: &CardPack) -> bool {
        todo!()
    }

    type RewardType = u32;

    fn current_reward(&self) -> Self::RewardType {
        //self.contract.total_tricks_taken_axis(self.current_side().)
        todo!()
    }
    /*
    fn side(&self) -> Side {
        self.contract.current_side()
    }

    fn set(&self) -> &Self::HandType {
        &self.node_stack.last().unwrap().hands()[&self.contract.current_side()]
    }
    */

    
}


 */


#[cfg(test)]
mod tests{
    use brydz_core::{
        cards::trump::Trump,
        bidding::Bid,
        contract::{Contract, ContractParametersGen, ContractMechanics},
        player::side::{Side::*, SideMap},
        karty::{suits::Suit::*, card_set, cards::{ACE_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS, ACE_HEARTS, KING_DIAMONDS, KING_HEARTS, JACK_DIAMONDS, ACE_DIAMONDS, ACE_CLUBS, QUEEN_HEARTS, QUEEN_CLUBS, KING_SPADES, QUEEN_DIAMONDS, JACK_SPADES, JACK_HEARTS}},
       
    };
    //use brydz_base::world::{BridgeContractAgentState, StateTrait};

    use crate::{node::{TrickNode}, };
    use crate::actions::DistinctCardGrouper;
    use crate::explore::ExplorerStateUpdate;

    use super::ExplorerGameState;

    


    #[test]
    fn double_dummy_aupdate_state(){
        
        
        let contract = Contract::new(
             ContractParametersGen::new(West, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
 
        let hands = SideMap::new(
            card_set![ACE_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS],
            card_set!(ACE_HEARTS, KING_DIAMONDS, KING_HEARTS, JACK_DIAMONDS),
            card_set![ACE_DIAMONDS, ACE_CLUBS, QUEEN_HEARTS, QUEEN_CLUBS],
            card_set![KING_SPADES, QUEEN_DIAMONDS, JACK_SPADES, JACK_HEARTS ]);
            
        let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    
        let mut explorer_state = ExplorerGameState::<DistinctCardGrouper>::new_checked(contract, node).unwrap();
        assert_eq!(explorer_state.available_actions().map(|a|a.len()).sum(), 4);
        assert!(explorer_state.update(ExplorerStateUpdate::PlaceCard(JACK_DIAMONDS)).is_err());
        explorer_state.update(ExplorerStateUpdate::PlaceCard(KING_CLUBS)).unwrap();
        assert_eq!(explorer_state.available_actions().map(|a|a.len()).sum(), 4);
        explorer_state.update(ExplorerStateUpdate::PlaceCard(KING_DIAMONDS)).unwrap();
        assert_eq!(explorer_state.available_actions().map(|a|a.len()).sum(), 2);
        explorer_state.update(ExplorerStateUpdate::PlaceCard(QUEEN_HEARTS)).unwrap();
        assert_eq!(explorer_state.available_actions().map(|a|a.len()).sum(), 4);
        explorer_state.update(ExplorerStateUpdate::PlaceCard(JACK_HEARTS)).unwrap();
        assert_eq!(explorer_state.contract.current_side(), East);
        assert_eq!(explorer_state.available_actions().map(|a|a.len()).sum(), 3);
        assert!(explorer_state.update(ExplorerStateUpdate::PlaceCard(KING_DIAMONDS)).is_err());
        explorer_state.update(ExplorerStateUpdate::PlaceCard(JACK_DIAMONDS)).unwrap();
        assert_eq!(explorer_state.available_actions().map(|a|a.len()).sum(), 1);
        assert_eq!(explorer_state.contract.current_side(), South);
        explorer_state.update(ExplorerStateUpdate::Undo).unwrap();
        assert_eq!(explorer_state.contract.current_side(), East);
        explorer_state.update(ExplorerStateUpdate::PlaceCard(KING_HEARTS)).unwrap();
        assert_eq!(explorer_state.available_actions().map(|a|a.len()).sum(), 3);


        

    }
}