use karty::cards::Card2SymTrait;

use karty::suits::SuitTrait;
use crate::contract::{ContractParametersGen, ContractMechanics};
use crate::error::BridgeCoreErrorGen;
use crate::player::axis::Axis;

pub trait ScoreTracker<Co: ContractMechanics<Card = Crd>, Crd: Card2SymTrait>: Default{
    fn winner_axis(&self) -> Option<Axis>;
    fn update(&mut self, deal: &Co) -> Result<(), BridgeCoreErrorGen<Crd>>;
    fn points(&self, axis: &Axis) -> i32;
}

pub trait ScoreIngredient<S: SuitTrait>{
    fn calculate(&self, contract: &ContractParametersGen<S>, taken: u8, vulnerability: bool) -> i32;
}