use std::collections::HashMap;
use std::hash::Hash;
use karty::suits::SuitTrait;
use crate::cards::trump::TrumpGen;
use crate::player::axis::Axis;
use crate::player::side::Side;

pub trait DeclarationStorage<S: SuitTrait>: Default{
    fn get_declarer(&self, axis: Axis, trump: &TrumpGen<S>) -> Option<&Side>;
    fn set_declarer(&mut self, side: Side, trump: TrumpGen<S>);
}

pub struct GeneralDeclarationStorage<S: SuitTrait + Hash>{
    east_west_declarations: HashMap<TrumpGen<S>, Side>,
    north_south_declarations: HashMap<TrumpGen<S>, Side>,
}

impl<S: SuitTrait + Hash > GeneralDeclarationStorage<S>{
    fn mut_declarations(&mut self, axis: Axis) -> &mut HashMap<TrumpGen<S>, Side>{
        match axis{
            Axis::EastWest => &mut self.east_west_declarations,
            Axis::NorthSouth => &mut self.north_south_declarations
        }
    }
    fn declarations(&self, axis: Axis) -> &HashMap<TrumpGen<S>, Side>{
        match axis{
            Axis::EastWest => & self.east_west_declarations,
            Axis::NorthSouth => & self.north_south_declarations
        }
    }
}

impl<S: SuitTrait + Hash> Default for GeneralDeclarationStorage<S> {
    fn default() -> Self {
        Self{north_south_declarations: HashMap::default(), east_west_declarations: HashMap::default()}
    }
}

impl<S: SuitTrait + Hash>  DeclarationStorage<S> for GeneralDeclarationStorage<S>{
    fn get_declarer(&self, axis: Axis, trump: &TrumpGen<S>) -> Option<&Side> {
        match self.declarations(axis).get(trump){
            None => None,
            Some(side) => Some(side)
        }
    }

    fn set_declarer(&mut self, side: Side, trump: TrumpGen<S>) {
        self.mut_declarations(side.axis()).insert(trump, side);
    }
}