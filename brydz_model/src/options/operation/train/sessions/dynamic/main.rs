use std::fs;

use log::info;
use rand::thread_rng;











use brydz_core::player::axis::RoleAxis;
use brydz_core::player::role::PlayRole;



use crate::error::BrydzModelError;

use crate::options::operation::train::sessions::{AgentRole, DynamicBridgeModelBuilder, DynamicModelOptions, DynamicSessionConfig};



/*
fn parse_declarer_config(options: &DynamicModelOptions) -> AgentConfiguration{

    AgentConfiguration{
        info_set_type: options.declarer_is_type,
        info_set_conversion_type: options.declarer_tensor,
        policy_params: PolicyParams{
            hidden_layers: vec![2048, 1024],
            optimizer_params: Default::default(),
            select_policy: PolicyTypeSelect::Q,
            learning_rate: 0.0001,
        },
        var_load_path: options.declarer_load.clone(),
        var_store_path: options.declarer_save.clone(),
        device: DeviceSelect::Cpu,
    }
}

fn parse_whist_config(options: &DynamicModelOptions) -> AgentConfiguration{

    AgentConfiguration{
        info_set_type: options.whist_is_type,
        info_set_conversion_type: options.whist_tensor,
        policy_params: PolicyParams{
            hidden_layers: vec![2048, 1024],
            optimizer_params: Default::default(),
            select_policy: PolicyTypeSelect::Q,
            learning_rate: 0.0001,
        },
        var_load_path: options.whist_load.clone(),
        var_store_path: options.whist_save.clone(),
        device: DeviceSelect::Cpu,
    }
}

fn parse_offside_config(options: &DynamicModelOptions) -> AgentConfiguration{

    AgentConfiguration{
        info_set_type: options.offside_is_type,
        info_set_conversion_type: options.offside_tensor,
        policy_params: PolicyParams{
            hidden_layers: vec![2048, 1024],
            optimizer_params: Default::default(),
            select_policy: PolicyTypeSelect::Q,
            learning_rate: 0.0001,
        },
        var_load_path: options.offside_load.clone(),
        var_store_path: options.offside_save.clone(),
        device: DeviceSelect::Cpu,
    }
}

fn parse_test_declarer_config(options: &DynamicModelOptions) -> AgentConfiguration{

    AgentConfiguration{
        info_set_type: options.test_declarer_is_type,
        info_set_conversion_type: options.test_declarer_tensor,
        policy_params: PolicyParams{
            hidden_layers: vec![2048, 1024],
            optimizer_params: Default::default(),
            select_policy: PolicyTypeSelect::Q,
            learning_rate: 0.0001,
        },
        var_load_path: options.declarer_load.clone(),
        var_store_path: options.declarer_save.clone(),
        device: DeviceSelect::Cpu,
    }
}

fn parse_test_whist_config(options: &DynamicModelOptions) -> AgentConfiguration{

    AgentConfiguration{
        info_set_type: options.test_whist_is_type,
        info_set_conversion_type: options.test_whist_tensor,
        policy_params: PolicyParams{
            hidden_layers: vec![2048, 1024],
            optimizer_params: Default::default(),
            select_policy: PolicyTypeSelect::Q,
            learning_rate: 0.0001,
        },
        var_load_path: options.whist_load.clone(),
        var_store_path: options.whist_save.clone(),
        device: DeviceSelect::Cpu,
    }
}

fn parse_test_offside_config(options: &DynamicModelOptions) -> AgentConfiguration{

    AgentConfiguration{
        info_set_type: options.test_offside_is_type,
        info_set_conversion_type: options.test_offside_tensor,
        policy_params: PolicyParams{
            hidden_layers: vec![2048, 1024],
            optimizer_params: Default::default(),
            select_policy: PolicyTypeSelect::Q,
            learning_rate: 0.0001,
        },
        var_load_path: options.test_offside_load.clone(),
        var_store_path: None,
        device: DeviceSelect::Cpu,
    }
}

 */

pub fn run_dynamic_model(options: &DynamicModelOptions) -> Result<(), BrydzModelError>{

    let config_str = fs::read_to_string(&options.config_file).map_err(|e|{
        BrydzModelError::IO(format!("Error opening file {:?} with error {e:}", &options.config_file))
    })?;
    let config: DynamicSessionConfig = ron::from_str(&config_str).map_err(|e|{
        BrydzModelError::Ron(e.code)
    })?;

    let conf_declarer = &config.declarer;
    let conf_whist = &config.whist;
    let conf_offside =&config.offside;
    let conf_test_declarer = &config.test_declarer;
    let conf_test_whist = &config.test_whist;
    let conf_test_offside = &config.test_offside;

    //println!("{}", ron::ser::to_string_pretty(&DynamicSessionConfig::default(), ron::ser::PrettyConfig::default()).unwrap());



    let mut model = DynamicBridgeModelBuilder::new()
        .with_agent(conf_declarer, AgentRole::Declarer)?
        .with_agent(conf_whist, AgentRole::Whist)?
        .with_agent(conf_offside, AgentRole::Offside)?
        .with_agent(conf_test_declarer, AgentRole::TestDeclarer)?
        .with_agent(conf_test_whist, AgentRole::TestWhist)?
        .with_agent(conf_test_offside, AgentRole::TestOffside)?
        .build()?;

    if let Some(test_vec_file) = &config.test_set{
        model.load_test_games_from_file(test_vec_file)?;
    } else if let Some(test_set_size) = &config.tests_set_size{
        let mut rng = thread_rng();
        model.generate_test_games(&mut rng, *test_set_size as usize)?;
    } else {
        panic!("Expected test set (.ron file) or test number to be specified")
    }

    let r1 = model.run_test_series(RoleAxis::Declarers)?;
    //info!("Testing declarers before learning: {r:?}");
    let r2 = model.run_test_series(RoleAxis::Defenders)?;
    info!("Test before learn. Trained declarer against reference: {}. Trained whist,offide against reference: {},{}",
        r1.scores[PlayRole::Declarer], r2.scores[PlayRole::Whist], r2.scores[PlayRole::Offside]);
    //info!("Testing defenders before learning: {r:?}");

    let epochs = config.epochs;
    let games_in_epoch = config.games as usize;
    for i in 0..epochs{

        model.learning_epoch(games_in_epoch)?;
        let r1 = model.run_test_series(RoleAxis::Declarers)?;

        let r2 = model.run_test_series(RoleAxis::Defenders)?;
        info!("Learning epoch: {}", i+1);
        info!("Test after epoch: {}. Trained declarer against reference: {}. Trained whist,offside against reference: {},{}",
        i+1, r1.scores[PlayRole::Declarer], r2.scores[PlayRole::Whist], r2.scores[PlayRole::Offside]);
    }

    if let Some(declarer_store) = &config.declarer.var_store_path{
        model.store_agents_var(&model.declarer, declarer_store).unwrap();
    }
    if let Some(offside_store) = &config.offside.var_store_path{
        model.store_agents_var(&model.offside, offside_store).unwrap();
    }
    if let Some(whist_store) = &config.whist.var_store_path{
        model.store_agents_var(&model.whist, whist_store).unwrap();
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