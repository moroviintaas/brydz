use amfiteatr_rl::agent::{NetworkLearningAgent, RlSimpleLearningAgent};
use amfiteatr_rl::policy::LearnSummary;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::deal::DealDistribution;

pub trait SimpleContractAgentT: RlSimpleLearningAgent<ContractDP, DealDistribution, LearnSummary>{}

