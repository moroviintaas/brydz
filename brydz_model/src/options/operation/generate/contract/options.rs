use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use clap::ValueEnum;
use brydz_core::player::side::Side;
use crate::error::BrydzModelError;
use crate::error::GenError::ConvForceDeclarerNoToSide;
use clap::Args;
use crate::options::operation::generate::DealMethod::Fair;

#[derive(ValueEnum, Clone, Debug)]
pub enum ChoiceDoubling{
    Any,
    No,
    Double,
    Redouble
}



#[derive(ValueEnum)]
#[derive(Clone, Debug)]
pub enum DealMethod {
    Fair,
    Biased
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
    type Error = BrydzModelError;

    fn try_from(value: &ForceDeclarer) -> Result<Self, Self::Error> {
        match value{
            ForceDeclarer::DontForce => Err(BrydzModelError::Gen(ConvForceDeclarerNoToSide)),
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


#[derive(Args)]
pub struct GenContractOptions {
    #[arg(short = 'g', long = "game_count", help = "Number of game parameters to generate", default_value = "1")]
    pub game_count: u64,
    #[arg(short = 'm', long = "method", value_enum,  help = "Probability method of distribution cards", default_value_t = DealMethod::Fair)]
    pub deal_method: DealMethod,
    #[arg(short = 'l', long = "lower_bound", help = "Minimal contract value", value_parser = clap::value_parser!(u8).range(1..=7), default_value = "1")]
    pub min_contract: u8,
    #[arg(short = 'u', long = "upper_bound", help = "Maximal contract value", value_parser = clap::value_parser!(u8).range(1..=7), default_value = "7")]
    pub max_contract: u8,
    #[arg(short = 'o', long = "output", help = "Path to output file")]
    pub output_file: Option<PathBuf>,
    #[arg(short = 'p', long = "probability_file", help = "Path to file with probabilities of cards")]
    pub probability_file: Option<PathBuf>,
    /*
    #[arg(short = 'n', long = "north_type", help = "Type of North's hand information set", default_value_t = HandInfoVariants::Simple)]
    pub north_hand_type: HandInfoVariants,
    #[arg(short = 'e', long = "east_type", help = "Type of East's hand information set", default_value_t = HandInfoVariants::Simple)]
    pub east_hand_type: HandInfoVariants,
    #[arg(short = 's', long = "south_type", help = "Type of South's hand information set", default_value_t = HandInfoVariants::Simple)]
    pub south_hand_type: HandInfoVariants,
    #[arg(short = 'w', long = "west_type", help = "Type of West's hand information set", default_value_t = HandInfoVariants::Simple)]
    pub west_hand_type: HandInfoVariants,

     */
    #[arg(short = 't', long = "trump_limit", help = "Subset of possible trumps", default_value_t = Subtrump::All, rename_all = "snake_case")]
    pub trump_limit: Subtrump,
    #[arg(short = 'f', long = "force_declarer", help = "Force one side to be declarer", default_value_t = ForceDeclarer::DontForce, value_enum)]
    pub force_declarer: ForceDeclarer,
    #[arg(short = 'd', long = "doubling", help = "Force one side to be declarer", default_value_t = ChoiceDoubling::No, value_enum)]
    pub choice_doubling: ChoiceDoubling,


}

impl Default for GenContractOptions{
    fn default() -> Self {
        Self{
            game_count: 1,
            deal_method: Fair,

            min_contract: 1,
            max_contract: 6,
            output_file: None,
            probability_file: None,
            trump_limit: Subtrump::All,
            force_declarer: ForceDeclarer::DontForce,
            choice_doubling: ChoiceDoubling::Any,
        }
    }
}