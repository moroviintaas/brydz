use std::fmt::{Display};
use std::ops::{Index};
use karty::cards::{Card2SymTrait, Card};
use crate::contract::{SmartTrickSolver, TrickSolver};

use crate::error::TrickErrorGen::{CardSlotAlreadyUsed, ViolatedOrder};
use crate::error::{Mismatch, TrickErrorGen};

use crate::player::side::Side::{North, South, East, West};
use crate::player::side::{Side, SIDES};





#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TrickGen<Card: Card2SymTrait>{
    north_card: Option<Card>,
    west_card: Option<Card>,
    south_card: Option<Card>,
    east_card: Option<Card>,
    first_player: Side,
    card_num: u8,

}

pub type Trick = TrickGen<Card>;

impl<Card: Card2SymTrait + Copy> Copy for TrickGen<Card>{}

impl<Card: Card2SymTrait> Index<Side> for TrickGen<Card>{
    type Output = Option<Card>;

    fn index(&self, index: Side ) -> &Self::Output {
        match index{
            North => &self.north_card,
            South => &self.south_card,
            West => &self.west_card,
            East => &self.east_card
        }
    }


}

impl Display for Trick{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        
        match f.alternate(){
            true => {
                write!(f,"{{ ")?;
                for i in 0..3{
                    write!(f, "{:?}: {}, ",self.first_player.next_i(i),
                    match self[self.first_player.next_i(i)]{
                        Some(c) => format!("{c:#}"),
                        None => "-".to_owned()
                    })?;
                }
                write!(f, "{:?}: {:#} }}", self.first_player.next_i(3),
                match self[self.first_player.next_i(3)]{
                    Some(c) => format!("{c:#}"),
                    None => "-".to_owned()
                })
                
            },
            false => {
                write!(f,"{{ ")?;
                for i in 0..3{
                    write!(f, "{:?}: {}, ",self.first_player.next_i(i),
                    match self[self.first_player.next_i(i)]{
                        Some(c) => format!("{c:#}"),
                        None => "-".to_owned()
                    })?;
                }
                write! (f, "{:?}: {:#} }}", self.first_player.next_i(3),
                match self[self.first_player.next_i(3)]{
                    Some(c) => format!("{c}"),
                    None => "-".to_owned()
                })
                
            }
        }
        
    }
}

impl<Card: Card2SymTrait> TrickGen<Card>{
    pub fn new( first_player: Side) -> Self{

        Self{first_player, north_card: None, south_card: None, west_card: None, east_card: None, card_num: 0}
    }

    /// # Returns:
    /// Option of whose turn it is now
    ///
    /// `Some(Side)` if determined
    /// `None` if trick is completed
    /// ```
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::player::side::Side::{East, North, South, West};
    /// use karty::cards::{ACE_SPADES, KING_CLUBS, KING_DIAMONDS, KING_HEARTS};
    /// let mut trick = TrickGen::new(North);
    /// assert_eq!(trick.current_side(), Some(North));
    /// trick.insert_card(North, ACE_SPADES).unwrap();
    /// assert_eq!(trick.current_side(), Some(East));
    /// trick.insert_card(East, KING_HEARTS).unwrap();
    /// assert_eq!(trick.current_side(), Some(South));
    /// trick.insert_card(South, KING_DIAMONDS).unwrap();
    /// assert_eq!(trick.current_side(), Some(West));
    /// trick.insert_card(West, KING_CLUBS).unwrap();
    /// assert!(trick.current_side().is_none());
    /// ```
    pub fn current_side(&self) -> Option<Side>{
        match self.card_num{
            x@ 0..=3 => Some(self.first_player.next_i(x)),
            _ => None
        }
    }



    fn set_card(&mut self, side: Side, card: Card){
        match self.index(side){
            None => self.card_num += 1,
            Some(_) => {}
        };
        match side{
            East => self.east_card = Some(card),
            South => self.south_card = Some(card),
            West => self.west_card = Some(card),
            North => self.north_card = Some(card),
        }
    }
    fn unset_card(&mut self, side: Side) -> Option<Card>{
        match self.index(side){
            None => {}
            Some(_s) => {
                self.card_num -=1
            }
        };

        match side{
            East => self.east_card.take(),
            South => self.south_card.take(),
            West => self.west_card.take(),
            North => self.north_card.take(),
        }
    }

    pub fn insert_card(&mut self, side: Side, card: Card) ->  Result<u8, TrickErrorGen<Card>>{
        let side_in_order = match self.current_side(){
            Some(s) => s,
            None => { return Err(TrickErrorGen::TrickFull)}
        };
        match side == side_in_order{
            true => match self[side]{
                None => match self.contains(&card){
                    false => {
                        //self.card_num += 1;
                        //self[side] = Some(card);
                        self.set_card(side, card);
                        Ok(self.card_num)
                    }
                    true => Err(TrickErrorGen::DuplicateCard(card))
                }

                Some(_) => Err(CardSlotAlreadyUsed(side))
            },
            false => Err(ViolatedOrder(Mismatch{expected:side_in_order, found: side}))
        }
    }


    /// ```
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::player::side::Side::{East, North, South, West};
    /// use karty::cards::{ACE_SPADES, JACK_SPADES, KING_SPADES, QUEEN_SPADES};
    /// let mut trick = TrickGen::new(North);
    /// assert_eq!(trick.undo(), None);
    /// trick.insert_card(North, ACE_SPADES).unwrap();
    /// trick.insert_card(East, KING_SPADES).unwrap();
    /// trick.insert_card(South, QUEEN_SPADES).unwrap();
    /// trick.insert_card(West, JACK_SPADES).unwrap();
    /// assert_eq!(trick.current_side(), None);
    /// assert!(trick.is_complete());
    /// assert_eq!(trick.undo(), Some(JACK_SPADES));
    /// assert_eq!(trick.undo(), Some(QUEEN_SPADES));
    /// assert_eq!(trick.undo(), Some(KING_SPADES));
    /// assert_eq!(trick.undo(), Some(ACE_SPADES));
    /// assert!(trick.is_empty());
    /// ```
    pub fn undo(&mut self) -> Option<Card>{
        match self.is_empty(){
            true => None,
            false => match &self.current_side(){
                    None => {
                        //self.card_num -= 1;
                        let side = self.first_player.next_i(3);
                        //self[side].take()
                        self.unset_card(side)
                    },
                    Some(s) => {
                        //self.card_num -= 1;
                        //self[s.prev()].take()
                        self.unset_card(s.prev())
                    }


            }
        }
    }






    /// Checks if trick contains a  specific card
    /// ```
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::player::side::Side;
    /// use brydz_core::contract::suit_exhaust::{SuitExhaust};
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut trick = TrickGen::new(Side::North);
    /// trick.insert_card(Side::North, JACK_SPADES);
    /// assert!(trick.contains(&JACK_SPADES));
    /// assert!(!trick.contains(&ACE_SPADES));
    /// ```
    pub fn contains(&self, card: &Card) -> bool{
        for side in [North, East, South, West]{
            if self[side].as_ref().map_or(false, |c| c == card){
                return true;
            }
        }
        false
    }


    /// Checks if two tricks collide in some card
    pub fn collides(&self, other: &TrickGen<Card>) -> bool{
        self.collision(other).is_some()
    }

    pub fn count_cards(&self) -> u8{
        self.card_num
    }



    /// Checks if two tricks collide with some card
    /// # Returns:
    /// `Some(c: Card)` if there is a collision with card `c`
    /// `None` if there is no collision
    /// ```
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::player::side::Side;
    /// use brydz_core::contract::suit_exhaust::SuitExhaust;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut trick1 = TrickGen::new(Side::North);
    /// trick1.insert_card(Side::North, JACK_SPADES).unwrap();
    ///
    /// trick1.insert_card(Side::East, ACE_SPADES).unwrap();
    /// trick1.insert_card(Side::South, ACE_HEARTS).unwrap();
    /// let mut trick2 = TrickGen::new(Side::North, );
    /// trick2.insert_card(Side::North, JACK_HEARTS).unwrap();
    /// trick2.insert_card(Side::East, ACE_DIAMONDS).unwrap();
    /// assert_eq!(trick1.collision(&trick2), None);
    /// trick2.insert_card(Side::South, ACE_HEARTS).unwrap();
    /// assert_eq!(trick1.collision(&trick2), Some(ACE_HEARTS));
    /// ```
    pub fn collision(&self, other: &TrickGen<Card>) -> Option<Card>{
        for oc in [&other[North], &other[East], &other[South], &other[West]]{
            match oc {
                Some(c) => match self.contains(c){
                    true => {return Some(c.to_owned())},
                    false => {}
                },
                None => {}
            }
        }
        None
    }



    /// Checks if trick is complete
    ///
    /// ```
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::player::side::Side;
    /// use brydz_core::contract::suit_exhaust::SuitExhaust;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut trick = TrickGen::new(Side::North);
    /// trick.insert_card(Side::North, JACK_SPADES);
    /// trick.insert_card(Side::East, ACE_SPADES);
    /// trick.insert_card(Side::South, ACE_HEARTS);
    /// assert!(!trick.is_complete());
    /// trick.insert_card(Side::West, JACK_HEARTS);
    /// assert!(trick.is_complete());
    ///
    /// ```
    pub fn is_complete(&self) -> bool{

        self[North].as_ref()
            .and(self[East].as_ref())
            .and(self[South].as_ref())
            .and(self[West].as_ref())
            .is_some()
    }
    pub fn is_empty(&self) -> bool {
        self[North].as_ref()
            .or(self[East].as_ref())
            .or(self[South].as_ref())
            .or(self[West].as_ref())
            .is_none()
    }
    pub fn missing_card(&self) -> Option<Side>{
        for s in SIDES{
            if self[s].is_none(){
                return Some(s)
            }
        }
        None
    }


/*
    fn winner_of_2(&self, side_one: Side, side_two: Side, trump: &TrumpGen<Card::Suit>) -> Result<Side, TrickErrorGen<Card>>{
        let leading_suit = match &self[self.first_player]{
            Some(c) => c.suit(),
            None => {return Err(MissingCard(self.first_player))},
        };
        match (self[side_one].as_ref(), self[side_two]. as_ref()) {
            (None, _) => Err(MissingCard(side_one)),
            (_, None) => Err(MissingCard(side_two)),
            /*Some(_) => match trump.order_cards(self[check_side].as_ref().unwrap(), self[winner_so_far].as_ref().unwrap()) {
                Ordering::Greater => Ok(check_side),
                _ => Ok(winner_so_far)
            }*/
            (Some(c1), Some(c2)) => {
                if c1.suit() == c2.suit(){
                    match c1.figure().cmp(&c2.figure()){
                        Ordering::Less => Ok(side_two),
                        _ => Ok(side_one)
                    }
                } else{
                    match trump{
                        TrumpGen::Colored(s) => {
                            if &c1.suit() == s{
                                return Ok(side_one);
                            }
                            if &c2.suit() == s{
                                return Ok(side_two);
                            }
                            if c1.suit() == leading_suit{
                                return Ok(side_one);
                            }
                            if c2.suit() == leading_suit{
                                return  Ok(side_two);
                            }
                            Ok(side_one)

                            
                        },
                        TrumpGen::NoTrump => {
                            if c1.suit() == leading_suit{
                                return Ok(side_one);
                            }
                            if c2.suit() == leading_suit{
                                return  Ok(side_two);
                            }
                            Ok(side_one)
                        }
                    }
                }
                
            }
        }
    }

 */

    /// Tries to pick a winner of a trick
    /// ```
    /// use brydz_core::cards::trump::{Trump, TrumpGen};
    /// use brydz_core::cards::trump::TrumpGen::{Colored, NoTrump};
    /// use brydz_core::cards::deck::Deck;
    /// use brydz_core::player::role::PlayRole::{Declarer, Dummy, Whist, Offside};
    /// use brydz_core::contract::{NoTrumpTrickSolver, SmartTrickSolver, SOLVE_CLUBS, SOLVE_HEARTS, SOLVE_NT, TrickGen, TrickSolver, TrumpTrickSolver};
    /// use brydz_core::player::side::Side::{North, South, East, West};
    /// use brydz_core::contract::suit_exhaust::SuitExhaust;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, TWO_CLUBS).unwrap();
    /// trick1.insert_card(South, ACE_SPADES).unwrap();
    /// trick1.insert_card(West, TEN_SPADES).unwrap();
    /// assert_eq!(trick1.taker(&SOLVE_HEARTS).unwrap(), North);
    /// //assert_eq!(TrumpTrickSolver::new(Hearts).taker(&trick1), North);
    /// let mut trick2 = TrickGen::new(North);
    ///
    /// trick2.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick2.insert_card(East, TWO_CLUBS).unwrap();
    /// trick2.insert_card(South, ACE_SPADES).unwrap();
    /// trick2.insert_card(West, TEN_SPADES).unwrap();
    /// //assert_eq!(TrumpTrickSolver::new(Clubs).taker(&trick2), East);
    /// assert_eq!(trick1.taker(&SOLVE_CLUBS).unwrap(), East);
    ///
    /// let mut trick3 = TrickGen::new(East);
    /// trick3.insert_card(East, ACE_CLUBS).unwrap();
    /// trick3.insert_card(South, ACE_SPADES).unwrap();
    /// trick3.insert_card(West, TEN_SPADES).unwrap();
    /// trick3.insert_card(North, QUEEN_HEARTS).unwrap();
    /// assert_eq!(trick3.taker(&SOLVE_NT).unwrap(), East);
    /// ```
    pub fn taker(&self, solver: &SmartTrickSolver<Card>) -> Result<Side, TrickErrorGen<Card>>{
        /*match trump{
            TrumpGen::Colored(s) => TrumpTrickSolver::new(s.to_owned()).taker(self),
            TrumpGen::NoTrump => NoTrumpTrickSolver::new().taker(self)
        }*/
        solver.winner(self)

    }
/*
    /// Tries to pick a winner of a trick
    /// ```
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::cards::trump::TrumpGen::{Colored, NoTrump};
    /// use brydz_core::cards::deck::Deck;
    /// use brydz_core::agent::role::PlayRole::{Declarer, Dummy, FirstDefender, SecondDefender};
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::agent::side::Side::{North, South, East, West};
    /// use std::str::FromStr;
    /// use brydz_core::contract::suit_exhaust::SuitExhaust;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, TWO_CLUBS).unwrap();
    /// trick1.insert_card(South, ACE_SPADES).unwrap();
    /// trick1.insert_card(West, TEN_SPADES).unwrap();
    /// assert_eq!(trick1.taker(&Colored(Hearts)).unwrap(), North);
    /// let mut trick2 = TrickGen::new(North);
    ///
    /// trick2.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick2.insert_card(East, TWO_CLUBS).unwrap();
    /// trick2.insert_card(South, ACE_SPADES).unwrap();
    /// trick2.insert_card(West, TEN_SPADES).unwrap();
    /// assert_eq!(trick2.taker(&Colored(Clubs)).unwrap(), East);
    ///
    /// let mut trick3 = TrickGen::new(East);
    /// trick3.insert_card(East, ACE_CLUBS).unwrap();
    /// trick3.insert_card(South, ACE_SPADES).unwrap();
    /// trick3.insert_card(West, TEN_SPADES).unwrap();
    /// trick3.insert_card(North, QUEEN_HEARTS).unwrap();
    /// assert_eq!(trick3.taker(&NoTrump).unwrap(), East);
    /// ```
    pub fn taker(&self, trump: &TrumpGen<Card::Suit>) -> Result<Side, TrickErrorGen<Card>>{
        let mut winner_so_far = match self[self.first_player] {
            None => { return Err(MissingCard(self.first_player))},
            Some(_) => self.first_player
        };

        match trump{
            TrumpGen::Colored(_) => {
                winner_so_far = self.winner_of_2(winner_so_far, self.first_player.next_i(1), trump)?;
                winner_so_far = self.winner_of_2(winner_so_far, self.first_player.next_i(2), trump)?;
                winner_so_far = self.winner_of_2(winner_so_far, self.first_player.next_i(3), trump)?;
                Ok(winner_so_far)
            },
            TrumpGen::NoTrump => {
                match &self[self.first_player]{
                    None => Err(MissingCard(self.first_player)),
                    Some(s) => {
                        let tmp_trump = TrumpGen::Colored(s.suit());
                        winner_so_far = self.winner_of_2(winner_so_far, self.first_player.next_i(1), &tmp_trump)?;
                        winner_so_far = self.winner_of_2(winner_so_far, self.first_player.next_i(2), &tmp_trump)?;
                        winner_so_far = self.winner_of_2(winner_so_far, self.first_player.next_i(3), &tmp_trump)?;
                        Ok(winner_so_far)
                    }

                }
            }
        }

    }
    /// ```
    /// use brydz_core::agent::side::Side::*;
    /// use karty::suits::Suit::*;
    /// use brydz_core::cards::trump::TrumpGen::*;
    /// use karty::cards::*;
    /// use brydz_core::contract::{TrickGen, suit_exhaust::*};
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, ACE_CLUBS).unwrap();

    /// assert_eq!(trick1.leading_side(&Colored(Hearts)).unwrap(), North);
    /// //assert_eq!(trick1.leading_side(&NoTrump).unwrap(), North);
    /// //assert_eq!(trick1.leading_side(&Colored(Clubs)).unwrap(), East);
    /// 
    /// trick1.insert_card(South, ACE_SPADES).unwrap();
    /// trick1.insert_card(West, ACE_HEARTS).unwrap();
    /// //assert_eq!(trick1.leading_side(&Colored(Hearts)).unwrap(), West);
    /// //assert_eq!(trick1.leading_side(&NoTrump).unwrap(), West);
    /// //assert_eq!(trick1.leading_side(&Colored(Spades)).unwrap(), South);
    /// 
    /// ```
    /// ```
    /// use karty::cards::*;
    /// use brydz_core::agent::side::Side::*;
    /// use brydz_core::contract::suit_exhaust::SuitExhaust;
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::contract::Trick;
    /// let mut trick = Trick::new(North);
    /// trick.insert_card(North, NINE_HEARTS).unwrap();
    /// trick.insert_card(East, TEN_SPADES).unwrap();
    /// trick.insert_card(South, FIVE_CLUBS).unwrap();
    /// assert_eq!(trick.leading_side(&Trump::NoTrump), Some(North));
    /// ```
    pub fn leading_side(&self, trump: &TrumpGen<Card::Suit>) -> Option<Side>{

        self[self.first_player_side()].as_ref().map(|_|{
            match self.winner_of_2(self.first_player, self.first_player.next(), trump){
                Ok(winner1) => match self.winner_of_2(winner1, self.first_player.next_i(2), trump){
                    Ok(winner2) => match self.winner_of_2(winner2, self.first_player.next_i(3), trump){
                        Ok(w) => w,
                        Err(_) => winner2,
                    }
                    Err(_) => winner1,
                },
                Err(_) => self.first_player,
            }
        })
    }
    /// ```
    /// use brydz_core::agent::side::Side::*;
    /// use karty::suits::Suit::*;
    /// use brydz_core::cards::trump::TrumpGen::*;
    /// use karty::cards::*;
    /// use brydz_core::contract::{TrickGen, suit_exhaust::*};
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, ACE_CLUBS).unwrap();
    /// trick1.insert_card(South, KING_HEARTS).unwrap();
    /// trick1.insert_card(West, TEN_SPADES).unwrap();
    /// assert_eq!(trick1.leader_in_suit(&Hearts), Some(South));
    /// assert_eq!(trick1.leader_in_suit(&Clubs), Some(East));
    /// assert_eq!(trick1.leader_in_suit(&Spades), Some(West));
    /// assert_eq!(trick1.leader_in_suit(&Diamonds), None);
    /// ```

 */
    pub fn leader_in_suit(&self, suit: &Card::Suit) -> Option<Side>{
        SIDES.iter()
            .map(|s| (s, &self[*s]))
            .filter_map(|(s, oc)| {
                match oc{
                    None => None,
                    Some(c) => match suit == &c.suit() {
                        true => Some((s, c.figure())),
                        false => None
                    }
                }
            } ).max_by_key(|(_s, f) | f.to_owned())
            .map(|(s, _)| *s)


    }
    pub fn leader_in_called_suit(&self) -> Option<Side>{
        self.called_suit().and_then(|s|self.leader_in_suit(&s))
    }

    pub fn leader_in_suit_with_card(&self, suit: &Card::Suit) -> Option<(Side, &Card)>{
        SIDES.iter()
            .map(|s| (s, &self[*s]))
            .filter_map(|(s, oc)| {
                match oc{
                    None => None,
                    Some(c) => match suit == &c.suit() {
                        true => Some((s, c)),
                        false => None
                    }
                }
            } ).max_by_key (|(_s, c) | c.figure())
            .map(|(s, c)| (*s,c))
    }
    pub fn leader_in_called_suit_with_card(&self) -> Option<(Side, &Card)>{
        self.called_suit().and_then(|s| self.leader_in_suit_with_card(&s))
    }
/*
    /// ```
    /// use karty::cards::*;
    /// use brydz_core::agent::side::Side::*;
    ///  use brydz_core::contract::suit_exhaust::SuitExhaust;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::Trick;
    /// use karty::suits::Suit::*;
    /// let mut trick = Trick::new(North);
    /// let mut exhaust_register = SuitExhaust::default();
    /// trick.insert_card(North, NINE_HEARTS).unwrap();
    /// trick.insert_card(East, TEN_SPADES).unwrap();
    /// trick.insert_card(South, FIVE_CLUBS).unwrap();
    /// assert!(!trick.is_winning_card(&FOUR_DIAMONDS, &TrumpGen::NoTrump));
    /// assert!(trick.is_winning_card(&FOUR_DIAMONDS, &TrumpGen::Colored(Diamonds)));
    /// assert!(trick.is_winning_card(&TEN_HEARTS, &TrumpGen::NoTrump));
    /// ```
    pub fn is_winning_card(&self, card: &Card, trump: &TrumpGen<Card::Suit>) -> bool{
        match self.leading_side(trump){
            None => true, //first card
            Some(s) =>{
                let c1 = self[s].as_ref().unwrap();
                if card.suit() == c1.suit(){ //same suit as currently leading
                    return card.figure() > c1.figure()
                }
                // different suit as currently leading
                match trump{
                    TrumpGen::Colored(trump_suit) => {
                        &card.suit() == trump_suit
                        //if card has trump color it is a winner
                    },
                    TrumpGen::NoTrump => false
                    //in no trump contract if card does not match first card it loses
                }
            }
        }
    }

    pub fn prepare_new(&self, trump: TrumpGen<Card::Suit>) -> Option<Self>{
        self.taker(&trump).ok().map(|s| TrickGen::new(s))
    }

 */
    pub fn called_suit(&self) -> Option<Card::Suit>{
        self[self.first_player].as_ref().map(|c| c.suit())
    }
    pub fn first_player_side(&self) -> Side{
        self.first_player
    }

    pub fn first_card(&self) -> Option<&Card>{
        self[self.first_player].as_ref()
    }

}

impl<Card: Card2SymTrait> Default for TrickGen<Card>{
    fn default() -> Self {
        Self{card_num:0, first_player: North, north_card: None, east_card: None, south_card: None, west_card:None}
    }
}


