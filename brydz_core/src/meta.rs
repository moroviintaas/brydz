

use karty::cards::{Card2SymTrait, Card, DECK_SIZE};
//pub const DECK_SIZE: usize = karty::suits::Suit::NUMBER_OF_SUITS * karty::figures::Figure::NUMBER_OF_FIGURES;
pub const HAND_SIZE: usize = Card::CARD_SPACE/4;



pub const QUARTER_SIZE: usize = DECK_SIZE/4 ;
pub const MIN_BID_NUMBER: u8 = 1;

pub const SIZE_SMALLER_HALF_TRICKS: usize = QUARTER_SIZE/2;
pub const SIZE_GREATER_HALF_TRICKS: usize = QUARTER_SIZE - SIZE_SMALLER_HALF_TRICKS;
pub const TOTAL_TRICKS: u8 = (Card::CARD_SPACE/4) as u8;
pub const HALF_TRICKS: u8 = (QUARTER_SIZE / 2) as u8;
pub const MAX_BID_NUMBER: u8 = QUARTER_SIZE as u8 - HALF_TRICKS;
pub const MAX_INDEX_IN_DEAL: usize = QUARTER_SIZE -1;

pub const PLAYER_NUM: usize = 4;

pub const CONTRACT_ACTION_GROUPING_ESTIMATE: usize = 8;


// As currently SmallVec can't be parametrized <[Self::Action; Self::ActionSpaceBound]>
pub const CONTRACT_ACTION_SPACE_BOUND: usize = 13;
pub const CONTRACT_ACTION_ESTIMATED_SUIT_MAP_BOUND: usize = 8;
pub const CONTRACT_ACTION_STACK_SIZE_BOUND: usize = 64;