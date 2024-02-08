use brydz_model::settings::{ContractConfig, PlayerCfg};
use brydz_model::settings::Connection::Local;
use std::str::FromStr;


use clap::Parser;






use brydz_model::error::BrydzSimError;
use brydz_model::{
    options};
use brydz_model::options::operation::{Operation,
};
use brydz_model::options::operation::demo_op::{test_sample_biased_deal_crossing, test_sample_biased_deal_single, test_sample_biased_distribution_parameters, DemoCommands};
use brydz_model::options::operation::generate::{gen2, GenerateSubcommand, op_generate_biased_distributions};
//use brydz_model::options::operation::simulate_local::sim2;
use brydz_model::options::operation::train::sessions::build_and_run_train_session;


//use crate::options::operation::{GenContract, Operation};
//mod error;
//mod options;
//mod error;


#[allow(dead_code)]
fn serialize_settings_toml(){
    let sim_conf = ContractConfig::new_raw(
        PlayerCfg::new(String::from_str("AQT32.JT94.76.QT").unwrap(), Local),
        PlayerCfg::new(String::from_str("J97.Q875.AQT94.K").unwrap(), Local),
        PlayerCfg::new(String::from_str("K8.AK32.82.J9532").unwrap(), Local),
        PlayerCfg::new(String::from_str("654.6.KJ53.A8764").unwrap(), Local),
        String::from_str("2S").unwrap(),


    );

    let toml = toml::to_string(&sim_conf).unwrap();
    println!("{}", toml);
}

fn main() -> Result<(), BrydzSimError> {

    let cli = options::CliOptions::parse();
    options::setup_logger(&cli).unwrap();
    //serialize_settings_toml();
    match &cli.command{
        //Operation::ContractGen(gen_options) => gen2(gen_options),
        Operation::Generate(subcommand) => match subcommand{
            GenerateSubcommand::Contract(options) => gen2(options),
            GenerateSubcommand::Distribution(options) => op_generate_biased_distributions(options)
        }

        /*
        Operation::LocalSimContract(options) => {
            sim2(options)
        }//sim2(options)}


         */
        Operation::Train(agent_type) => {
            Ok(build_and_run_train_session(agent_type)?)



        },


        Operation::Demo(command) => {
            match command{
                DemoCommands::Local =>{
                    options::operation::demo_op::tur_sim();
                    Ok(())
                }
                DemoCommands::Tcp => {
                    options::operation::demo_op::tur_sim_tcp();
                    Ok(())
                }
                DemoCommands::Generic => {
                    match options::operation::demo_op::test_generic_model(){
                        Ok(_) => Ok(()),
                        Err(e) => Err(BrydzSimError::Custom(format!("{e:}")))
                    }
                },
                /*
                DemoCommands::RunNN => {
                    options::operation::demo_op::test_with_untrained_network()?;
                    Ok(())
                },

                 */
                DemoCommands::BiasedParams => {
                    Ok(test_sample_biased_distribution_parameters()?)
                },
                DemoCommands::BiasedSample => {
                    test_sample_biased_deal_crossing()?;
                    test_sample_biased_deal_single()?;
                    Ok(())
                }
            }
        }
    }







    //
}
