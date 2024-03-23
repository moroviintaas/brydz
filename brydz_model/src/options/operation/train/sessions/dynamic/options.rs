use std::path::PathBuf;
use clap::{Args, ValueEnum};
use log::warn;
use serde::{Deserialize, Serialize};
use amfiteatr_rl::tch::nn::Adam;
use crate::options::operation::train::{DeviceSelect, InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::AgentConfiguration;

#[derive(ValueEnum, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum PolicyTypeSelect{
    Q,
    A2C
}
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct AdamParams{
    pub beta1: f64,
    pub beta2: f64,
    pub wd: f64,
    pub eps: f64,
    pub amsgrad: bool,
}

impl From<AdamParams> for Adam{
    fn from(value: AdamParams) -> Self {
        Adam{
            beta1: value.beta1,
            beta2: value.beta2,
            wd: value.wd,
            eps: value.eps,
            amsgrad: value.amsgrad,
        }
    }
}
impl From<Adam> for AdamParams{
    fn from(value: Adam) -> Self {
        AdamParams{
            beta1: value.beta1,
            beta2: value.beta2,
            wd: value.wd,
            eps: value.eps,
            amsgrad: value.amsgrad,
        }
    }
}

impl Default for AdamParams{
    fn default() -> Self {
        Adam::default().into()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PolicyParams{
    pub hidden_layers: Vec<i64>,
    pub optimizer_params: AdamParams,
    pub select_policy: PolicyTypeSelect,
    pub learning_rate: f64,

}

impl Default for PolicyParams{
    fn default() -> Self {
        Self{
            hidden_layers: vec![1024,512],

            optimizer_params: Default::default(),
            select_policy: PolicyTypeSelect::Q,
            learning_rate: 0.0001,
        }
    }
}


#[derive(Args, Clone)]
pub struct DynamicModelOptions{
    /*
    #[arg(short = 'd', long = "declarer_save", help = "Declarer VarStore save file")]
    pub declarer_save: Option<PathBuf>,
    #[arg(short = 'w', long = "whist_save", help = "Whist VarStore save file")]
    pub whist_save: Option<PathBuf>,
    #[arg(short = 'o', long = "offside_save", help = "Offside VarStore save file")]
    pub offside_save: Option<PathBuf>,

    #[arg(long = "declarer_iset", default_value = "simple", help = "Declarer's information set type")]
    pub declarer_is_type: InfoSetTypeSelect,
    #[arg(long = "whist_iset", default_value = "simple", help = "Whists's information set type")]
    pub whist_is_type: InfoSetTypeSelect,
    #[arg(long = "offside_iset", default_value = "simple", help = "Offside's information set type")]
    pub offside_is_type: InfoSetTypeSelect,

    #[arg(long = "test_declarer_iset", default_value = "complete", help = "Test Declarer's information set type")]
    pub test_declarer_is_type: InfoSetTypeSelect,
    #[arg(long = "test_whist_iset", default_value = "complete", help = "Test Whists's information set type")]
    pub test_whist_is_type: InfoSetTypeSelect,
    #[arg(long = "test_offside_iset", default_value = "complete", help = "Test Offside's information set type")]
    pub test_offside_is_type: InfoSetTypeSelect,

    #[arg(long = "declarer_tensor", default_value = "sparse", help = "Declarer's information set conversion type")]
    pub declarer_tensor: InfoSetWayToTensorSelect,
    #[arg(long = "whist_tensor", default_value = "sparse", help = "Whists's information set conversion type")]
    pub whist_tensor: InfoSetWayToTensorSelect,
    #[arg(long = "offside_tensor", default_value = "sparse", help = "Offside's information set conversion type")]
    pub offside_tensor: InfoSetWayToTensorSelect,

    #[arg(long = "test_declarer_tensor", default_value = "sparse", help = "Test Declarer's information set conversion type")]
    pub test_declarer_tensor: InfoSetWayToTensorSelect,
    #[arg(long = "test_whist_tensor", default_value = "sparse", help = "Test Whists's information set conversion type")]
    pub test_whist_tensor: InfoSetWayToTensorSelect,
    #[arg(long = "test_offside_tensor", default_value = "sparse", help = "Test Offside's information set conversion type")]
    pub test_offside_tensor: InfoSetWayToTensorSelect,


    #[arg(short = 'D', long = "declarer_load", help = "Declarer VarStore load file")]
    pub declarer_load: Option<PathBuf>,
    #[arg(short = 'W', long = "whist_load", help = "Whist VarStore load file")]
    pub whist_load: Option<PathBuf>,
    #[arg(short = 'O', long = "offside_load", help = "Offside VarStore load file")]
    pub offside_load: Option<PathBuf>,

    #[arg(long = "test_declarer_load", help = "Test Declarer VarStore load file")]
    pub test_declarer_load: Option<PathBuf>,
    #[arg(long = "test_whist_load", help = "Test Whist VarStore load file")]
    pub test_whist_load: Option<PathBuf>,
    #[arg(long = "test_offside_load", help = "Test Offside VarStore load file")]
    pub test_offside_load: Option<PathBuf>,



    #[arg(short = 'e', long = "epochs", help = "Number of epochs", default_value = "10")]
    pub epochs: u32,

    #[arg(short = 'n', long = "games", help = "games in epoch", default_value = "100")]
    pub games: u32,

    #[arg(short = 't', long = "tests", help = "test_set_number", default_value = "100")]
    pub tests_set_size: u32,

    #[arg(long = "device", help = "Device to be used", default_value = "cpu")]
    pub device: DeviceSelect,

    #[arg(short = 'T', long = "test_set", help = "Pre-generated set of contracts with cards distribution and deal for tests")]
    pub test_set: Option<PathBuf>,

     */

    //#[arg(short = 'l', long = "layers",  num_args = 1.., value_delimiter = ',', help = "Add hidden layers", default_value = "1024,512")]
    //pub hidden_layers: Vec<i64>,

    //#[arg(long = "separate", help = "Separate learning for different agents")]
    //pub separate: bool,



    #[arg(help = "Configuration file")]
    pub config_file: PathBuf
    //#[arg(short = 'c', long = "info_set_tensor", help = "Way to convert info set to tensor", default_value = "sparse")]
    //pub w2t: InfoSetWayToTensorSelect,



    //#[arg(short = 'r', long = "learning_rate", help = "learning_rate", default_value = "0.0001")]
    //pub learning_rate: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicSessionConfig{
    pub test_set: Option<PathBuf>,
    pub tests_set_size: Option<u32>,
    pub epochs: u32,

    pub games: u32,

    pub declarer: AgentConfiguration,
    pub whist: AgentConfiguration,
    pub offside: AgentConfiguration,
    pub test_declarer: AgentConfiguration,
    pub test_whist: AgentConfiguration,
    pub test_offside: AgentConfiguration,


}

impl Default for DynamicSessionConfig{
    fn default() -> Self {
        DynamicSessionConfig{
            test_set: Some("test_games.ron".into()),
            tests_set_size: None,
            epochs: 100,
            games: 100,
            declarer: Default::default(),
            whist: Default::default(),
            offside: Default::default(),
            test_declarer: Default::default(),
            test_whist: Default::default(),
            test_offside: Default::default(),
        }
    }
}