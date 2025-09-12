

use std::path::PathBuf;
use clap::Parser;
use log::{debug, error, LevelFilter};
use rand::{rng, Rng};
use ron::ser::{to_string_pretty, PrettyConfig};
use brydz_core::deal::{BiasedHandDistribution, DealDistribution};
use brydz_model::options::{DataFormat, GenerateSubcommand};
use std::io::Write;
use brydz_model::generate::generate_contracts;

#[derive(Parser)]
pub struct CliGenerationOptions {
    #[command(subcommand, rename_all = "snake_case")]
    pub command: GenerateSubcommand,
    #[arg(short = 'l', long = "log", default_value_t= LevelFilter::Info)]
    pub log_level: LevelFilter,
    #[arg(short = 'c', long = "log_core", default_value_t= LevelFilter::Error)]
    pub brydz_core_log_level: LevelFilter,
    #[arg(short = 'a', long = "log_amfi", default_value_t= LevelFilter::Error)]
    pub amfi_log_level: LevelFilter,
    #[arg(short = 'r', long = "log_amfi-rl", default_value_t= LevelFilter::Error)]
    pub amfiteatr_rl_log_level: LevelFilter,

    #[arg(long = "log_file")]
    pub log_file: Option<PathBuf>,

}
pub fn setup_logger(options: &CliGenerationOptions) -> Result<(), fern::InitError> {
    let dispatch  = fern::Dispatch::new()

        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        //.level(log_level)
        .level_for("brydz_model", options.log_level)
        .level_for("brydz_core", options.brydz_core_log_level)
        .level_for("amfiteatr_rl", options.amfiteatr_rl_log_level)
        .level_for("amfiteatr_core", options.amfi_log_level);

    match &options.log_file{
        None => dispatch.chain(std::io::stdout()),
        Some(f) => dispatch.chain(fern::log_file(f)?)
    }
        .apply()?;
    Ok(())
}

fn main() -> anyhow::Result<()>{

    let cli = CliGenerationOptions::parse();
    setup_logger(&cli)?;

    match cli.command {
        GenerateSubcommand::Contract(contract_options) => {
            debug!("Contract options: {:?}", contract_options);
            let contracts = generate_contracts(&contract_options)?;
            let data_str = match contract_options.format {
                DataFormat::Ron => {
                    let my_config = PrettyConfig::new()
                        .depth_limit(4)
                        // definitely superior (okay, just joking)
                        .indentor("\t".to_owned());
                    let ser = to_string_pretty(&contracts, my_config).inspect_err(|_e|{
                        error!("Error serializing generated biased distributions");
                    })?;
                    ser
                }
            };
            match &contract_options.output_file{
                None => {
                    println!("{}",  data_str);
                }
                Some(file) => {
                    let mut output = std::fs::File::create(file)?;
                    write!(output, "{}", data_str)?;
                }

            }

        }
        GenerateSubcommand::Distribution(distribution_options) => {
            debug!("Contract options: {:?}", distribution_options);
            let mut rng = rng();

            let generated = (0..distribution_options.distribution_count)
                .into_iter().map(|_|{
                let biased_distribution: BiasedHandDistribution = rng.random();

                DealDistribution::Biased(Box::new(biased_distribution))

            }).collect::<Vec<DealDistribution>>();


            let data_str = match distribution_options.format{
                DataFormat::Ron => {
                    let my_config = PrettyConfig::new()
                        .depth_limit(4)
                        // definitely superior (okay, just joking)
                        .indentor("\t".to_owned());
                    let ser = to_string_pretty(&generated, my_config).inspect_err(|_e|{
                        error!("Error serializing generated biased distributions");
                    })?;
                    ser

                }
            };

            match &distribution_options.distribution_output{
                None => {
                    println!("{}", data_str);
                }
                Some(file_path) => {
                    let mut output = std::fs::File::create(file_path).inspect_err(|_e|{
                        error!("Failed creating distribution output file: {file_path:?}");
                    })?;
                    write!(output, "{}", data_str).map_err(|e|{
                        error!("Failed writing serialisation to file");
                        e
                    })?;
                }
            }



            //anyhow::Result::Ok(());
        }
    }


    //let options =
    Ok(())
}