mod options;

use log::error;
use rand::{Rng, thread_rng};
use ron::ser::{PrettyConfig, to_string_pretty};
use brydz_core::deal::{BiasedHandDistribution, DealDistribution};
pub use options::*;
use crate::error::BrydzSimError;
use std::io::Write;


pub fn generate_biased_deal_distributions(number_of_distributions: u64) -> Vec<DealDistribution>{
    let mut result = Vec::with_capacity(number_of_distributions as usize);

    let mut rng  = thread_rng();
    for _ in 0..number_of_distributions{
        let biased_distribution: BiasedHandDistribution = rng.gen();
        result.push(DealDistribution::Biased(Box::new(biased_distribution)))
    }

    result

}

pub fn op_generate_biased_distributions(options: &BiasDistributionOptions) -> Result<(), BrydzSimError>{
    let generated = generate_biased_deal_distributions(options.distribution_count);
    let my_config = PrettyConfig::new()
        .depth_limit(4)
        // definitely superior (okay, just joking)
        .indentor("\t".to_owned());
    let ser = to_string_pretty(&generated, my_config).map_err(|e|{
        error!("Error serializing generated biased distributions");
        e
    })?;
    match &options.distribution_output{
        None => {
            println!("{}", ser);
        }
        Some(file_path) => {
            let mut output = std::fs::File::create(file_path).map_err(|e|{
                error!("Failed creating distribution output file: {file_path:?}");
                e
            })?;
            write!(output, "{}", ser).map_err(|e|{
                error!("Failed writing serialisation to file");
                e
            })?;
        }
    }

    Ok(())
}