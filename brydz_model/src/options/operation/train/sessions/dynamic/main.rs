use std::sync::{Arc, Mutex};
use rand::thread_rng;
use amfiteatr_core::agent::TracingAgentGen;
use amfiteatr_core::comm::StdEnvironmentEndpoint;
use amfiteatr_rl::agent::{RlSimpleLearningAgent, RlSimpleTestAgent};
use amfiteatr_rl::error::AmfiRLError;
use amfiteatr_rl::tch::Device;
use amfiteatr_rl::tch::nn::VarStore;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ContractAgentInfoSetAllKnowing, ContractAgentInfoSetSimple, ContractInfoSetConvertSparse};
use brydz_core::contract::ContractParameters;
use brydz_core::deal::DescriptionDeckDeal;
use brydz_core::player::side::Side;
use brydz_core::player::side::Side::North;
use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{AgentConfiguration, ContractInfoSetSeed, DynamicBridgeModelBuilder, DynamicModelOptions, PolicyParams, PolicyTypeSelect};
use crate::SimContractParams;

pub fn run_dynamic_model(options: &DynamicModelOptions) -> Result<(), AmfiRLError<ContractDP>>{


    let mut model = DynamicBridgeModelBuilder::new();

    let mut rng = thread_rng();
    let si = SimContractParams::new_fair_random(&mut rng);
    let description = DescriptionDeckDeal{
        probabilities: si.distribution().clone(),
        cards: si.cards().clone(),
    };

    let agent_conf = AgentConfiguration{
        info_set_type: InfoSetTypeSelect::Simple,
        info_set_conversion_type: InfoSetWayToTensorSelect::Sparse,
        policy_params: PolicyParams {
            hidden_layers: vec![1024,1024],
            optimizer_params: Default::default(),
            select_policy: PolicyTypeSelect::Q,
            learning_rate: 0.001,
        },
    };
    let (_,e) = StdEnvironmentEndpoint::new_pair();
    let info_set = ContractAgentInfoSetSimple::from((&North, si.parameters() , &description));
    let policy = model.create_agent_q_policy(&agent_conf, VarStore::new(Device::Cpu), ContractInfoSetConvertSparse::default())?;
    //let mut agent: Arc<Mutex<dyn RlSimpleTestAgent<ContractDP, (Side, ContractParameters, DescriptionDeckDeal)>>> = Arc::new(Mutex::new(TracingAgentGen::new(info_set, e, policy)));
    let mut agent = TracingAgentGen::new(info_set, e, policy);

    agent.simple_apply_experience()?;


    Ok(())

}