use brydz_core::error::CardSetErrorGen;
use brydz_core::karty::cards::{Card, Card2SymTrait};
use brydz_core::karty::error::CardSetError;
use brydz_core::karty::set::{CardSet, CardSetStd};
use brydz_core::karty::suits::Suit;
use brydz_core::karty::symbol::CardSymbol;
use brydz_core::player::side::{Side, SideMap, SIDES};
use brydz_core::player::side::Side::{East, North, South, West};
//use crate::hash::{StateHash24, PartialHash, StateHash24EntryDistinguish};


/// ```
/// use brydz_dd::node::TrickNode;
/// assert_eq!(std::mem::size_of::<TrickNode>(), 4*8 + 8)
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TrickNode {
    hands: SideMap<CardSetStd>,
    current_side: Side,

}

impl TrickNode{
    pub fn new(hands: SideMap<CardSetStd>, current_side: Side) -> Self{
        Self{hands, current_side}
    }

    /// ```
    /// use brydz_core::deal::fair_bridge_deal;
    /// use brydz_core::player::side::Side::{North, West};
    /// use brydz_dd::node::TrickNode;
    /// use brydz_core::karty::cards::Card;
    /// use brydz_core::karty::set::CardSet;
    /// use brydz_core::error::CardSetErrorGen;
    /// let mut hands = fair_bridge_deal();
    /// let trick_node = TrickNode::new_checked(hands, North).unwrap();
    ///
    /// let cards_north: Vec<Card> = hands[&North].into_iter().collect();
    /// let cards_west: Vec<Card> = hands[&West].into_iter().collect();
    ///
    /// hands[&North].remove_card(&cards_north[0]).unwrap();
    /// hands[&North].insert_card(cards_west[1]).unwrap();
    /// let res_trick_node = TrickNode::new_checked(hands, North);
    /// assert_eq!(res_trick_node, Err(CardSetErrorGen::CardDuplicated(cards_west[1])));
    ///
    /// ```
    /// ```
    /// use brydz_core::{
    /// cards::trump::Trump,
    /// bidding::Bid,
    /// contract::{Contract, ContractParametersGen, ContractMechanics},
    /// player::side::{Side::*, SideMap},
    /// karty::{suits::Suit::*, set::{CardSetStd, CardSet}, card_set, cards::{ACE_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS, ACE_HEARTS, KING_DIAMONDS, KING_HEARTS, JACK_DIAMONDS, ACE_DIAMONDS, ACE_CLUBS, QUEEN_HEARTS, QUEEN_CLUBS, KING_SPADES, QUEEN_DIAMONDS, JACK_SPADES, JACK_HEARTS}},
    /// };
    /// use brydz_dd::node::TrickNode;
    /// let mut contract = Contract::new(
    /// ContractParametersGen::new(West, Bid::init(Trump::Colored(Spades), 1).unwrap()));
    /// let hands = SideMap::new(
    ///     card_set![ACE_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS],
    ///     card_set!(ACE_HEARTS, KING_DIAMONDS, KING_HEARTS, JACK_DIAMONDS),
    ///     card_set![ACE_DIAMONDS, ACE_CLUBS, QUEEN_HEARTS, QUEEN_CLUBS],
    ///     card_set![KING_SPADES, QUEEN_DIAMONDS, JACK_SPADES, JACK_HEARTS ]);
    /// assert_eq!(hands[&North].len(), 4);
    /// let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    /// ```

    pub fn new_checked(hands: SideMap<CardSetStd>, current_side: Side) -> Result<Self, CardSetError>{
        for s1 in SIDES{
            for s2 in SIDES{
                if s1 != s2{
                    if hands[&s1].len() != hands[&s2].len(){
                        return Err(CardSetErrorGen::DifferentLengths(hands[&s1].len(), hands[&s2].len()))
                    }
                    let int_hand = hands[&s1].intersection(&hands[&s2]);

                    if !int_hand.is_empty(){
                        let duplicated = int_hand.to_vec()[0];
                        return Err(CardSetErrorGen::CardDuplicated(duplicated))

                    }
                }
            }
        }
        Ok(Self{hands, current_side})
    }

    pub fn flatten_hands(&self) -> u64{
        self.hands[&East]
            .union(&self.hands[&South])
            .union(&self.hands[&West])
            .union(&self.hands[&North]).into()
    }
    #[allow(dead_code)]
    fn hands_mut(&mut self) -> &mut SideMap<CardSetStd>{
        &mut self.hands
    }
    pub fn hands(&self) -> &SideMap<CardSetStd>{
        &self.hands
    }
    pub fn current_side(&self) -> Side{
        self.current_side
    }
    pub fn remove_card_current_side(&mut self, card: &Card) -> Result<(), CardSetError> {
        self.hands[&self.current_side].remove_card(card)
    }
    pub fn set_current_side(&mut self, side: Side){
        self.current_side = side;
    }

    /// ```
    /// use brydz_core::karty::cards::{*};
    /// use brydz_core::karty::card_set;
    /// use brydz_core::karty::suits::Suit::*;
    /// use brydz_core::player::side::Side::*;
    /// use brydz_core::player::side::SideMap;
    /// use brydz_dd::node::TrickNode;
    /// let hands = SideMap::new(card_set![ACE_SPADES, QUEEN_CLUBS],
    ///     card_set![ACE_CLUBS,  KING_HEARTS],
    ///     card_set![ACE_HEARTS, KING_CLUBS],
    ///     card_set![KING_SPADES,  QUEEN_HEARTS]);
    /// let trick_node = TrickNode::new_checked(hands, North).unwrap();
    /// assert_eq!(trick_node.suit_leader(&Spades), Some((North, ACE_SPADES)));
    /// assert_eq!(trick_node.suit_leader(&Hearts), Some((South, ACE_HEARTS)));
    /// assert_eq!(trick_node.suit_leader(&Diamonds), None);
    /// assert_eq!(trick_node.suit_leader(&Clubs), Some((East, ACE_CLUBS)));
    /// ```
    pub fn suit_leader(&self, suit: &Suit) -> Option<(Side, Card)>{
        let player_leading_card = SideMap::new_with_fn(|s|self.hands[&s].highest_in_suit(suit));
        let leading_side = player_leading_card.select_best_fit(|c|
            match c{
                Some(card) => card.figure().usize_index() as i64,
                None => -1
            });
        /*match player_leading_card[&leading_side]{
            Some(card) => Some((leading_side, card)),
            None => None

        }*/
        player_leading_card[&leading_side].map(|card| (leading_side, card))

    }
    /// ```
    /// use brydz_core::karty::cards::{*};
    /// use brydz_core::karty::card_set;
    /// use brydz_core::karty::suits::Suit::*;
    /// use brydz_core::player::side::Side::*;
    /// use brydz_core::player::side::SideMap;
    /// use brydz_dd::node::TrickNode;
    /// let hands = SideMap::new(card_set![ACE_SPADES, QUEEN_CLUBS],
    ///     card_set![ACE_CLUBS,  KING_HEARTS],
    ///     card_set![ACE_HEARTS, KING_CLUBS],
    ///     card_set![KING_SPADES,  QUEEN_HEARTS]);
    /// let trick_node = TrickNode::new_checked(hands, North).unwrap();
    /// assert_eq!(trick_node.who_has_card(&ACE_HEARTS), Some(South));
    /// assert_eq!(trick_node.who_has_card(&JACK_DIAMONDS), None);
    /// ```
    pub fn who_has_card(&self, card: &Card) -> Option<Side>{
        for side in &SIDES{
            if self.hands[side].contains(card){
                return Some(*side)
            }
        }
        None
    }


}

/*
impl PartialHash<StateHash24, StateHash24EntryDistinguish> for TrickNode{
    fn partial_hash(&self) -> StateHash24 {
        let cards_still_in_game = self.flatten_hands();

        let high6cards = ((cards_still_in_game >> SHIFT_HIGH_CLUBS) & MASK_HIGH_CLUBS) ^
            ((cards_still_in_game >> SHIFT_HIGH_DIAMONDS) & MASK_HIGH_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_HIGH_HEARTS) & MASK_HIGH_HEARTS) ^
            ((cards_still_in_game >> SHIFT_HIGH_SPADES) & MASK_HIGH_SPADES);

        //let buffer6high_cards = high6cards.to_le_bytes();

            //TODO Make hash more distributed  __ mix()

        StateHash24::new(high6cards as u32)

    }

    fn entry_distinguish(&self) -> StateHash24EntryDistinguish {
        let cards_still_in_game = self.flatten_hands();


        let lower8cards = ((cards_still_in_game >> (SHIFT_LOWER_CLUBS)) & MASK_LOWER_CLUBS) ^

            ((cards_still_in_game >> SHIFT_LOWER_DIAMONDS) & MASK_LOWER_DIAMONDS) ^

            ((cards_still_in_game >> SHIFT_LOWER_HEARTS) & MASK_LOWER_HEARTS) ^
            ((cards_still_in_game >> SHIFT_LOWER_SPADES) & MASK_LOWER_SPADES);

        StateHash24EntryDistinguish::new(lower8cards as u32, self.current_side)

    }

    fn check(&self, hash: &StateHash24, entry_distinguish: &StateHash24EntryDistinguish) -> bool {


        //here todo changes when changing hashing method

        &self.partial_hash() == hash && &self.entry_distinguish() == entry_distinguish


    }
}
*/
/*
const MASK_HIGH_CLUBS: u64 =            0x3f;
const MASK_HIGH_DIAMONDS: u64 =       0x0fc0;
const MASK_HIGH_HEARTS:u64 =        0x03f000;
const MASK_HIGH_SPADES: u64 =       0xfc0000;

const MASK_LOWER_CLUBS: u64 = 0xff;
const MASK_LOWER_DIAMONDS: u64 = 0xff00;
const MASK_LOWER_HEARTS: u64 = 0xff0000;
const MASK_LOWER_SPADES: u64 = 0xff000000;


const SHIFT_HIGH_CLUBS: usize = 7;
const SHIFT_HIGH_DIAMONDS: usize = 14; // 13 to 0, but 6 to be left for clubs and 7 to cut lower
const SHIFT_HIGH_HEARTS: usize = 21 ;
const SHIFT_HIGH_SPADES: usize = 28;

const SHIFT_LOWER_CLUBS: usize = 0;
const SHIFT_LOWER_DIAMONDS: usize = 5;
const SHIFT_LOWER_HEARTS: usize = 10;
const SHIFT_LOWER_SPADES: usize = 15;

 */
/* 
#[cfg(test)]
mod tests{
    use brydz_core::deal::fair_bridge_deal;
    use brydz_core::karty::set::{HandTrait, StackHand};
    use brydz_core::agent::side::Side::North;
    //use crate::hash::{PartialHash, StateHash24, StateHash24EntryDistinguish};
    use crate::node::trick_node::TrickNode;

    
    #[test]
    fn check_full_hands_hash(){
        let mut trick_node = TrickNode::new(fair_bridge_deal::<StackHand>(), North);
        let hash1 = StateHash24::new(0xffffff);
        let hash_dist_1 = StateHash24EntryDistinguish::new(0xffffffff, North);
        assert_eq!(trick_node.partial_hash(), hash1);
        assert_eq!(trick_node.entry_distinguish(), hash_dist_1);
        assert!(trick_node.check(&hash1, &hash_dist_1));
        let north_card_1 = trick_node.hands()[&North].into_iter().next().unwrap();
        trick_node.hands_mut()[&North].remove_card(&north_card_1).unwrap();
        assert!(!trick_node.check(&hash1, &hash_dist_1));


    }

     
}
*/