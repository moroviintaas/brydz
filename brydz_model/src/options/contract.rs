use std::path::PathBuf;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use amfiteatr_rl::policy::{ConfigA2C, ConfigPPO};
use amfiteatr_rl::tch;
use amfiteatr_rl::tch::Device;
use amfiteatr_rl::torch_net::Layer;
use brydz_core::player::side::{Side, SideMap};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "tch::Device")]
enum DeviceDef {
    Cpu,
    Cuda(usize),
    Mps,
    Vulkan
}

fn default_learning_rate() -> f64{
    0.0001
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PolicyOuterConfig{
    pub var_store_load: Option<PathBuf>,
    pub var_store_save: Option<PathBuf>,
    #[serde(with = "DeviceDef")]
    pub device: tch::Device,
    pub network_layers: Vec<Layer>,
    #[serde(default = "default_learning_rate")]
    pub adam_learning_rate: f64,



}

impl std::default::Default for PolicyOuterConfig {
    fn default() -> Self {
        //log::debug!("Creating default policy outer config");

        Self{
            device: Device::Cpu,
            network_layers: vec![Layer::Linear(64), Layer::Relu],
            adam_learning_rate: 0.0001,
            var_store_load: None,
            var_store_save: None,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum AgentPolicyInnerConfig{
    MaskingPPO(ConfigPPO),
    MaskingA2C(ConfigA2C),
    PPO(ConfigPPO),
    A2C(ConfigA2C),

}

impl Default for AgentPolicyInnerConfig{
    fn default() -> Self {
        Self::MaskingPPO(ConfigPPO::default())
    }
}



#[derive(Copy, Clone, Serialize, Deserialize, ValueEnum, Default, Debug)]
pub enum InformationSetSelection{
    CompleteKnowledge,
    DistributionAssume,
    #[default]
    Simple,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct AgentConfig{
    #[serde(default)]
    pub limit_learn_epochs: Option<usize>,
    pub policy: AgentPolicyInnerConfig,
    pub policy_data: PolicyOuterConfig,
    pub information_set_type: InformationSetSelection,
    pub information_set_conversion: InformationSetRepresentation,


}



#[derive(Clone, Serialize, Deserialize, ValueEnum, Default, Debug)]
pub enum InformationSetRepresentation{
    Dense,
    #[default]
    Sparse,
    SparseHistoric
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TestSet{
    Saved(PathBuf),
    New(usize),
}

impl Default for TestSet{
    fn default() -> TestSet{
        Self::New(100)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModelConfig{
    pub number_of_epochs: usize,
    pub number_of_games_in_epoch: usize,
    pub agents: SideMap<AgentConfig>,
    pub test_set: TestSet,
    pub game_deal_biases: Option<PathBuf>,
    pub force_declarer_when_rand: Option<Side>,
}



/*
#[derive(Debug, Args)]
pub struct ContractModelOptions{


    #[arg(short = 'p', long = "probability-file", help = "Path to file with probabilities of cards")]
    pub probability_file: Option<PathBuf>,
}

 */