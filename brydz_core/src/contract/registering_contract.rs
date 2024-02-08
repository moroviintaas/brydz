use std::cmp::min;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::ops::Index;
use karty::cards::{Card2SymTrait, Card};
use karty::register::{Register, CardRegister};
use crate::cards::trump::TrumpGen;
use crate::contract::suit_exhaust::{SuitExhaust};
use crate::contract::spec::ContractParametersGen;
use crate::contract::maintainer::ContractMechanics;
use crate::contract::{SmartTrickSolver, TrickGen, TrickSolver};
use crate::error::{ContractErrorGen, TrickErrorGen};
use crate::error::ContractErrorGen::IndexedOverCurrentTrick;
use crate::error::TrickErrorGen::MissingCard;
use crate::meta::{MAX_INDEX_IN_DEAL, QUARTER_SIZE};
use crate::player::axis::Axis;
use crate::player::side::Side;

#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct ContractGen<Crd: Card2SymTrait, Um: Register<Crd>, Se:Register<(Side, Crd::Suit)>>{
    contract_spec: ContractParametersGen<Crd::Suit>,
    tricks: [TrickGen<Crd>; QUARTER_SIZE],
    completed_tricks_number: usize,
    exhaust_table: Se,
    current_trick: TrickGen<Crd>,
    used_cards_memory: Um,
    solver: SmartTrickSolver<Crd>

}

impl<Crd: Card2SymTrait,
    Um: Register<Crd>,
    Se:Register<(Side, Crd::Suit)>> ContractMechanics for ContractGen<Crd, Um, Se>{

    type Card = Crd;

    fn current_trick(&self) -> &TrickGen<Self::Card>{
        &self.current_trick
    }
    fn contract_spec(&self) -> &ContractParametersGen<Crd::Suit>{
        &self.contract_spec
    }
    fn count_completed_tricks(&self) -> usize{
        self.completed_tricks_number
    }
    /// Inserts card to current trick in contract. If trick is closed (contains a card from each side (4)) it is closed and added to array of completed tricks.
    /// # Returns:
    /// `Ok(())` if card has been successfully added
    /// `Err(DealError)` Hopefully an error describing problem
    ///
    /// # Examples:
    /// ```
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::bidding::Doubling;
    /// use brydz_core::contract::{ContractParametersGen};
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::contract::Contract;
    /// use brydz_core::error::ContractErrorGen;
    /// use brydz_core::player::side::Side;
    /// use std::str::FromStr;
    /// use brydz_core::contract::ContractMechanics;
    /// use brydz_core::player::axis::Axis;
    /// use brydz_core::error::TrickErrorGen;
    /// use brydz_core::contract::suit_exhaust::{SuitExhaust};
    /// use karty::figures::Figure;
    /// use karty::suits::Suit;
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    /// use karty::register::Register;
    /// let mut contract = Contract::new(
    ///     ContractParametersGen::new(Side::West, Bid::init(TrumpGen::Colored(Suit::Hearts), 1).unwrap(),));
    /// contract.insert_card(Side::North, KING_HEARTS).unwrap();
    /// assert_eq!(contract.current_trick().called_suit(), Some(Suit::Hearts));
    /// contract.insert_card(Side::East, ACE_HEARTS).unwrap();
    /// contract.insert_card(Side::South, TWO_CLUBS).unwrap();
    /// assert_eq!(contract.suits_exhausted().is_registered(&(Side::South, Suit::Hearts)),true );
    /// assert_eq!(contract.count_completed_tricks(), 0);
    /// let r = contract.insert_card(Side::West, SEVEN_HEARTS);
    /// assert_eq!(r.unwrap(), Side::East);
    /// assert_eq!(contract.count_completed_tricks(), 1);
    /// assert_eq!(contract.side_winning_trick(0).unwrap(), Side::East);
    /// let r = contract.insert_card(Side::East, TEN_HEARTS);
    /// assert_eq!(r.unwrap(), Side::South);
    /// let r = contract.insert_card(Side::South, JACK_HEARTS);
    /// assert_eq!(r, Err(ContractErrorGen::UsedExhaustedSuit(Side::South, Suit::Hearts)));
    /// let r = contract.insert_card(Side::South, TWO_CLUBS);
    ///
    /// assert_eq!(r, Err(ContractErrorGen::DuplicateCard(TWO_CLUBS)));
    ///
    /// ```
    fn insert_card(&mut self, side: Side, card: Crd) -> Result<Side, ContractErrorGen<Crd>>{
        if self.completed_tricks_number >= QUARTER_SIZE{
            return Err(ContractErrorGen::ContractFull);
        }

        if self.used_cards_memory.is_registered(&card){
            Err(ContractErrorGen::DuplicateCard(card))
        } else if self.exhaust_table.is_registered(&(side, card.suit())){
            Err(ContractErrorGen::UsedExhaustedSuit(side, card.suit()))
        } else {
            if let Some(called) = self.current_trick.called_suit(){
                if card.suit() != called{
                    self.exhaust_table.register((side, called));
                }

            }
            match self.current_trick.insert_card(side, card.clone()){
                Ok(4) => {
                    self.used_cards_memory.register(card);
                    //match self.current_trick.taker(&self.solver){
                    match self.solver.winner(&self.current_trick){
                        Ok(winner) => {
                            match self.complete_current_trick(){
                                Ok(()) => Ok(winner),
                                Err(e) => Err(e)
                            }
    
                        }
                        Err(e) => Err(ContractErrorGen::BadTrick( e))
                    }
                },
                Ok(_) => {
                    self.used_cards_memory.register(card);
                    Ok(side.next())
                }
                Err(e) => Err(ContractErrorGen::BadTrick( e))
            }
        }
    
        
    }
    
    fn is_completed(&self) -> bool{
        match self.completed_tricks_number {
            n if n < QUARTER_SIZE => false,
            QUARTER_SIZE => true,
            //Infallible, I guess
            _ => panic!("Number of tricks in contract should never ever exceed {QUARTER_SIZE}.")
        }
    }
    fn completed_tricks(&self) -> Vec<TrickGen<Crd>> {
        let mut r = Vec::new();
        for i in 0..self.completed_tricks_number{
            r.push(self.tricks[i].to_owned());
        }
        r
    }
    /// Counts tricks taken by `Side` (one agent)
    /// # Examples:
    /// ```
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::{ContractMechanics,Contract};
    /// use std::str::FromStr;
    /// use brydz_core::contract::{ContractParametersGen};
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::bidding::Doubling;
    /// use brydz_core::contract::suit_exhaust::{SuitExhaust};
    /// use brydz_core::player::side::Side::*;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut deal = Contract::new(ContractParametersGen::new(West, Bid::init(TrumpGen::Colored(Diamonds), 1).unwrap(),));
    ///
    /// deal.insert_card(North, JACK_SPADES).unwrap();
    /// deal.insert_card(East, TEN_SPADES).unwrap();
    /// deal.insert_card(South, FOUR_SPADES).unwrap();
    /// deal.insert_card(West, FIVE_DIAMONDS).unwrap(); //winner
    ///
    /// deal.insert_card(West, EIGHT_HEARTS).unwrap();
    /// deal.insert_card(North, JACK_DIAMONDS).unwrap(); //winner
    /// deal.insert_card(East, KING_HEARTS).unwrap();
    /// deal.insert_card(South, NINE_HEARTS).unwrap();
    ///
    /// deal.insert_card(North, ACE_CLUBS).unwrap(); //winner
    /// deal.insert_card(East, QUEEN_SPADES).unwrap();
    /// deal.insert_card(South, SEVEN_HEARTS).unwrap();
    /// deal.insert_card(West, FOUR_CLUBS).unwrap();
    /// assert_eq!(deal.total_tricks_taken_side(North), 2);
    /// assert_eq!(deal.total_tricks_taken_side(West), 1);
    /// assert_eq!(deal.total_tricks_taken_side(South), 0);
    /// assert_eq!(deal.total_tricks_taken_side(East), 0);
    /// ```
    fn total_tricks_taken_side(&self, side: Side) -> u32{
        self.tricks[0..self.completed_tricks_number].iter()
            .filter(|t| self.solver.winner(t).unwrap() == side).count() as u32
    }

    fn tricks_taken_side_in_n_first_tricks(&self, side: Side, n: usize) -> u32{
        let mut tricks_taken = 0u32;
        for i in 0..min(n, self.tricks.len()){
            if self.tricks[i].taker(&self.solver).unwrap() == side{
                tricks_taken += 1;
            }
        }
        tricks_taken
    }
    fn tricks_taken_axis_in_n_first_tricks(&self, axis: Axis, n: usize) -> u32 {
        let mut tricks_taken = 0u32;
        for i in 0..min(n, self.tricks.len()){
            if self.tricks[i].taker(&self.solver).unwrap().axis() == axis{
                tricks_taken += 1;
            }
        }
        tricks_taken
    }

    /// Counts tricks taken by `Side` (one agent)
    /// # Examples:
    /// ```
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::{ContractMechanics, Contract};
    /// use std::str::FromStr;
    /// use brydz_core::bidding::Doubling;
    /// use brydz_core::contract::{ContractParametersGen};
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::contract::suit_exhaust::{SuitExhaust};
    /// use brydz_core::player::axis::Axis;
    /// use brydz_core::player::side::Side::*;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    /// let mut deal = Contract::new(ContractParametersGen::new(West, Bid::init(TrumpGen::Colored(Diamonds), 1).unwrap(),));
    /// deal.insert_card(North, JACK_SPADES).unwrap();
    /// deal.insert_card(East, TEN_SPADES).unwrap();
    /// deal.insert_card(South, FOUR_SPADES).unwrap();
    /// deal.insert_card(West, FIVE_DIAMONDS).unwrap(); //winner
    ///
    /// deal.insert_card(West, EIGHT_HEARTS).unwrap();
    /// deal.insert_card(North, JACK_DIAMONDS).unwrap(); //winner
    /// deal.insert_card(East, KING_HEARTS).unwrap();
    /// deal.insert_card(South, NINE_HEARTS).unwrap();
    ///
    /// deal.insert_card(North, ACE_CLUBS).unwrap(); //winner
    /// deal.insert_card(East, QUEEN_SPADES).unwrap();
    /// deal.insert_card(South, SEVEN_HEARTS).unwrap();
    /// deal.insert_card(West, FOUR_CLUBS).unwrap();
    /// assert_eq!(deal.total_tricks_taken_axis(Axis::NorthSouth), 2);
    /// assert_eq!(deal.total_tricks_taken_axis(Axis::EastWest), 1);
    /// ```
    fn total_tricks_taken_axis(&self, axis: Axis) -> u32{
        self.tricks[0..self.completed_tricks_number].iter()
            .filter(|t| self.solver.winner(t).unwrap().axis() == axis).count() as u32
            //.filter(|t| t.taker(&self.solver).unwrap().axis() == axis).count()

    }

    /// ```
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::{Contract, ContractMechanics, ContractParametersGen};
    /// use brydz_core::player::side::Side::{East, North, South, West};
    /// use karty::cards::{ACE_SPADES,EIGHT_HEARTS, FIVE_DIAMONDS, FOUR_SPADES, JACK_SPADES, TEN_SPADES};
    /// use karty::suits::Suit::Diamonds;
    /// let mut contract = Contract::new(ContractParametersGen::new(West, Bid::init(TrumpGen::Colored(Diamonds), 1).unwrap(),));
    /// assert_eq!(contract.count_completed_tricks(), 0);
    /// contract.insert_card(North, JACK_SPADES).unwrap();
    /// contract.insert_card(East, TEN_SPADES).unwrap();
    /// contract.insert_card(South, FOUR_SPADES).unwrap();
    /// //assert_eq!(!contract.count_completed_tricks(), 0);
    /// contract.insert_card(West, FIVE_DIAMONDS).unwrap(); //winner
    /// assert!(contract.current_trick().is_empty());
    /// contract.insert_card(West, EIGHT_HEARTS).unwrap();
    /// assert_eq!(contract.count_completed_tricks(), 1);
    /// assert!(!contract.current_trick().is_empty());
    /// assert_eq!(contract.undo(), Ok(EIGHT_HEARTS));
    /// assert_eq!(contract.current_side(), West);
    /// assert!(contract.current_trick().is_empty());
    /// assert_eq!(contract.undo(), Ok(FIVE_DIAMONDS));
    /// assert_eq!(contract.current_side(), West);
    /// assert_eq!(contract.undo(), Ok(FOUR_SPADES));
    /// assert_eq!(contract.current_side(), South);
    /// assert_eq!(contract.undo(), Ok(TEN_SPADES));
    /// assert_eq!(contract.current_side(), East);
    /// assert_eq!(contract.undo(), Ok(JACK_SPADES));
    /// assert_eq!(contract.current_side(), North);
    /// contract.insert_card(North, JACK_SPADES).unwrap();
    /// contract.insert_card(East, TEN_SPADES).unwrap();
    /// contract.insert_card(South, ACE_SPADES).unwrap();
    /// assert_eq!(contract.undo(), Ok(ACE_SPADES));
    /// contract.insert_card(South, ACE_SPADES).unwrap();
    /// ```
    fn undo(&mut self) -> Result<Self::Card, ContractErrorGen<Self::Card>>{
        match self.current_trick.is_empty(){
            true => {
                match self.completed_tricks_number{
                    0 => Err(ContractErrorGen::UndoOnEmptyContract),
                    n => {
                        self.current_trick = mem::take(&mut self.tricks[n-1]);
                        match self.current_trick.undo(){
                            Some(card) => {
                                
                                self.used_cards_memory.unregister(&card);
                                self.exhaust_table.unregister(&(self.current_side(), card.suit()));
                                self.completed_tricks_number -= 1;
                                Ok(card)
                            },
                            None => Err(ContractErrorGen::UndoOnEmptyContract),
                        }
                    }
                }
                
                
                
               
            },
            false => match self.current_trick.undo(){
                Some(card) => {
                    self.used_cards_memory.unregister(&card);
                    self.exhaust_table.unregister(&(self.current_side(), card.suit()));
                    Ok(card)
                },
                None => Err(ContractErrorGen::BadTrick(TrickErrorGen::ImposibleUndo))
            }
        }
    }
}

impl<Card: Card2SymTrait, Um: Register<Card>, Se: Register<(Side, Card::Suit)>> ContractGen<Card, Um, Se>{
    pub fn new(contract_parameters: ContractParametersGen<Card::Suit>) -> Self{
        let first_player = contract_parameters.declarer().next();
        let trump = contract_parameters.bid().trump().to_owned();
        let mut tricks = <[TrickGen::<Card>; QUARTER_SIZE]>::default();
        tricks[0] = TrickGen::new(first_player);
        Self{
            contract_spec: contract_parameters, tricks, completed_tricks_number: 0,
            exhaust_table: Se::default(), current_trick: TrickGen::new(first_player), used_cards_memory: Um::default(),
            solver: SmartTrickSolver::new(trump)}
    }

    pub fn card_used(&self) -> &Um{
        &self.used_cards_memory
    }

    pub fn suits_exhausted(&self) -> &Se{
        &self.exhaust_table
    }


    fn complete_current_trick(&mut self) -> Result<(), ContractErrorGen<Card>>{
        match self.completed_tricks_number {
            n@0..=MAX_INDEX_IN_DEAL => match self.current_trick.missing_card(){
                Some(s) => Err(ContractErrorGen::BadTrick( MissingCard(s))),
                None => {/* 
                    if let Some(c) = self.used_cards_memory.trick_collision(&self.current_trick){
                        return Err(ContractErrorGen::DuplicateCard(c));
                    }*/
                    //let next_player = self.current_trick.taker(self.trump()).unwrap();
                    let next_player = self.solver.winner(&self.current_trick).unwrap();

                    //self.used_cards_memory.mark_cards_of_trick(&self.current_trick);
                    self.tricks[n] = mem::replace(&mut self.current_trick, TrickGen::new(next_player));

                    self.completed_tricks_number = n+1;
                    Ok(())
                }

            }

            _ => Err(ContractErrorGen::ContractFull),
        }
    }


    pub fn trump(&self) -> &TrumpGen<Card::Suit>{
        self.contract_spec.bid().trump()
    }
    pub fn last_completed_trick(&self) -> Option<&TrickGen<Card>>{
        match self.completed_tricks_number {
            0 => None,
            i @1..=QUARTER_SIZE => Some(&self[i-1]),
            _ => panic!("Deal::Last_trick: contract overflow shouldn't happen")

        }
    }

    pub fn init_new_trick(&self) -> Option<TrickGen<Card>>{
        //println!("{:?}", self.trump());
        match self.last_completed_trick(){
            None => Some(TrickGen::new(self.contract_spec.declarer().prev())),

            //Some(t) => t.taker(self.trump()).ok().map(|s| TrickGen::new(s))
            Some(t) => self.solver.winner(t).ok()
                .map(|s| TrickGen::new(s))
        }

    }


    /// Based on index of trick returns the side who won the trick.
    /// # Examples:
    /// ```
    /// use brydz_core::player::side::Side::*;
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::cards::deck::Deck;
    /// use brydz_core::player::side::SIDES;
    /// use brydz_core::contract::{ContractMechanics,Contract};
    /// use std::str::FromStr;
    /// use brydz_core::bidding::Doubling;
    /// use brydz_core::contract::{ContractParametersGen};
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::contract::suit_exhaust::{SuitExhaust};
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    /// let deck = Deck::new_sorted_by_figures();
    /// let mut deal_1 = Contract::new(ContractParametersGen::new_d(North, Bid::init(TrumpGen::Colored(Diamonds), 1).unwrap(), Doubling::None));
    ///
    /// deal_1.insert_card(East, KING_SPADES).unwrap();
    /// deal_1.insert_card(South, QUEEN_SPADES).unwrap();
    /// deal_1.insert_card(West, JACK_SPADES).unwrap();
    /// deal_1.insert_card(North, ACE_SPADES).unwrap();
    /// assert_eq!(deal_1.side_winning_trick(0), Ok(North));
    /// deal_1.insert_card(North, ACE_HEARTS).unwrap();
    /// deal_1.insert_card(East, KING_HEARTS).unwrap();
    /// deal_1.insert_card(South, QUEEN_HEARTS).unwrap();
    /// deal_1.insert_card(West, TWO_DIAMONDS).unwrap();
    /// assert_eq!(deal_1.side_winning_trick(1), Ok(West));
    ///
    /// let mut deal_2 = Contract::new(ContractParametersGen::new_d(West, Bid::init(TrumpGen::NoTrump, 1u8).unwrap(), Doubling::None));
    ///
    /// deal_2.insert_card(North, TWO_DIAMONDS).unwrap();
    /// deal_2.insert_card(East, THREE_DIAMONDS).unwrap();
    /// deal_2.insert_card(South, ACE_SPADES).unwrap();
    /// deal_2.insert_card(West, SIX_DIAMONDS).unwrap();
    /// assert_eq!(deal_2.side_winning_trick(0), Ok(West));
    /// deal_2.insert_card(West, FOUR_DIAMONDS).unwrap();
    /// deal_2.insert_card(North, KING_CLUBS).unwrap();
    /// deal_2.insert_card(East, FIVE_DIAMONDS).unwrap();
    /// deal_2.insert_card(South, NINE_SPADES).unwrap();
    /// //deal_2.insert_trick(trick_2_2).unwrap();
    /// assert_eq!(deal_2.side_winning_trick(1), Ok(East));
    ///
    /// ```
    pub fn side_winning_trick(&self, index: usize) -> Result<Side, ContractErrorGen<Card>>{
        match index < self.completed_tricks_number {
            /*true => self[index].taker(self.contract_spec.bid().trump())
                .map_err(|trick_err| ContractErrorGen::BadTrick(trick_err)),

             */
            true => self.solver.winner(&self[index])
                .map_err(|trick_err| ContractErrorGen::BadTrick(trick_err)),
            false => Err(IndexedOverCurrentTrick(self.completed_tricks_number))
        }
    }

    pub fn used_cards(&self) -> &Um{
        &self.used_cards_memory
    }

    /// ```
    /// use brydz_core::bidding::{Bid, Doubling};
    /// use brydz_core::cards::deck::Deck;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::{*};
    /// use brydz_core::player::side::Side::*;
    /// use karty::cards::*;
    /// use karty::suits::Suit::*;
    /// let deck = Deck::new_sorted_by_figures();
    /// let mut deal_1 = Contract::new(ContractParametersGen::new_d(North, Bid::init(TrumpGen::Colored(Diamonds), 1).unwrap(), Doubling::None));
    ///
    /// deal_1.insert_card(East, KING_SPADES).unwrap();
    /// deal_1.insert_card(South, QUEEN_SPADES).unwrap();
    /// deal_1.insert_card(West, JACK_SPADES).unwrap();
    /// deal_1.insert_card(North, TWO_HEARTS).unwrap();
    /// assert!(deal_1.side_possibly_has_card(West,&TEN_SPADES));
    /// assert!(!deal_1.side_possibly_has_card(West,&KING_SPADES));
    /// assert!(!deal_1.side_possibly_has_card(North,&TWO_SPADES));
    /// ```
    pub fn side_possibly_has_card(&self, side: Side, card: &Card) -> bool{
        if self.used_cards_memory.is_registered(card){
            return false;
        }
        if self.exhaust_table.is_registered(&(side, card.suit())){
            return false
        }
        true
    }





}

impl<Card: Card2SymTrait,
    Um: Register<Card> + Debug,
    Se: Register<(Side,Card::Suit)> + std::fmt::Debug>  Display for ContractGen<Card, Um, Se>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}



impl<Card: Card2SymTrait, Um: Register<Card>, Se: Register<(Side, Card::Suit)>> Index<usize> for ContractGen<Card, Um, Se>{
    type Output = TrickGen<Card>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tricks[index]
    }
}

pub type Contract = ContractGen<Card, CardRegister, SuitExhaust>;

#[cfg(test)]
mod tests{
    use karty::cards::{*};
    use karty::register::CardRegister;
    use karty::suits::Suit::Diamonds;
    use crate::cards::trump::TrumpGen;
    use crate::contract::spec::{ContractParametersGen};
    use crate::bidding::Bid;
    use crate::contract::suit_exhaust::SuitExhaust;
    use crate::cards::deck::{Deck};
    use crate::contract::{ContractGen, ContractMechanics};
    use crate::error::ContractErrorGen;
    use crate::error::ContractErrorGen::ContractFull;
    use crate::meta::QUARTER_SIZE;
    use crate::player::side::Side;
    use crate::player::side::Side::{East, North, South, West};


    #[test]
    fn deal_duplicate_card(){
        let mut deal = ContractGen::<Card, CardRegister, SuitExhaust>::new(ContractParametersGen::new(West, Bid::init(TrumpGen::NoTrump, 1).unwrap(), ));
        //let deck = Deck::new_sorted_by_suits();



        deal.insert_card(Side::North, ACE_SPADES).unwrap();
        deal.insert_card(Side::East, QUEEN_SPADES).unwrap();
        deal.insert_card(Side::South, KING_SPADES).unwrap();
        deal.insert_card(Side::West, JACK_SPADES).unwrap();

        let r = deal.insert_card(Side::North, ACE_SPADES);

        assert_eq!(r, Err(ContractErrorGen::DuplicateCard(ACE_SPADES)));

    }


    #[test]
    fn deal_overflow_tricks(){
        let num_of_sides = 4usize;
        let deck = Deck::new_sorted_by_suits();
        //let mut contract = Deal::new(South, Trump::NoTrump);
        let mut deal = ContractGen::<Card, CardRegister, SuitExhaust>::new(
            ContractParametersGen::new(
                West, Bid::init(TrumpGen::NoTrump, 1).unwrap(), ));
        for i in 0..QUARTER_SIZE{

            deal.insert_card(Side::North,deck[num_of_sides*i].clone()).unwrap();
            deal.insert_card(Side::East,deck[num_of_sides*i + 1].clone()).unwrap();
            deal.insert_card(Side::South,deck[num_of_sides*i + 2].clone()).unwrap();
            deal.insert_card(Side::West,deck[num_of_sides*i +3].clone()).unwrap();

        }

        let r = deal.insert_card(Side::North, deck[0]);
        assert_eq!(r, Err(ContractFull));



    }

    #[test]
    fn calculate_score_1(){
        let mut deal = ContractGen::<Card, CardRegister, SuitExhaust>::new(ContractParametersGen::new(
            East,
            Bid::init(TrumpGen::Colored(Diamonds), 3).unwrap(),
        ));
        deal.insert_card(South, ACE_SPADES).unwrap();
        deal.insert_card(West, THREE_SPADES).unwrap();
        deal.insert_card(North, FOUR_SPADES).unwrap();
        deal.insert_card(East, SIX_SPADES).unwrap();
        assert_eq!(deal.count_completed_tricks(), 1);

        deal.insert_card(South, TWO_SPADES).unwrap();
        deal.insert_card(West, THREE_DIAMONDS).unwrap();
        deal.insert_card(North, EIGHT_SPADES).unwrap();
        deal.insert_card(East, TEN_SPADES).unwrap();
        assert_eq!(deal.count_completed_tricks(), 2);

        deal.insert_card(West, FOUR_DIAMONDS).unwrap();
        deal.insert_card(North, JACK_DIAMONDS).unwrap();
        deal.insert_card(East, QUEEN_DIAMONDS).unwrap();
        deal.insert_card(South, TEN_DIAMONDS).unwrap();

        deal.insert_card(East, KING_DIAMONDS).unwrap();
        deal.insert_card(South, FIVE_SPADES).unwrap();
        deal.insert_card(West, FIVE_DIAMONDS).unwrap();
        deal.insert_card(North, NINE_DIAMONDS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 4);

        deal.insert_card(East, FOUR_HEARTS).unwrap();
        deal.insert_card(South, FIVE_HEARTS).unwrap();
        deal.insert_card(West, KING_HEARTS).unwrap();
        deal.insert_card(North, ACE_HEARTS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 5);

        deal.insert_card(North, FIVE_CLUBS).unwrap();
        deal.insert_card(East, ACE_CLUBS).unwrap();
        deal.insert_card(South, FOUR_CLUBS).unwrap();
        deal.insert_card(West, TWO_CLUBS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 6);

        deal.insert_card(East, QUEEN_HEARTS).unwrap();
        deal.insert_card(South, EIGHT_HEARTS).unwrap();
        deal.insert_card(West, THREE_HEARTS).unwrap();
        deal.insert_card(North, TWO_HEARTS).unwrap();

        deal.insert_card(East, QUEEN_SPADES).unwrap();
        deal.insert_card(South, SEVEN_SPADES).unwrap();
        deal.insert_card(West, SIX_DIAMONDS).unwrap();
        deal.insert_card(North, NINE_SPADES).unwrap();
        assert_eq!(deal.count_completed_tricks(), 8);

        deal.insert_card(West, KING_CLUBS).unwrap();
        deal.insert_card(North, NINE_CLUBS).unwrap();
        deal.insert_card(East, TEN_HEARTS).unwrap();
        deal.insert_card(South, SIX_CLUBS).unwrap();

        deal.insert_card(West, THREE_CLUBS).unwrap();
        deal.insert_card(North, QUEEN_CLUBS).unwrap();
        deal.insert_card(East, TWO_DIAMONDS).unwrap();
        deal.insert_card(South, SEVEN_CLUBS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 10);

        deal.insert_card(East, SEVEN_DIAMONDS).unwrap();
        deal.insert_card(South, EIGHT_CLUBS).unwrap();
        deal.insert_card(West, SIX_HEARTS).unwrap();
        deal.insert_card(North, SEVEN_HEARTS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 11);

        deal.insert_card(East, EIGHT_DIAMONDS).unwrap();
        deal.insert_card(South, JACK_CLUBS).unwrap();
        deal.insert_card(West, NINE_HEARTS).unwrap();
        deal.insert_card(North, JACK_SPADES).unwrap();
        assert_eq!(deal.count_completed_tricks(), 12);

        deal.insert_card(East, ACE_DIAMONDS).unwrap();
        deal.insert_card(South, JACK_HEARTS).unwrap();
        deal.insert_card(West, TEN_CLUBS).unwrap();
        deal.insert_card(North, KING_SPADES).unwrap();


        //assert_eq!(contract.completed_tricks(), 13);
        assert_eq!(deal.total_tricks_taken_side(East), 8);
        assert_eq!(deal.total_tricks_taken_side(South), 1);
        assert_eq!(deal.total_tricks_taken_side(West), 3);
        assert_eq!(deal.total_tricks_taken_side(North), 1);


    }





}