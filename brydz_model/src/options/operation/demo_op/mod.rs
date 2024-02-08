mod local;
mod tcp;
mod generic;
//mod run_nn;
mod biased_params;
mod biased_sample;


use clap::Subcommand;

pub use local::*;
pub use tcp::*;
pub use generic::*;
//pub use run_nn::*;
pub use biased_params::*;
pub use biased_sample::*;

#[derive(Subcommand)]
pub enum DemoCommands {
    Local,
    Tcp,
    Generic,
    //RunNN,
    BiasedParams,
    BiasedSample,
}