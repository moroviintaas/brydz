use std::path::PathBuf;
use clap::Args;
use log::LevelFilter;
use brydz_model::options::contract::ModelConfig;

#[derive(Debug, Args)]
pub struct RunCli{

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

    #[arg(short = 'f', long = "config", help = "File with model configuration")]
    pub config_path: Option<PathBuf>,
}

pub fn setup_logger(options: &RunCli) -> Result<(), fern::InitError> {
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
fn main() -> anyhow::Result<()> {

    let config  = ModelConfig::default();

    let s = serde_yaml::to_string(&config)?;

    println!("{}",s);
    Ok(())
}