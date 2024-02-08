
pub mod test_ops;
//pub mod simulate_local;
pub mod train;
pub mod demo_op;
pub mod generate;


use clap::Subcommand;
use crate::options::operation::demo_op::DemoCommands;
//use crate::options::operation::simulate_local::SimContractOptions;
use crate::options::operation::train::sessions::AgentType;
use crate::options::operation::generate::GenerateSubcommand;

#[derive(Subcommand)]
pub enum Operation {

    //ContractGen(GenContractOptions),
    //LocalSimContract(SimContractOptions),
    #[command(subcommand, rename_all = "snake_case")]
    Generate(GenerateSubcommand),

    #[command(subcommand, rename_all = "snake_case")]
    Train(AgentType),
    #[command(subcommand, rename_all = "snake_case")]
    Demo(DemoCommands),
}