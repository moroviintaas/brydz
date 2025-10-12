use std::path::PathBuf;
use clap::{Args, ValueEnum};
use serde::{Deserialize, Serialize};
use amfiteatr_rl::policy::{ConfigA2C, ConfigPPO};



#[derive(Clone, Serialize, Deserialize)]
pub struct PolicyOuterConfig{
    pub var_store_load: Option<PathBuf>,
    pub var_store_save: Option<PathBuf>,


}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum AgentPolicyInnerConfig{
    PPO(ConfigPPO),
    A2C(ConfigA2C)
}

#[derive(Copy, Clone, Serialize, Deserialize, ValueEnum)]
pub enum InformationSetSelection{
    CompleteKnowledge,
    DistributionAssume,
    Simple,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AgentConfig{
    pub limit_learn_epochs: Option<usize>,
    pub policy: AgentPolicyInnerConfig,
    pub policy_data: PolicyOuterConfig,
    pub information_set_type: InformationSetSelection,
    pub information_set_conversion: InformationSetRepresentation

}

#[derive(Clone, Serialize, Deserialize, ValueEnum)]
pub enum InformationSetRepresentation{
    Dense,
    Sparse,
    SparseHistoric
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelConfig{

}

#[derive(Debug, Args)]
pub struct ContractModelOptions{


    #[arg(short = 'p', long = "probability-file", help = "Path to file with probabilities of cards")]
    pub probability_file: Option<PathBuf>,
}