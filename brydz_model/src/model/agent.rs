use amfiteatr_core::agent::InformationSet;
use amfiteatr_core::util::TensorboardSupport;
use amfiteatr_rl::agent::RlSimpleLearningAgent;
use amfiteatr_rl::policy::{LearnSummary, LearningNetworkPolicy, LearningNetworkPolicyDynamic, LearningNetworkPolicyGeneric, PolicyMaskingDiscretePPO};
use amfiteatr_rl::tch::nn::{Adam, VarStore};
use amfiteatr_rl::tensor_data::{ContextEncodeTensor, TensorEncoding};
use amfiteatr_rl::torch_net::{create_net_model_discrete_ac, A2CNet};
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ActionPlaceCardConvertion1D, ContractActionWayToTensor, ContractInfoSetConvertDense1, ContractInfoSetConvertSparse, ContractInfoSetConvertSparseHistoric};
use brydz_core::deal::DealDistribution;
use crate::options::contract::{AgentConfig, AgentPolicyInnerConfig, InformationSetRepresentation, InformationSetSelection};
use amfiteatr_rl::tch::nn::OptimizerConfig;

pub trait SimpleContractAgentT:  RlSimpleLearningAgent<ContractDP, DealDistribution, LearnSummary>
    + TensorboardSupport<ContractDP>
{}

#[allow(dead_code)]
pub struct BAgent{
    agent: Box<dyn SimpleContractAgentT>,
    config: AgentConfig,
}

impl BAgent{

    /*
    fn create_policy<
        IS: InformationSet<ContractDP>
            + ContextEncodeTensor<ContractInfoSetConvertDense1>
            + ContextEncodeTensor<ContractInfoSetConvertSparse>
            + ContextEncodeTensor<ContractInfoSetConvertSparseHistoric>
    >(config: &AgentConfig)

        -> anyhow::Result<Box<dyn LearningNetworkPolicyDynamic<ContractDP,  InfoSetType=IS>>> {

        let tensor_encoding: Box<dyn TensorEncoding> = match config.information_set_conversion{
            InformationSetRepresentation::Dense => Box::new(ContractInfoSetConvertDense1{}),
            InformationSetRepresentation::Sparse => Box::new(ContractInfoSetConvertSparse{}),
            InformationSetRepresentation::SparseHistoric => Box::new(ContractInfoSetConvertSparseHistoric{}),
        };
        let network_input_shape = tensor_encoding.desired_shape();


        match config.policy{
            AgentPolicyInnerConfig::MaskingPPO(ppo) => {



                let vs =  config.policy_data.var_store_load.as_ref()
                    .map_or_else(
                        || VarStore::new(config.policy_data.device),
                        |v| VarStore::new(config.policy_data.device));


                let net_box = create_net_model_discrete_ac(
                    vs.root().clone(),
                    &config.policy_data.network_layers,
                    network_input_shape, 52);

                let optimizer = Adam::default().build(&vs, config.policy_data.adam_learning_rate)?;
                let network = A2CNet::new_from_box(vs, net_box);

                Ok(match config.information_set_conversion{
                    InformationSetRepresentation::Dense => Box::new(PolicyMaskingDiscretePPO::new(
                                    ppo, network, optimizer, ContractInfoSetConvertDense1{}, ActionPlaceCardConvertion1D{})),
                    InformationSetRepresentation::Sparse => {todo!()},
                    InformationSetRepresentation::SparseHistoric => {todo!()}
                })



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
    
     */



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