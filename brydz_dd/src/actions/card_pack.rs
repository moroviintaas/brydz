use std::fmt::{Display, Formatter};

use brydz_core::{
    karty::{
        suits::Suit,
        figures::Figure,
        hand::CardSet,
        cards::{Card2SymTrait, Card}
    },
    meta::{CONTRACT_ACTION_SPACE_BOUND},
    contract::Trick };

use smallvec::SmallVec;
use std::fmt::Debug;
use brydz_core::error::Mismatch;
use brydz_core::meta::CONTRACT_ACTION_GROUPING_ESTIMATE;
use brydz_core::karty::hand::HandSuitedTrait;
use brydz_core::amfi::re_export::domain::Action;
use crate::error::GroupingError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupingReason{
    SureLose,
    SureWin,
    Neighbouring,
    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardPack {
    suit: Suit,
    figures: SmallVec<[Figure; CONTRACT_ACTION_GROUPING_ESTIMATE]>,
    #[allow(dead_code)]
    grouping:  GroupingReason
}


impl CardPack {
    /*pub fn new(suit: Suit, figures: BinaryHeap<Reverse<Figure>>, grouping: GroupingReason) -> Self{
        Self{suit, figures, grouping}
    }*/
    pub fn group_single_card(card: &Card) -> Self{
        let mut figures  = SmallVec::new();
        figures.push(card.figure());
        Self { suit: card.suit(), figures, grouping: GroupingReason::None}
    }

    pub fn push(&mut self, card: Card) -> Result<(), GroupingError>{
        if card.suit() == self.suit{
            self.figures.push(card.figure());
            Ok(())
        } else{
            Err(GroupingError::SuitMismatch(Mismatch{expected: self.suit, found: card.suit()}))
        }

    }

    pub fn set_reason(&mut self, reason: GroupingReason){
        self.grouping = reason
    }
    pub fn first(&self) -> Option<Card>{
        self.figures.first().map(|f| Card::from_figure_and_suit(*f, self.suit))
    }
    pub fn last(&self) -> Option<Card>{
        self.figures.last().map(|f| Card::from_figure_and_suit(*f, self.suit))
    }
    /*
    fn add_figure(&mut self, figure: Figure){
        self.figures.push(Reverse(figure))
    } */

    pub fn figures(&self) -> &SmallVec<[Figure;CONTRACT_ACTION_GROUPING_ESTIMATE]>{
        &self.figures
    }
    //pub fn cards(&self) -> SmallVec::<[Figure;CONTRACT_ACTION_SPACE_BOUND]>
    pub fn suit(&self) -> Suit{
        self.suit
    }
    /*pub fn pop(&mut self) -> Option<Figure>{
        self.figures.pop().map(|f| f)
    }*/
    pub fn lowest_card(&self) -> Card{
        /*self.figures.peek().map(|f|
        Card::from_figure_and_suit(*f, self.suit)).unwrap()*/

        Card::from_figure_and_suit(*self.figures.iter().min().unwrap(), self.suit)
        //logic should ensure that group is never empty - it must consist of at least one card
    }





    pub fn vec_no_grouping(hand: &CardSet, suit: &Suit) -> Vec<CardPack>{
        hand.suit_iterator(suit).rev().map(|c| CardPack::group_single_card(&c)).collect()
    }
    /// ```
    /// use brydz_core::karty::{card_set, cards::*, suits::Suit::*};
    /// use brydz_core::contract::Trick;
    /// use brydz_dd::actions::CardPack;
    /// use brydz_core::player::side::Side::North;
    /// let hand_no_hearts = card_set![TEN_SPADES, JACK_CLUBS, QUEEN_CLUBS, KING_DIAMONDS];
    /// let jack_pack = CardPack::group_single_card(&JACK_CLUBS);
    /// let hand_hearts = card_set![TEN_SPADES, JACK_CLUBS, QUEEN_CLUBS, KING_DIAMONDS, ACE_HEARTS];
    /// let mut trick = Trick::new(North);
    /// assert!(jack_pack.is_legal(&trick, &hand_hearts));
    /// trick.insert_card(North, KING_HEARTS);
    /// assert!(!jack_pack.is_legal(&trick, &hand_hearts));
    /// assert!(jack_pack.is_legal(&trick, &hand_no_hearts));
    /// ```
    /// 
    pub fn is_legal(&self, trick: &Trick, hand: &CardSet) -> bool{
        match trick.called_suit(){
            None => true,
            Some(this) if this == self.suit => true,
            Some(other) => !hand.contains_in_suit(&other)
        }
    }
    /*
    fn groups(side: Side, hands: &SideMap<StackHand>, trick: &Trick, trump: &TrumpGen<Suit>) -> Vec<FigureGroup>{
        let mut _result:Vec<FigureGroup> = Vec::new();
        todo!();
    }*/

    
}

impl Display for CardPack{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /*write!(f, "Group: [")?;
        for c in self.figures() {
            write!(f, "{:#}", Card::from_figure_and_suit(*c, self.suit))?;
        }
        write!(f, "]")
        */
        write!(f, "{{")?;
        match f.alternate(){
            true => {
                for i in 0..self.figures.len().saturating_sub(1){
                        write!(f, "{:#}, ", Card::from_figure_and_suit(self.figures[i], self.suit))?;
                }
                if let Some(last) = self.figures.last(){
                    write!(f, "{:#}", Card::from_figure_and_suit(*last, self.suit))?;
                }
            },
            false => {
                for i in 0..self.figures.len().saturating_sub(1){
                    write!(f, "{}, ", Card::from_figure_and_suit(self.figures[i], self.suit))?;
                }
                if let Some(last) = self.figures.last(){
                    write!(f, "{}", Card::from_figure_and_suit(*last, self.suit))?;
                }
            }
        }
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, Default)]
pub struct CardPackVec(pub SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]>);
pub type VCardPack = SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]>;

impl CardPackVec{
    pub fn new() -> Self{
        Self(SmallVec::new())
    }
}

impl IntoIterator for CardPackVec{
    type Item = <SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]> as IntoIterator>::Item;
    type IntoIter = <SmallVec<[CardPack; CONTRACT_ACTION_SPACE_BOUND]> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for CardPackVec{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        match f.alternate(){
            true => {
                for i in 0..self.0.len().saturating_sub(1){
                        write!(f, "{:#}, ", self.0[i])?;
                }
                if let Some(last) = self.0.last(){
                    write!(f, "{last:#}")?;
                }
            },
            false => {
                for i in 0..self.0.len().saturating_sub(1){
                    write!(f, "{}, ", self.0[i])?;
                }
                if let Some(last) = self.0.last(){
                    write!(f, "{last}")?;
                }
            }
        }
        write!(f, "]")
    }
}


impl Action for CardPack {
    //const SPACE_BOUND: usize = Card::CARD_SPACE/ PLAYER_NUM;
}






#[cfg(test)]
mod tests{
    use brydz_core::karty::{
        hand::{CardSet, HandTrait},
        cards::*,
        suits::Suit};
    use crate::actions::card_pack::CardPack;

    #[test]
    fn vec_no_grouping(){
        let mut hand = CardSet::empty();
        for c in [ACE_SPADES, KING_SPADES, QUEEN_SPADES, ACE_HEARTS, TEN_SPADES, JACK_SPADES, KING_HEARTS]{
        hand.insert_card(c).unwrap();
        }
        let spades_non_groups = CardPack::vec_no_grouping(&hand, &Suit::Spades);
        assert_eq!(spades_non_groups[4].lowest_card(), TEN_SPADES);
        assert_eq!(spades_non_groups[3].lowest_card(), JACK_SPADES);
        assert_eq!(spades_non_groups[2].lowest_card(), QUEEN_SPADES);
        assert_eq!(spades_non_groups[1].lowest_card(), KING_SPADES);
        assert_eq!(spades_non_groups[0].lowest_card(), ACE_SPADES);

    }
}