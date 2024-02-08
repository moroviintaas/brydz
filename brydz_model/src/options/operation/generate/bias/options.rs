use std::path::PathBuf;
use clap::Args;

#[derive(Args)]
pub struct BiasDistributionOptions{
    #[arg(short = 'n', long = "number", help = "Number of distributions to generate", default_value = "1")]
    pub distribution_count: u64,
    #[arg(short = 'o', long = "output", help = "File to save distributions")]
    pub distribution_output: Option<PathBuf>,
}

