use karty::cards::{Card2SymTrait};
use crate::contract::trick::{TrickGen};
use crate::player::side::Side;
use crate::player::axis::Axis;
use crate::contract::spec::ContractParametersGen;
use crate::error::ContractErrorGen;
use crate::player::role::PlayRole;
use crate::player::role::PlayRole::{Declarer, Dummy, Offside, Whist};


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

    fn side_by_role(&self, role: PlayRole) -> Side{
        match role{
            PlayRole::Whist => self.contract_spec().whist(),
            PlayRole::Declarer => self.contract_spec().declarer(),
            PlayRole::Offside => self.contract_spec().offside(),
            PlayRole::Dummy => self.contract_spec().dummy(),
        }
    }

    fn role_by_side(&self, side: Side) -> PlayRole{
        let x = side - self.contract_spec().declarer();
        match x{
            0 => Declarer,
            1 => Whist,
            2 => Dummy,
            3 => Offside,
            _ => panic!("Unfailable")
        }
    }

    fn total_tricks_taken_role(&self, role: PlayRole) -> u32{
        let side = self.side_by_role(role);
        self.total_tricks_taken_side(side)
    }
    fn total_tricks_taken_role_axis(&self, role: PlayRole) -> u32{
        let side = self.side_by_role(role);
        self.total_tricks_taken_axis(side.axis())
    }


}

