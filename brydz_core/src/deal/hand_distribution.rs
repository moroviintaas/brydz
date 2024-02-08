use karty::symbol::CardSymbol;
use rand::{prelude::SliceRandom, Rng, thread_rng};

use karty::cards::STANDARD_DECK;
use karty::hand::{CardSet, HandTrait};

use crate::player::side::{Side, SideMap};
use crate::player::side::Side::{East, North, South, West};

//use super::hand::HandTrait;

pub struct HandDistribution{

}



/// Creates fair distribution of cards for bridge game, it assumes that used `CardSymbol` has space divisible by 4;
/// It uses `CardSymbol::iterator()` 
/// ```
/// use brydz_core::deal::fair_bridge_deal;
/// use karty::cards::STANDARD_DECK;
/// use karty::hand::{HandSetStd, HandTrait};
/// let mut table = fair_bridge_deal::<HandSetStd>();
/// assert_eq!(table.north.len(), 13);
/// assert_eq!(table.east.len(), 13);
/// assert_eq!(table.west.len(), 13);
/// assert_eq!(table.south.len(), 13);
/// for c in &STANDARD_DECK{
///     assert!(table.or(|h| h.contains(c)));
/// }
///
/// ```
/// ```
/// use brydz_core::deal::fair_bridge_deal;
/// use karty::cards::STANDARD_DECK;
/// use karty::hand::{HandTrait, CardSet};
/// let mut table = fair_bridge_deal::<CardSet>();
/// assert_eq!(table.north.len(), 13);
/// assert_eq!(table.east.len(), 13);
/// assert_eq!(table.west.len(), 13);
/// assert_eq!(table.south.len(), 13);
/// for c in &STANDARD_DECK{
///     assert!(table.or(|h| h.contains(c)));
/// }
///
/// ```
pub fn fair_bridge_deal<H: HandTrait>() -> SideMap<H>{
    let mut result = SideMap::<H>{
        north: H::empty(),
        east: H::empty(),
        south: H::empty(),
        west: H::empty(),
    };
    let mut rng = thread_rng();
    let mut v  = Vec::from_iter(H::CardType::iterator()); 
    
    v.shuffle(&mut rng);
    let hand_size = v.len()/4;
    /*let north = &v[..hand_size];
    let east = &v[hand_size..2*hand_size];
    let west = &v[2*hand_size..3*hand_size];
    let south = &v[3*hand_size..];
    */

    for _ in 0..hand_size{
        result.north.insert_card(v.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_size{
        result.south.insert_card(v.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_size{
        result.east.insert_card(v.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_size{
        result.west.insert_card(v.pop().unwrap()).unwrap();
    }
    result
}


pub fn distribute_standard_deck_on_4<R: Rng + ?Sized>(rng: &mut R) -> SideMap<CardSet>{
    let mut cards = STANDARD_DECK;
    let mut result = SideMap::<CardSet>::new_symmetric(CardSet::empty());
    cards.shuffle(rng);
    for i in 0..cards.len()/4{
        result.north.insert_card(cards[i*4]).unwrap();
        result.east.insert_card(cards[(i*4) + 1]).unwrap();
        result.south.insert_card(cards[(i*4) + 2]).unwrap();
        result.west.insert_card(cards[(i*4) + 3]).unwrap();

    }
    result

}



/// ```
/// use brydz_core::deal::fair_bridge_partial_deal;
/// use brydz_core::player::side::Side::{North, West};
/// use karty::cards::{Card, Card2SymTrait};
/// use karty::figures::{Ace, Jack, King, Queen, F10};
/// use karty::hand::{HandTrait, CardSet};
/// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
/// let card_supply: Vec<Card> = Card::card_subset(vec![Ace, King, Queen], vec![Spades, Hearts, Diamonds, Clubs]).collect();
/// let hands = fair_bridge_partial_deal::<CardSet>(card_supply, North);
/// assert_eq!(hands[&North].len(), 3);
/// assert_eq!(hands[&West].len(), 3);
/// ```
/// ```
/// use brydz_core::deal::fair_bridge_partial_deal;
/// use brydz_core::player::side::Side::{East, North, South, West};
/// use karty::cards::{Card, Card2SymTrait};
/// use karty::figures::{Ace, Jack, King, Queen, F10};
/// use karty::hand::{HandTrait, CardSet};
/// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
/// let mut card_supply: Vec<Card> = Card::card_subset(vec![Ace, King, Queen], vec![Spades, Hearts, Diamonds, Clubs]).collect();
/// card_supply.pop();
/// card_supply.pop();
/// let hands = fair_bridge_partial_deal::<CardSet>(card_supply.clone(), North);
/// assert_eq!(hands[&North].len(), 2);
/// assert_eq!(hands[&West].len(), 3);
/// assert_eq!(hands[&South].len(), 3);
/// assert_eq!(hands[&East].len(), 2);
/// card_supply.pop();
/// let hands = fair_bridge_partial_deal::<CardSet>(card_supply, North);
/// assert_eq!(hands[&West].len(), 3);
/// assert_eq!(hands[&South].len(), 2);
/// ```
pub fn fair_bridge_partial_deal<H: HandTrait>(mut card_supply: Vec<H::CardType>, first_side: Side ) -> SideMap<H> {
    let mut result = SideMap::<H>{
        north: H::empty(),
        east: H::empty(),
        south: H::empty(),
        west: H::empty(),
    };
    let min_hands = card_supply.len()/4;
    let rest = card_supply.len() - (4 * min_hands);
    let hand_sizes = match rest as u8{
        0 =>{
            SideMap::new(min_hands, min_hands, min_hands, min_hands)
        }
        r =>{
            let mut tmp = SideMap::new(min_hands, min_hands, min_hands, min_hands);
            for i in 0..r{
                tmp[&first_side.next_i(3-i)]+=1;
            }
            tmp
        }
    };
    let mut rng = thread_rng();
    card_supply.shuffle(&mut rng);

    for _ in 0..hand_sizes[&North]{
        result.north.insert_card(card_supply.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_sizes[&South]{
        result.south.insert_card(card_supply.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_sizes[&East]{
        result.east.insert_card(card_supply.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_sizes[&West]{
        result.west.insert_card(card_supply.pop().unwrap()).unwrap();
    }
    result
}