mod contract;
mod bias;

pub use contract::*;
pub use bias::*;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum GenerateSubcommand{
    Contract(GenContractOptions),
    Distribution(BiasDistributionOptions)

}