use std::fmt::{Display, Formatter};
use karty::suits::{SuitTrait, Suit};
use crate::bidding::bid::Bid;

use crate::player::side::Side;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};



#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Doubling{
    None,
    Double,
    Redouble
}

#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum Call<S: SuitTrait> {
    NewBid(Bid<S>),
    Double,
    Redouble,
    Pass
}

pub type CallStd = Call<Suit>;

impl<S: SuitTrait + Display> Display for Call<S>{
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!();
        /*
        if f.alternate(){
            match self{
                Call::Bid(bid) => write!(f, "Call::Bid{{ {:#} }}", bid),

            }

        }
        else{
            match self{
                Call::Bid(bid) => write!(f, "Call::Bid{{ {} }}", bid),
            }
        }*/
    }
}

#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
pub struct CallEntry<S: SuitTrait>{
    player_side: Side,
    call: Call<S>
}


impl<S: SuitTrait> CallEntry<S> {
    pub fn new(player_side: Side, call: Call<S>) -> Self{
        Self{ player_side, call}
    }
    pub fn player_side(&self)-> Side{
        self.player_side
    }
    pub fn call(&self) -> &Call<S> {
        &self.call
    }
}








