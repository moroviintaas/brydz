use std::marker::PhantomData;
use karty::suits::{SuitTrait, Suit};
use crate::bidding::Doubling;
use crate::contract::ContractParametersGen;
use crate::error::{ScoreError};
use crate::meta::{SIZE_GREATER_HALF_TRICKS};


#[derive(Default)]
pub struct PenaltyTable{
    pub not_vulnerable: i32,
    pub vulnerable: i32,
    pub not_vulnerable_doubled: i32,
    pub vulnerable_doubled: i32,
    pub not_vulnerable_redoubled: i32,
    pub vulnerable_redoubled: i32,
}

impl PenaltyTable{
    pub fn get_points(&self, doubling: Doubling, vulnerability: bool) -> i32{
        match doubling{
            Doubling::None => match vulnerability{
                true => self.vulnerable,
                false => self.not_vulnerable
            }
            Doubling::Double => match vulnerability{
                true => self.vulnerable_doubled,
                false => self.not_vulnerable_doubled
            }
            Doubling::Redouble => match vulnerability{
                true => self.vulnerable_redoubled,
                false => self.not_vulnerable_redoubled
            }
        }
    }
}
pub const FIRST_TRICK_PENALTY: PenaltyTable = PenaltyTable{
    not_vulnerable: 50,
    vulnerable: 100,
    not_vulnerable_doubled: 100,
    vulnerable_doubled: 200,
    not_vulnerable_redoubled: 200,
    vulnerable_redoubled: 400
};
pub const LEVEL_2_TRICK_PENALTY: PenaltyTable = PenaltyTable{
    not_vulnerable: 50,
    vulnerable: 100,
    not_vulnerable_doubled: 200,
    vulnerable_doubled: 300,
    not_vulnerable_redoubled: 400,
    vulnerable_redoubled: 600
};
pub const LEVEL_3_TRICK_PENALTY: PenaltyTable = PenaltyTable{
    not_vulnerable: 50,
    vulnerable: 100,
    not_vulnerable_doubled: 300,
    vulnerable_doubled: 300,
    not_vulnerable_redoubled: 600,
    vulnerable_redoubled: 600
};

pub struct PenaltyUnderTrick<S: SuitTrait, const L: usize, >{

    pub penalty_tables: [PenaltyTable;L],
    _phantom: PhantomData<S>

    /*pub first_undertrick: PenaltyTable,
    pub following_first_undertricks: PenaltyTable,
    pub following_third_undertricks: PenaltyTable*/
}

impl<S: SuitTrait, const L: usize> PenaltyUnderTrick<S, L>{

    pub fn penalty_checked(&self, contract: &ContractParametersGen<S>, taken: u8, vulnerability: bool) -> Result<i32, ScoreError>{
        let number_of_undertricks = contract.bid().number_normalised().saturating_sub(taken);
        if (number_of_undertricks as usize) > self.penalty_tables.len(){
            return Err(ScoreError::NegativeTrickNumber)
        }
        let mut penalty = 0;
        for i in 0..number_of_undertricks as usize{
            penalty += self.penalty_tables[i].get_points(contract.doubling(), vulnerability);
        }
        Ok(penalty)


    }
}

pub const PENALTY_UNDER_TRICK: PenaltyUnderTrick<Suit, SIZE_GREATER_HALF_TRICKS> = PenaltyUnderTrick{
    penalty_tables: [
        FIRST_TRICK_PENALTY,
        LEVEL_2_TRICK_PENALTY,
        LEVEL_2_TRICK_PENALTY,
        LEVEL_3_TRICK_PENALTY,
        LEVEL_3_TRICK_PENALTY,
        LEVEL_3_TRICK_PENALTY,
        LEVEL_3_TRICK_PENALTY
    ],
    _phantom: PhantomData
};