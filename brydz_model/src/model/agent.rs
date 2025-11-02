use amfiteatr_core::util::TensorboardSupport;
use amfiteatr_rl::agent::RlSimpleLearningAgent;
use amfiteatr_rl::policy::LearnSummary;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::deal::DealDistribution;
use crate::options::contract::AgentConfig;

pub trait SimpleContractAgentT:  RlSimpleLearningAgent<ContractDP, DealDistribution, LearnSummary>
    + TensorboardSupport<ContractDP>
{}

#[allow(dead_code)]
pub struct BAgent{
    agent: Box<dyn SimpleContractAgentT>,
    config: AgentConfig
}

impl BAgent{

    #[allow(dead_code)]
    pub fn build(_config: AgentConfig) -> anyhow::Result<Self>{


        todo!()

    }
}