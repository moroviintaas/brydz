use brydz_core::error::Mismatch;
use brydz_core::karty::suits::Suit;

#[derive(Clone, Copy, Debug)]
pub enum GroupingError{
    SuitMismatch(Mismatch<Suit>),
    CachingNewDuringTrick
}