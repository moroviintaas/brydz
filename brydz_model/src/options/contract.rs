use std::path::PathBuf;
use clap::ValueEnum;
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
    pub number_of_epochs: usize,
    pub number_of_game_in_epoch: usize,
    pub agents: SideMap<AgentConfig>,
    pub test_deal_file: Option<PathBuf>,
    pub game_deal_biases: Option<PathBuf>,
}



/*
#[derive(Debug, Args)]
pub struct ContractModelOptions{


    #[arg(short = 'p', long = "probability-file", help = "Path to file with probabilities of cards")]
    pub probability_file: Option<PathBuf>,
}

 */