use std::path::PathBuf;
use clap::{Args, ValueEnum};
use amfiteatr_rl::tch::nn::Adam;
use crate::options::operation::train::{DeviceSelect, InfoSetWayToTensorSelect};

#[derive(ValueEnum, Copy, Clone)]
pub enum PolicyTypeSelect{
    Q,
    A2C
}

#[derive(Clone)]
pub struct PolicyParams{
    pub hidden_layers: Vec<i64>,
    pub optimizer_params: Adam,
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
    #[arg(short = 'd', long = "declarer_save", help = "Declarer VarStore save file")]
    pub declarer_save: Option<PathBuf>,
    #[arg(short = 'w', long = "whist_save", help = "Whist VarStore save file")]
    pub whist_save: Option<PathBuf>,
    #[arg(short = 'o', long = "offside_save", help = "Offside VarStore save file")]
    pub offside_save: Option<PathBuf>,

    #[arg(short = 'D', long = "declarer_load", help = "Declarer VarStore load file")]
    pub declarer_load: Option<PathBuf>,
    #[arg(short = 'W', long = "whist_load", help = "Whist VarStore load file")]
    pub whist_load: Option<PathBuf>,
    #[arg(short = 'O', long = "offside_load", help = "Offside VarStore load file")]
    pub offside_load: Option<PathBuf>,

    #[arg(short = 'e', long = "epochs", help = "Number of epochs", default_value = "10")]
    pub epochs: u32,

    #[arg(short = 'n', long = "games", help = "games iin epoch", default_value = "100")]
    pub games: u32,

    #[arg(short = 't', long = "tests", help = "test_set_number", default_value = "100")]
    pub tests_set_size: u32,

    #[arg(short = 'l', long = "layers",  num_args = 1.., value_delimiter = ',', help = "Add hidden layers", default_value = "1024,512")]
    pub hidden_layers: Vec<i64>,

    //#[arg(long = "separate", help = "Separate learning for different agents")]
    //pub separate: bool,

    #[arg(long = "device", help = "Device to be used", default_value = "cpu")]
    pub device: DeviceSelect,

    #[arg(short = 'c', long = "info_set_tensor", help = "Way to convert info set to tensor", default_value = "sparse")]
    pub w2t: InfoSetWayToTensorSelect,

    #[arg(short = 'T', long = "test_set", help = "Pre-generated set of contracts with cards distribution and deal for tests")]
    pub test_set: Option<PathBuf>,

    #[arg(short = 'r', long = "learning_rate", help = "learning_rate", default_value = "0.0001")]
    pub learning_rate: f64,
}