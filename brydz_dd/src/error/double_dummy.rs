use std::fmt::{Display, Formatter};
use brydz_core::error::BridgeCoreError;
use crate::actions::CardPack;
use crate::error::OptimiserError;

#[derive(Debug, Clone, PartialEq)]
pub enum DoubleDummyError{
    Core(BridgeCoreError),
    EmptyPack(CardPack),
    Optimiser(OptimiserError)
}

impl Display for DoubleDummyError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<BridgeCoreError> for DoubleDummyError{
    fn from(e: BridgeCoreError) -> Self {
        Self::Core(e)
    }
}