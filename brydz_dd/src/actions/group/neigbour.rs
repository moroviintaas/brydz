use smallvec::SmallVec;
use brydz_core::contract::ContractMechanics;
use brydz_core::error::{BridgeCoreError};
use brydz_core::karty::hand::{HandSuitedTrait, HandTrait};
use brydz_core::karty::suits::{Suit, SuitMap};
use brydz_core::karty::symbol::CardSymbol;
use brydz_core::meta::{CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND, CONTRACT_ACTION_SPACE_BOUND};
use brydz_core::player::side::{Side, SideMap};
use crate::actions::{CardPack, ActionOptimiser};
use crate::actions::GroupingReason::Neighbouring;
use crate::explore::ExplorerGameState;


type NeighbourCache = SideMap<SuitMap<SmallVec<[CardPack; CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND]>>>;

#[derive(Debug, Clone, Default)]
pub struct NeighbourCardGrouper{
    stack: SmallVec<[NeighbourCache; CONTRACT_ACTION_SPACE_BOUND-1]>
}




impl ActionOptimiser for NeighbourCardGrouper{
    fn cache_on_trick_new(&mut self, state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError> {
        /*
        let cache_entry = SideMap::new_with_fn(|side|{
           let side_actions = SuitMap::new_from_f(|suit|{

           })
        });

         */
        let all_cards = state.hands().merge(|s1, s2| s1.union(s2));

        let mut result = NeighbourCache::default();

        for suit in Suit::iterator(){
            //let mut current_card_pack = None;
            //let mut current_side = None;

            let mut current: Option<(CardPack, Side)> = None;
            for card in all_cards.suit_iterator(&suit){
                match state.hands().find(|h| h.contains(&card)){
                    None => panic!("Bug: Card not found in any of hands, despite iterating over their union."),
                    Some(side) => match current {
                        None => {
                            current = Some((CardPack::group_single_card(&card), side))
                        }
                        Some(( ref mut c,  ref mut s)) => {
                            if s==&side{
                                c.push(card).expect("Bug: Iterating in one suit, so it should not mismatch");
                                c.set_reason(Neighbouring);

                            } else{

                                let mut swp = CardPack::group_single_card(&card);
                                std::mem::swap(&mut swp, c);
                                result[s][suit].push(swp);
                                *s = side;
                            }

                        }
                    }
                }

            }
            match current{
                None => {}
                Some((cp, cs))  => {
                    result[&cs][suit].push(cp);
                }
            }

        }
        self.stack.push(result);

        Ok(())
    }

    fn cache_on_trick_drop(&mut self, _state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError> {
        //Ok(Self{})
        self.stack.pop().expect("Error system for action searching is not yet stable.");
        Ok(())
        //Need to think if this should return BridgeCoreError or something of DoubleDummyError, the latter might be better.
        //It is to be thought about
    }

    fn cache_on_partial_trick(&mut self, _state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError> {
        //Ok(Self{})
        todo!()
    }

    fn prepare_vec(&self, state: &ExplorerGameState<Self>) -> SuitMap<SmallVec<[CardPack; CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND]>> {
        let hand = state.hands()[&state.contract().current_side()];
        match state.contract().current_trick().called_suit(){
            None => {
                self.stack.last().unwrap_or(&NeighbourCache::default())[&state.current_side()].clone()
            }
            Some(called) => match hand.contains_in_suit(&called){
                true => {
                    //todo!();
                    //we are forced to iterate over this single suit
                    //SuitMap::single(*called, hand.suit_iterator(&called).collect())
                    SuitMap::single(&called, self.stack.last().unwrap_or(&NeighbourCache::default())[&state.current_side()][called].clone())//.clone()
                }
                false => self.stack.last().unwrap_or(&NeighbourCache::default())[&state.current_side()].clone()



            }
        }

    }
}