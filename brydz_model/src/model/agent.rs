use amfiteatr_core::agent::TracingAgentGen;
use amfiteatr_core::comm::StdAgentEndpoint;
use amfiteatr_rl::policy::{PolicyDiscreteA2C,
                           PolicyDiscretePPO,
                           PolicyMaskingDiscreteA2C,
                           PolicyMaskingDiscretePPO
};
use amfiteatr_rl::tch::nn::{AdamW, VarStore};
use amfiteatr_rl::tensor_data::TensorEncoding;
use amfiteatr_rl::torch_net::{build_network_operator_ac, NeuralNetActorCritic};
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{
    ActionPlaceCardConvertion1D,
    ContractAgentInfoSetAllKnowing,
    ContractAgentInfoSetAssuming,
    ContractAgentInfoSetSimple,
    ContractEnvStateComplete,
    ContractInfoSetConvertDense1,
    ContractInfoSetConvertSparse,
    ContractInfoSetConvertSparseHistoric,
    ContractInfoSetEncoding,
    ContractInformationSet};
use crate::options::contract::{
    AgentConfig,
    AgentPolicyInnerConfig,
    InformationSetRepresentation,
    InformationSetSelection,
    PolicyConfig
};
use amfiteatr_rl::tch::nn::OptimizerConfig;
use brydz_core::player::side::{Side, SideMap};
use crate::model::policy::ContractPolicy;
use brydz_core::amfiteatr::state::ContractState;


#[allow(dead_code)]
pub struct BAgent{
    agent: TracingAgentGen<ContractDP, ContractPolicy, StdAgentEndpoint<ContractDP>>,
    config: AgentConfig,
    reference_policy: ContractPolicy,
    reference_mode: bool,
}



impl BAgent{



    fn create_policy(policy_config: &PolicyConfig) -> anyhow::Result<ContractPolicy>{

        //let policy_config = config.policy.clone().ok_or_else(||anyhow!("Agent policy config missing"))?;

        let tensor_encoding = match policy_config.external.information_set_conversion{
            InformationSetRepresentation::Dense => ContractInfoSetEncoding::Dense1(ContractInfoSetConvertDense1{}),
            InformationSetRepresentation::Sparse => ContractInfoSetEncoding::Sparse(ContractInfoSetConvertSparse{}),
            InformationSetRepresentation::SparseHistoric => ContractInfoSetEncoding::SparseHistoric(ContractInfoSetConvertSparseHistoric{}),
        };
        let network_input_shape = tensor_encoding.desired_shape();

        let vs =  policy_config.external.var_store_load.as_ref()
            .map_or_else(
                || VarStore::new(policy_config.external.device),
                |_| VarStore::new(policy_config.external.device));

        let optimizer = AdamW::default().build(&vs, policy_config.external.adam_learning_rate)?;



        let operator = build_network_operator_ac(policy_config.external.network_layers.clone(),
                                                 network_input_shape.to_vec(), 52);
        let network = NeuralNetActorCritic::new(vs, operator);



        let policy = match policy_config.internal{
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





    pub fn build(config: AgentConfig, side: Side, comm: StdAgentEndpoint<ContractDP>,
                 shared_policy_config: &PolicyConfig) -> anyhow::Result<Self>{

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

        let policy = match &config.policy{
            Some(policy_config) => Self::create_policy(&policy_config)?,
            None => Self::create_policy(&shared_policy_config)?
        };

        let reference_policy = match &config.policy{
            Some(policy_config) => Self::create_policy(&policy_config)?,
            None => Self::create_policy(&shared_policy_config)?
        };

        Ok(
            BAgent{
                agent: TracingAgentGen::new(info_set, comm, policy),
                config,
                reference_policy,
                reference_mode: false
            }
        )
    }

    pub fn go_reference_mode(&mut self){
        if !self.reference_mode{
            self.agent.swap_policy(&mut self.reference_policy);
            self.reference_mode = true
        }
    }

    pub fn go_main_mode(&mut self){
        if self.reference_mode{
            self.agent.swap_policy(&mut self.reference_policy);
            self.reference_mode = false
        }
    }

    pub fn agent_mut(&mut self) -> &mut TracingAgentGen<ContractDP, ContractPolicy, StdAgentEndpoint<ContractDP>>{
        &mut self.agent
    }

    pub fn agent(&self) -> &TracingAgentGen<ContractDP, ContractPolicy, StdAgentEndpoint<ContractDP>>{
        &self.agent
    }
}