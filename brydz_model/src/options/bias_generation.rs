use std::path::PathBuf;
use clap::Args;
use crate::options::DataFormat;

#[derive(Debug, Args)]
pub struct BiasDistributionOptions{
    #[arg(short = 'n', long = "number", help = "Number of distributions to generate", default_value = "1")]
    pub distribution_count: u64,
    #[arg(short = 'o', long = "output", help = "File to save distributions")]
    pub distribution_output: Option<PathBuf>,

    #[arg(short = 'F', long = "format", help = "Generated biased distributions format", default_value = "ron")]
    pub format: DataFormat,
}
