use config::{Config, ConfigError, File};
use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize)]
pub enum Connection{
    Local
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerCfg {
    cards: String,
    connection: Connection
}

impl PlayerCfg{
    pub fn new(cards: String, connection: Connection) -> Self{
        Self{cards, connection}
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractConfig {
    north: PlayerCfg,
    south: PlayerCfg,
    east: PlayerCfg,
    west: PlayerCfg,
    contract_spec: String

}

impl ContractConfig {
    pub fn new_raw(north: PlayerCfg, east: PlayerCfg, south: PlayerCfg, west: PlayerCfg, contract_spec: String) -> Self{
        Self{north, east, south, west, contract_spec}
    }

    pub fn from_file(file_path: &str) -> Result<Self, ConfigError>{

        let s = Config::builder()
            .add_source(File::with_name(file_path))
            .build()?;
        s.try_deserialize()
    }
}