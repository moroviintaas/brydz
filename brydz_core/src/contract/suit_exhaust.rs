use std::fmt::Debug;
use karty::symbol::CardSymbol;
use karty::register::{Register};
use karty::suits::{Suit};
use crate::player::side::{Side};


#[derive(Debug, Default, Clone, Copy)]
pub struct SuitExhaust {
    array: u16
}




impl Register<(Side, Suit)> for SuitExhaust {
    fn register(&mut self, element: (Side, Suit)) {
        self.array  |= 1u16 << (usize::from(element.0.index()*4) + element.1.usize_index());
    }

    fn unregister(&mut self, element: &(Side, Suit)) {
        let mask_neg  = 1u16 << (usize::from(element.0.index()*4) + element.1.usize_index());
        let mask = mask_neg ^ u16::MAX;
        self.array &= mask;
    }

    fn is_registered(&self, element: &(Side, Suit)) -> bool {
        !matches!(self.array & (1u16 << (usize::from(element.0.index()*4) + element.1.usize_index())), 0)
    }
}
