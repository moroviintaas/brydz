use std::sync::{Arc, Mutex};
use log::info;
use rand::thread_rng;
use amfiteatr_core::agent::TracingAgentGen;
use amfiteatr_core::comm::StdEnvironmentEndpoint;
use amfiteatr_rl::agent::{RlSimpleLearningAgent, RlSimpleTestAgent};
use amfiteatr_rl::error::AmfiRLError;
use amfiteatr_rl::tch::Device;
use amfiteatr_rl::tch::Device::Cpu;
use amfiteatr_rl::tch::nn::VarStore;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ContractAgentInfoSetAllKnowing, ContractAgentInfoSetSimple, ContractInfoSetConvertSparse};
use brydz_core::contract::ContractParameters;
use brydz_core::deal::{ContractGameDescription, DescriptionDeckDeal};
use brydz_core::player::axis::RoleAxis;
use brydz_core::player::role::PlayRole;
use brydz_core::player::role::PlayRole::Declarer;
use brydz_core::player::side::Side;
use brydz_core::player::side::Side::North;
use crate::error::BrydzModelError;
use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{AgentConfiguration, AgentRole, ContractInfoSetSeedLegacy, DynamicBridgeModelBuilder, DynamicModelOptions, PolicyParams, PolicyTypeSelect};
use crate::options::operation::train::sessions::AgentRole::{Offside, Whist};

pub fn run_dynamic_model(options: &DynamicModelOptions) -> Result<(), BrydzModelError>{


    let conf_declarer = AgentConfiguration::default();
    let conf_whist = AgentConfiguration::default();
    let conf_offside = AgentConfiguration::default();
    let conf_test_declarer = AgentConfiguration::default();
    let conf_test_whist = AgentConfiguration::default();
    let conf_test_offside = AgentConfiguration::default();



    let mut model = DynamicBridgeModelBuilder::new()
        .with_agent(&conf_declarer, AgentRole::Declarer)?
        .with_agent(&conf_whist, AgentRole::Whist)?
        .with_agent(&conf_offside, AgentRole::Offside)?
        .with_agent(&conf_test_declarer, AgentRole::TestDeclarer)?
        .with_agent(&conf_test_whist, AgentRole::TestWhist)?
        .with_agent(&conf_test_offside, AgentRole::TestOffside)?
        .build()?;

    if let Some(test_vec_file) = &options.test_set{
        model.load_test_games_from_file(test_vec_file)?;
    } else {
        let mut rng = thread_rng();
        model.generate_test_games(&mut rng, options.tests_set_size as usize)?;
    }

    let r1 = model.run_test_series(RoleAxis::Declarers)?;
    //info!("Testing declarers before learning: {r:?}");
    let r2 = model.run_test_series(RoleAxis::Defenders)?;
    info!("Test before learn. Trained declarer against reference: {}. Trained whist,offide against reference: {},{}",
        r1.scores[PlayRole::Declarer], r2.scores[PlayRole::Whist], r2.scores[PlayRole::Offside]);
    //info!("Testing defenders before learning: {r:?}");

    let epochs = 50;
    let games_in_epoch = 100;
    for i in 0..epochs{
        info!("Learning epoch: {}", i+1);
        model.learning_epoch(games_in_epoch)?;
        let r1 = model.run_test_series(RoleAxis::Declarers)?;

        let r2 = model.run_test_series(RoleAxis::Defenders)?;

        info!("Test after epoch: {}. Trained declarer against reference: {}. Trained whist,offide against reference: {},{}",
        i+1, r1.scores[PlayRole::Declarer], r2.scores[PlayRole::Whist], r2.scores[PlayRole::Offside]);
    }


    /*
    let mut rng = thread_rng();
    let si = ContractGameDescription::new_fair_random(&mut rng);
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
        var_load_path: None,
        var_store_path: None,
        device: Cpu
    };
    let (_,e) = StdEnvironmentEndpoint::new_pair();
    let info_set = ContractAgentInfoSetSimple::from((&North, si.parameters() , &description));
    let policy = model.create_agent_q_policy(&agent_conf, VarStore::new(Device::Cpu), ContractInfoSetConvertSparse::default())?;
    let mut agent: Arc<Mutex<dyn RlSimpleLearningAgent<ContractDP, (&Side, &ContractParameters, &DescriptionDeckDeal)>>> = Arc::new(Mutex::new(TracingAgentGen::new(info_set, e, policy)));
    //let mut agent = TracingAgentGen::new(info_set, e, policy);

    agent.lock().unwrap().simple_apply_experience()?;
    */




    Ok(())

}