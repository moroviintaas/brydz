use amfiteatr_core::agent::InformationSet;
use amfiteatr_core::util::TensorboardSupport;
use amfiteatr_rl::agent::RlSimpleLearningAgent;
use amfiteatr_rl::policy::{LearnSummary, LearningNetworkPolicy, LearningNetworkPolicyDynamic, LearningNetworkPolicyGeneric, PolicyMaskingDiscretePPO};
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::deal::DealDistribution;
use crate::options::contract::{AgentConfig, AgentPolicyInnerConfig, InformationSetSelection};

pub trait SimpleContractAgentT:  RlSimpleLearningAgent<ContractDP, DealDistribution, LearnSummary>
    + TensorboardSupport<ContractDP>
{}

#[allow(dead_code)]
pub struct BAgent{
    agent: Box<dyn SimpleContractAgentT>,
    config: AgentConfig
}

impl BAgent{


    fn create_policy<IS: InformationSet<ContractDP>>(config: &AgentConfig)
        -> anyhow::Result<Box<dyn LearningNetworkPolicyDynamic<ContractDP,  InfoSetType=IS>>> {

        match config.policy{
            AgentPolicyInnerConfig::MaskingPPO(ppo) => {
                todo!()
                /*
                Ok(Box::new(PolicyMaskingDiscretePPO::new(
                    ppo, (), (), (), ())))

                 */
            },
            AgentPolicyInnerConfig::MaskingA2C(_) => {
                todo!()
            }
            AgentPolicyInnerConfig::PPO(_) => {
                todo!()
            }
            AgentPolicyInnerConfig::A2C(_) => {
                todo!()
            }
        }
    }



    pub fn build(config: AgentConfig) -> anyhow::Result<Self>{

        match config.information_set_type{
            InformationSetSelection::CompleteKnowledge => {}
            InformationSetSelection::DistributionAssume => {}
            InformationSetSelection::Simple => {

            }
        }


        todo!()

    }
}