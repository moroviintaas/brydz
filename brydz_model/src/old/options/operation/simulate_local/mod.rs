mod options;

use std::fs;
pub use options::*;
use crate::error::BrydzSimError;
use crate::SimContractParams;

/*
pub fn sim2(gen_options: &SimContractOptions) -> Result<(), BrydzSimError>{
    match &gen_options.input_file{
        None => {todo!()}
        Some(f) => {
            let games_str = fs::read_to_string(f).unwrap();
            let games: Vec<SimContractParams> = ron::de::from_str(&games_str).unwrap();

            for g in games{
                for _ in 0..gen_options.game_count{
                    let mut model = generate_local_model(&g)?;
                    model.play()?
                }
            }
        }
    }
    Ok(())
}

 */