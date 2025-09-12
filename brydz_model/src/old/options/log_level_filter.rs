

use crate::options::CliOptions;

//#[derive(Args)]
//pub struct ArgLogLevel(LevelFilter);

pub fn setup_logger(options: &CliOptions) -> Result<(), fern::InitError> {
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

        //.chain(std::io::stdout())
        //.chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}