use log::info;
use amfiteatr_core::agent::{InformationSet, TracingAgentGen};
use amfiteatr_core::comm::StdAgentEndpoint;
use amfiteatr_core::util::TensorboardSupport;
use amfiteatr_rl::agent::RlSimpleLearningAgent;
use amfiteatr_rl::policy::{LearnSummary, LearningNetworkPolicy, LearningNetworkPolicyDynamic, LearningNetworkPolicyGeneric, PolicyDiscreteA2C, PolicyDiscretePPO, PolicyMaskingDiscreteA2C, PolicyMaskingDiscretePPO};
use amfiteatr_rl::tch::nn::{Adam, AdamW, VarStore};
use amfiteatr_rl::tensor_data::{ContextEncodeTensor, TensorEncoding};
use amfiteatr_rl::torch_net::{build_network_operator_ac, A2CNet, NeuralNetActorCritic};
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ActionPlaceCardConvertion1D, ContractActionWayToTensor, ContractAgentInfoSetAllKnowing, ContractAgentInfoSetAssuming, ContractAgentInfoSetSimple, ContractEnvStateComplete, ContractInfoSetConvertDense1, ContractInfoSetConvertSparse, ContractInfoSetConvertSparseHistoric, ContractInfoSetEncoding, ContractInformationSet};
use brydz_core::deal::{ContractGameDescription, DealDistribution};
use crate::options::contract::{AgentConfig, AgentPolicyInnerConfig, InformationSetRepresentation, InformationSetSelection};
use amfiteatr_rl::tch::nn::OptimizerConfig;
use brydz_core::player::side::{Side, SideMap};
use crate::model::policy::ContractPolicy;
use brydz_core::amfiteatr::state::ContractState;

pub trait SimpleContractAgentT:  RlSimpleLearningAgent<ContractDP, DealDistribution, LearnSummary>
    + TensorboardSupport<ContractDP>
{}

#[allow(dead_code)]
pub struct BAgent{
    //agent: Box<dyn SimpleContractAgentT>,
    agent: TracingAgentGen<ContractDP, ContractPolicy, StdAgentEndpoint<ContractDP>>,
    config: AgentConfig,
}



impl BAgent{



    fn create_policy(config: &AgentConfig) -> anyhow::Result<ContractPolicy>{

        let tensor_encoding = match config.information_set_conversion{
            InformationSetRepresentation::Dense => ContractInfoSetEncoding::Dense1(ContractInfoSetConvertDense1{}),
            InformationSetRepresentation::Sparse => ContractInfoSetEncoding::Sparse(ContractInfoSetConvertSparse{}),
            InformationSetRepresentation::SparseHistoric => ContractInfoSetEncoding::SparseHistoric(ContractInfoSetConvertSparseHistoric{}),
        };
        let network_input_shape = tensor_encoding.desired_shape();

        let vs =  config.policy_data.var_store_load.as_ref()
            .map_or_else(
                || VarStore::new(config.policy_data.device),
                |v| VarStore::new(config.policy_data.device));

        let optimizer = AdamW::default().build(&vs, config.policy_data.adam_learning_rate)?;



        let operator = build_network_operator_ac(config.policy_data.network_layers.clone(),
                                                 network_input_shape.to_vec(), 52);
        let network = NeuralNetActorCritic::new(vs, operator);



        let policy = match config.policy{
            AgentPolicyInnerConfig::MaskingPPO(policy_config) => {


                ContractPolicy::MaskedPpo(PolicyMaskingDiscretePPO::new(
                    policy_config, network, optimizer, tensor_encoding, ActionPlaceCardConvertion1D {}))
            }
            AgentPolicyInnerConfig::MaskingA2C(policy_config) => {
                ContractPolicy::MaskedA2C(PolicyMaskingDiscreteA2C::new(policy_config, network, optimizer, tensor_encoding, ActionPlaceCardConvertion1D {}))
            }
            AgentPolicyInnerConfig::PPO(policy_config) => {
                ContractPolicy::Ppo(PolicyDiscretePPO::new(
                    policy_config, network, optimizer, tensor_encoding, ActionPlaceCardConvertion1D {}))
            }
            AgentPolicyInnerConfig::A2C(policy_config) => {
                ContractPolicy::A2C(PolicyDiscreteA2C::new(policy_config, network, optimizer, tensor_encoding, ActionPlaceCardConvertion1D {}))
            }
        };
        Ok(policy)
    }





    pub fn build(config: AgentConfig, side: Side, comm: StdAgentEndpoint<ContractDP>) -> anyhow::Result<Self>{

        let default_contract = ContractEnvStateComplete::default();

        let default_hand = default_contract[side];

        let info_set = match config.information_set_type{
            InformationSetSelection::CompleteKnowledge => ContractInformationSet::AllKnowing(
                ContractAgentInfoSetAllKnowing::new(side, SideMap {
                    north: default_contract[Side::North],
                    east: default_contract[Side::East],
                    south: default_contract[Side::South],
                    west: default_contract[Side::West],
                }, default_contract.contract_data().clone())
            ),
            InformationSetSelection::DistributionAssume => ContractInformationSet::Assuming(
                ContractAgentInfoSetAssuming::new_fair(side, default_hand, default_contract.contract_data().clone(), None)
            ),
            InformationSetSelection::Simple => ContractInformationSet::Simple(
                ContractAgentInfoSetSimple::new(side, default_hand, default_contract.contract_data().clone(), None)
            )
        };

        let policy = Self::create_policy(&config)?;

        Ok(
            BAgent{
                agent: TracingAgentGen::new(info_set, comm, policy),
                config,
            }
        )





    }

    pub fn agent_mut(&mut self) -> &mut TracingAgentGen<ContractDP, ContractPolicy, StdAgentEndpoint<ContractDP>>{
        &mut self.agent
    }

    pub fn agent(&self) -> &TracingAgentGen<ContractDP, ContractPolicy, StdAgentEndpoint<ContractDP>>{
        &self.agent
    }
}