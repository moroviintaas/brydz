use std::fmt::{Display, Formatter};
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum HandInfoVariants{
    Simple,
    Suspect,
}

impl Display for HandInfoVariants{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::Simple => "simple",
            Self::Suspect => "suspect"
        })
    }
}