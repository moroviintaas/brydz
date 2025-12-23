use std::path::PathBuf;
use clap::{Args, Subcommand};
use log::LevelFilter;
use brydz_model::options::contract::{AgentConfig, AgentPolicyInnerConfig, InformationSetRepresentation, ModelConfig, PolicyOuterConfig};
use clap::Parser;
use brydz_model::model::GameModel;


#[derive(Debug, Args)]
pub struct RunOptions{
    #[arg(short = 'f', long = "config", help = "File with model configuration")]
    pub config_path: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum ModelTask{


    Run(RunOptions),
    Default
}
impl Default for ModelTask{
    fn default() -> Self{
        ModelTask::Run(RunOptions{config_path: None})
    }
}

#[derive(Debug, Parser)]
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

    #[command(subcommand)]
    pub task: ModelTask


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

    let opt = RunCli::parse();
    setup_logger(&opt)?;




    match opt.task{
        ModelTask::Default => {
            let config  = ModelConfig::default();

            let s = serde_yaml::to_string(&config)?;
            println!("{}",s);
        }

        ModelTask::Run(run_config) => {
            let model_config  = match run_config.config_path{
                None => ModelConfig{
                    number_of_epochs: 100,
                    number_of_games_in_epoch: 100,
                    ..ModelConfig::default()
                },
                Some(path) => {
                    //let file = std::fs::File::open(path)?;
                    let s = std::fs::read_to_string(&path).map_err(|e|
                    anyhow::format_err!("Can't open config file {:?}", &path)
                    )?;
                    serde_yaml::from_str(&s)?
                }
            };

            let mut model = GameModel::try_from(model_config)?;

            model.run_session_own_trajectories()?;
        }
    }







     
    Ok(())


}