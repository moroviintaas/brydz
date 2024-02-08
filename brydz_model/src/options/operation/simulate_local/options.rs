use std::fs;
use std::path::PathBuf;
use clap::Args;
use crate::error::BrydzSimError;
use crate::SimContractParams;

#[derive(Args)]
pub struct SimContractOptions{

    #[arg(short = 'r', long = "repeat", help = "Repeat each contract a number of times", default_value = "1")]
    pub game_count: u16,
    #[arg(short = 'i', long = "input", help = "File with contracts to play")]
    pub input_file: Option<PathBuf>

}


#[allow(dead_code)]
pub fn sim2(gen_options: &SimContractOptions) -> Result<(), BrydzSimError>{
    match &gen_options.input_file{
        None => {todo!()}
        Some(f) => {
            let games_str = fs::read_to_string(f).unwrap();
            let games: Vec<SimContractParams> = ron::de::from_str(&games_str).unwrap();

            for _g in games{

            }
        }
    }
    Ok(())
}