mod distinct;
mod neigbour;

use std::cmp::Ordering;
use std::fmt::Debug;
use smallvec::SmallVec;
use brydz_core::error::BridgeCoreError;
use brydz_core::karty::cards::Card;
use brydz_core::karty::set::{CardSetStd, StackHandIntervalIterator};
use brydz_core::karty::suits::SuitMap;
use brydz_core::meta::{CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND, };
pub use distinct::*;
pub use neigbour::*;
use crate::actions::card_pack::CardPack;
use crate::explore::ExplorerGameState;


pub trait ActionOptimiser: Debug  + Clone + Default{
    //fn group(set: &StackHand, suit: &Suit) -> SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]>;
    //fn group_in_context(hands: &SideMap<StackHand>, side: Side, suit: &Suit) -> SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]>;
    //fn merge(packs: VCardPack) -> VCardPack;
    //fn generate_card_groups(side: Side, hands: &SideMap<StackHand>, trick: &Trick, used_cards: &CardRegister, trump: &Trump) -> SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]>;
    //fn generate_card_groups(state: &ExplorerGameState<Self>) -> SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]>;

    fn cache_on_trick_new(&mut self, state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError>;
    fn cache_on_trick_drop(&mut self, state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError>;
    fn cache_on_partial_trick(&mut self, state: &ExplorerGameState<Self>) -> Result<(), BridgeCoreError>;


    fn prepare_vec(&self, state: &ExplorerGameState<Self>) -> SuitMap<SmallVec<[CardPack; CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND]>>;



}

/// ```
/// use brydz_core::karty::set::CardSetStd;
/// use brydz_core::karty::card_set;
/// use brydz_core::karty::cards::*;
/// use brydz_dd::actions::{are_cards_neighbouring, DistinctCardGrouper};
/// let card_set = card_set![KING_CLUBS, QUEEN_CLUBS, TEN_CLUBS];
/// assert!(are_cards_neighbouring(&card_set, &TEN_CLUBS, &QUEEN_CLUBS));
/// assert!(!are_cards_neighbouring(&card_set, &TEN_CLUBS, &KING_CLUBS));
/// assert!(!are_cards_neighbouring(&card_set,  &KING_CLUBS, &TEN_CLUBS));
/// ```
pub fn are_cards_neighbouring(hand_set: &CardSetStd, card_1: &Card, card_2: &Card) -> bool{
    let (card_h , card_l) : (Card, Card) = match card_1.mask().cmp(&card_2.mask()){
        Ordering::Less =>  (Card::from_mask(card_2.mask()>>1).unwrap(), Card::from_mask(card_1.mask()<<1).unwrap()),
        Ordering::Equal => {return true},
        Ordering::Greater => (Card::from_mask(card_1.mask()>>1).unwrap(), Card::from_mask(card_2.mask()<<1).unwrap()),
    };
    StackHandIntervalIterator::new(*hand_set, &card_l, &card_h).next().is_none()
}


