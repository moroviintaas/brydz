use crate::error::DoubleDummyError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptimiserError{
    ErrorWhileCachingOnNewTrick,
    ErrorWhileCachingOnPartialTrickError,
    ErrorWhileCachingOnTrickDump
}

impl From<OptimiserError> for DoubleDummyError{
    fn from(value: OptimiserError) -> Self {
        Self::Optimiser(value)
    }
}