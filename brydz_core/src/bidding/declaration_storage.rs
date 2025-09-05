use std::collections::HashMap;
use std::hash::Hash;
use karty::suits::SuitTrait;
use crate::cards::trump::TrumpGen;
use crate::player::axis::Axis;
use crate::player::side::Side;

pub trait DeclarationStorage<SU: SuitTrait>: Default{
    fn get_declarer(&self, axis: Axis, trump: &TrumpGen<SU>) -> Option<&Side>;
    fn set_declarer(&mut self, side: Side, trump: TrumpGen<SU>);
}

pub struct GeneralDeclarationStorage<SU: SuitTrait + Hash>{
    east_west_declarations: HashMap<TrumpGen<SU>, Side>,
    north_south_declarations: HashMap<TrumpGen<SU>, Side>,
}

impl<SU: SuitTrait + Hash > GeneralDeclarationStorage<SU>{
    fn mut_declarations(&mut self, axis: Axis) -> &mut HashMap<TrumpGen<SU>, Side>{
        match axis{
            Axis::EastWest => &mut self.east_west_declarations,
            Axis::NorthSouth => &mut self.north_south_declarations
        }
    }
    fn declarations(&self, axis: Axis) -> &HashMap<TrumpGen<SU>, Side>{
        match axis{
            Axis::EastWest => & self.east_west_declarations,
            Axis::NorthSouth => & self.north_south_declarations
        }
    }
}

impl<SU: SuitTrait + Hash> Default for GeneralDeclarationStorage<SU> {
    fn default() -> Self {
        Self{north_south_declarations: HashMap::default(), east_west_declarations: HashMap::default()}
    }
}

impl<SU: SuitTrait + Hash>  DeclarationStorage<SU> for GeneralDeclarationStorage<SU>{
    fn get_declarer(&self, axis: Axis, trump: &TrumpGen<SU>) -> Option<&Side> {
        match self.declarations(axis).get(trump){
            None => None,
            Some(side) => Some(side)
        }
    }

    fn set_declarer(&mut self, side: Side, trump: TrumpGen<SU>) {
        self.mut_declarations(side.axis()).insert(trump, side);
    }
}