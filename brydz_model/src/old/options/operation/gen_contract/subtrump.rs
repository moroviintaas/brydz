use std::fmt::{Display, Formatter};
use clap::ValueEnum;


#[derive(ValueEnum, Clone, Debug)]
pub enum Subtrump{
    All,
    Colored,
    NoTrump
}

impl Display for Subtrump{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Subtrump::All => "all",
            Subtrump::Colored => "colored",
            Subtrump::NoTrump => "no-trump"
        })
    }
}