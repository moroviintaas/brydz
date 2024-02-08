use crate::player::side::Side;

pub trait StateWithSide {
    fn id(&self) -> Side;
}