use std::marker::PhantomData;
use karty::suits::{SuitTrait, Suit};
use crate::bidding::Doubling;
use crate::contract::ContractParametersGen;

pub struct PointsPremiumSport{
    pub game_vulnerable: i32,
    pub game_not_vulnerable: i32,
    pub partscore: i32,
    pub game_activation_level: i32
}

pub const POINTS_PREMIUM_SPORT: PointsPremiumSport = PointsPremiumSport{
    game_vulnerable: 500,
    game_not_vulnerable: 300,
    partscore: 50,
    game_activation_level: 100
};

impl PointsPremiumSport{

    pub fn points(&self, points_from_contract: i32, vulnerability: bool) -> i32{
        if points_from_contract >= self.game_activation_level{
            return match vulnerability{
                true => self.game_vulnerable,
                false => self.game_not_vulnerable
            }
        } else if points_from_contract > 0 {
            return self.partscore
        }
        0
    }
}


pub struct PointsPremiumContract<SU: SuitTrait>{
    pub on_doubled: i32,
    pub on_redoubled: i32,
    _phantom: PhantomData<SU>
}
pub type PointsPremiumContractStd = PointsPremiumContract<Suit>;

impl<SU: SuitTrait> PointsPremiumContract<SU>{

    pub fn points(&self, contract: &ContractParametersGen<SU>, taken: u8) -> i32{
        if taken >= contract.bid().number_normalised(){
            return match contract.doubling(){
                Doubling::None => 0,
                Doubling::Double => self.on_doubled,
                Doubling::Redouble => self.on_redoubled,
            }
        }
        0

    }
}

pub const POINTS_PREMIUM_CONTRACT: PointsPremiumContractStd = PointsPremiumContractStd {
    on_doubled: 50,
    on_redoubled: 100, _phantom: PhantomData };