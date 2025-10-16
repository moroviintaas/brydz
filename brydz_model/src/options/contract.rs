use std::path::PathBuf;
use clap::{Args, ValueEnum};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use amfiteatr_rl::policy::{ConfigA2C, ConfigPPO};
use brydz_core::player::side::SideMap;


#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct PolicyOuterConfig{
    pub var_store_load: Option<PathBuf>,
    pub var_store_save: Option<PathBuf>,


}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum AgentPolicyInnerConfig{
    PPO(ConfigPPO),
    A2C(ConfigA2C)
}

impl Default for AgentPolicyInnerConfig{
    fn default() -> Self {
        Self::PPO(ConfigPPO::default())
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
    pub information_set_conversion: InformationSetRepresentation

}

#[derive(Clone, Serialize, Deserialize, ValueEnum, Default, Debug)]
pub enum InformationSetRepresentation{
    Dense,
    #[default]
    Sparse,
    SparseHistoric
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ModelConfig{
    number_of_epochs: usize,
    agents: SideMap<AgentConfig>,
    test_deal_file: Option<PathBuf>,
    game_deal_biases: Option<PathBuf>,
}



/*
#[derive(Debug, Args)]
pub struct ContractModelOptions{


    #[arg(short = 'p', long = "probability-file", help = "Path to file with probabilities of cards")]
    pub probability_file: Option<PathBuf>,
}

 */