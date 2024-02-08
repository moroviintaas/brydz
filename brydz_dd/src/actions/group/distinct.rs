use smallvec::SmallVec;
use brydz_core::meta::{CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND};
use crate::actions::card_pack::CardPack;
use crate::actions::{ActionOptimiser};
use brydz_core::contract::{ContractMechanics};
use brydz_core::error::BridgeCoreError;
use brydz_core::karty::hand::HandSuitedTrait;
use brydz_core::karty::suits::{SuitMap};
use crate::explore::ExplorerGameState;

#[derive(Debug, Copy, Clone, Default)]
pub struct DistinctCardGrouper {

}
/*
impl Default for DistinctCardGrouper {
    fn default() -> Self {
        Self{}
    }
}*/

impl ActionOptimiser for DistinctCardGrouper{
    fn cache_on_trick_new(&mut self, _state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError> {
        Ok(())
    }

    fn cache_on_trick_drop(&mut self, _state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError> {
        Ok(())
    }

    fn cache_on_partial_trick(&mut self, _state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError> {
        Ok(())
    }


    fn prepare_vec(&self, state: &ExplorerGameState<Self>) -> SuitMap<SmallVec<[CardPack; CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND]>> {
        match state.contract().current_trick().called_suit(){
            None => SuitMap::new_from_f(|s|{
                state.hands()[&state.contract().current_side()].suit_iterator(&s).map(|c| CardPack::group_single_card(&c)).collect()
            }),
            Some(called) => match state.hands()[&state.current_side()].contains_in_suit(&called){
                true => SuitMap::single(&called, state.current_hand().suit_iterator(&called).map(|c| CardPack::group_single_card(&c)).collect()),
                false => SuitMap::new_from_f(|s|{
                    state.hands()[&state.contract().current_side()].suit_iterator(&s).map(|c| CardPack::group_single_card(&c)).collect()
                })
            }
        }
    }
    /*fn generate_card_groups(state: &ExplorerGameState<Self>) -> SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]> {
        BridgeLegalCards::new(&state.hands()[&state.contract().current_side()], state.contract().current_trick().called_suit())

            .rev().map(|c| CardPack::group_single_card(&c)).collect()
    }*/


}