use std::fmt::{Display, Formatter};
use clap::{Subcommand, ValueEnum};
use amfiteatr_core::error::{AmfiteatrError, ConvertError};
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::player::side::Side;
use crate::options::bias_generation::BiasDistributionOptions;
use crate::options::contract_generation::GenContractOptions;

mod contract_generation;
mod bias_generation;
mod logger;

#[derive(ValueEnum)]
#[derive(Clone, Debug)]
pub enum DataFormat{
    Ron,
    //Yaml,
    //Json,
}

#[derive(ValueEnum)]
#[derive(Clone, Debug)]
pub enum DealMethod {
    Fair,
    Biased
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ChoiceDoubling{
    Any,
    No,
    Double,
    Redouble
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ForceDeclarer{
    DontForce,
    ForceNorth,
    ForceEast,
    ForceSouth,
    ForceWest
}

impl TryFrom<&ForceDeclarer> for Side{
    type Error = AmfiteatrError<ContractDP>;

    fn try_from(value: &ForceDeclarer) -> Result<Self, Self::Error> {
        match value{
            ForceDeclarer::DontForce => Err(AmfiteatrError::DataConvert(ConvertError::IllegalValue {
                value: "ForceDeclarer::DontForce".to_string(),
                context: "Can't be converted to karty::Side".to_string(),
            })),
            ForceDeclarer::ForceNorth => Ok(Side::North),
            ForceDeclarer::ForceEast => Ok(Side::East),
            ForceDeclarer::ForceSouth => Ok(Side::South),
            ForceDeclarer::ForceWest => Ok(Side::West)
        }
    }
}

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

#[derive(Subcommand)]
pub enum GenerateSubcommand{
    Contract(GenContractOptions),
    Distribution(BiasDistributionOptions)

}