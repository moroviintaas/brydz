use karty::cards::{Card2SymTrait};
use crate::contract::trick::{TrickGen};
use crate::player::side::Side;
use crate::player::axis::Axis;
use crate::contract::spec::ContractParametersGen;
use crate::error::ContractErrorGen;


pub trait ContractMechanics {
    type Card: Card2SymTrait;
    
    fn current_trick(&self) -> &TrickGen<Self::Card>;
    fn contract_spec(&self) -> &ContractParametersGen<<Self::Card as Card2SymTrait>::Suit>;
    fn count_completed_tricks(&self) -> usize;
    fn insert_card(&mut self, side: Side, card: Self::Card) -> Result<Side, ContractErrorGen<Self::Card>>;
    fn is_completed(&self) -> bool;
    fn completed_tricks(&self) -> Vec<TrickGen<Self::Card>>;
    fn total_tricks_taken_side(&self, side: Side) -> u32;
    fn tricks_taken_side_in_n_first_tricks(&self, side: Side, n: usize) -> u32;
    fn tricks_taken_axis_in_n_first_tricks(&self, axis: Axis, n: usize) -> u32;
    fn total_tricks_taken_axis(&self, axis: Axis) -> u32;
    fn current_side(&self) -> Side{
        self.current_trick().current_side().unwrap()
    }
    fn declarer(&self) -> Side{
        self.contract_spec().declarer()
    }
    fn dummy(&self) -> Side{
        self.contract_spec().declarer().partner()
    }
    fn undo(&mut self) -> Result<Self::Card, ContractErrorGen<Self::Card>>;


}

