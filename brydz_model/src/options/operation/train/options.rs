use clap::{Args, ValueEnum};
use std::path::PathBuf;
use amfiteatr_rl::tch::Device;
use amfiteatr_rl::policy::TrainConfig;

#[derive(Debug, Clone)]
pub enum TestPolicyChoice{
    RandomPolicy,
    InitLikeLearning,
}

#[derive(ValueEnum, Copy, Clone)]
pub enum DeviceSelect{
    Cpu,
    Cuda,
    Vulkan
}

#[derive(ValueEnum, Copy, Clone)]
pub enum InfoSetTypeSelect{
    Simple,
    Assume,
    Complete
}

#[derive(ValueEnum, Copy, Clone)]
pub enum InfoSetWayToTensorSelect{
    _420,
    Sparse,
}

impl DeviceSelect{
    pub fn map(self) -> amfiteatr_rl::tch::Device{
        match self{
            DeviceSelect::Cpu => {Device::Cpu}
            DeviceSelect::Cuda => {Device::Cuda(32000)}
            DeviceSelect::Vulkan => {Device::Vulkan}
        }
    }
}



#[derive(Args, Clone)]
pub struct TrainOptions{

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

    /*
    #[arg(short = 'S', long = "test_declarer_load", help = "Test Declarer VarStore load file")]
    pub test_declarer_load: Option<PathBuf>,
    #[arg(short = 'Q', long = "test_whist_load", help = "Test Whist VarStore load file")]
    pub test_whist_load: Option<PathBuf>,
    #[arg(short = 'I', long = "test_offside_load", help = "Test Offside VarStore load file")]
    pub test_offside_load: Option<PathBuf>,
    */
    #[arg(short = 'e', long = "epochs", help = "Number of epochs", default_value = "10")]
    pub epochs: u32,

    #[arg(short = 'n', long = "games", help = "games iin epoch", default_value = "100")]
    pub games: u32,

    #[arg(short = 't', long = "tests", help = "test_set_number", default_value = "100")]
    pub tests_set_size: u32,

    #[arg(short = 'l', long = "layers",  num_args = 1.., value_delimiter = ',', help = "Add hidden layers", default_value = "1024,512")]
    pub hidden_layers: Vec<i64>,

    #[arg(long = "separate", help = "Separate learning for different agents")]
    pub separate: bool,

    #[arg(long = "device", help = "Device to be used", default_value = "cpu")]
    pub device: DeviceSelect,

    #[arg(short = 'g', long = "gamma", help = "Discount factor (gamma)", default_value = "0.99")]
    pub gamma: f64,

    #[arg(short = 'i', long = "info_set", help = "InfoSet type", default_value = "simple")]
    pub info_set_select: InfoSetTypeSelect,

    #[arg(short = 'c', long = "info_set_tensor", help = "Way to convert info set to tensor", default_value = "sparse")]
    pub w2t: InfoSetWayToTensorSelect,

    #[arg(short = 'T', long = "test_set", help = "Pre-generated set of contracts with cards distribution and deal for tests")]
    pub test_set: Option<PathBuf>,

    #[arg(short = 'r', long = "learning_rate", help = "learning_rate", default_value = "0.0001")]
    pub learning_rate: f64,



}

impl From<&TrainOptions> for TrainConfig{
    fn from(value: &TrainOptions) -> Self {
        TrainConfig{
            gamma: value.gamma
        }
    }
}