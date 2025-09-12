use clap::{Subcommand, ValueEnum};

use crate::options::bias_generation::BiasDistributionOptions;
use crate::options::contract_generation::GenContractOptions;

pub mod contract_generation;
pub mod bias_generation;
pub mod logger;

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
/*

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
*/

#[derive(Subcommand)]
pub enum GenerateSubcommand{
    Contract(GenContractOptions),
    Distribution(BiasDistributionOptions)

}