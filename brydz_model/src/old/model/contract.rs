use std::sync::{Arc, Mutex};
use std::thread;
use brydz_core::contract::{Contract};
use brydz_core::player::side::{SideMap};

use brydz_core::amfi::comm::ContractEnvSyncComm;
use brydz_core::amfi::spec::ContractDP;
use brydz_core::amfi::state::{ContractAgentInfoSetSimple, ContractDummyState, ContractEnvStateMin};
use amfiteatr_core::agent::{AgentGen, RandomPolicy};
use amfiteatr_core::comm::DynEndpoint;
//use amfiteatr_core::env::{RoundRobinModel, RoundRobinModelBuilder};
use amfiteatr_core::error::{CommunicationError};
use amfiteatr_core::domain::{AgentMessage, EnvironmentMessage};
use amfiteatr_core::env::RoundRobinUniversalEnvironment;
//use amfiteatr_core::env::{RoundRobinModel, RoundRobinModelBuilder};
use amfiteatr_net_ext::{ComplexComm1024};
use brydz_core::amfi::env::ContractEnv;
use crate::error::{BrydzSimError};
use crate::SimContractParams;

/*
pub(crate) type LocalModelContract =
RoundRobinModel<
    ContractDP,
    ContractEnvStateMin,
    ComplexComm1024<
        EnvironmentMessage<ContractDP>,
        AgentMessage<ContractDP>,
        CommunicationError<ContractDP>>>;


 */
pub fn generate_local_model(params: &SimContractParams) -> Result<LocalModelContract, BrydzSimError>{
    let (comm_env_north, comm_north) = ContractEnvSyncComm::new_pair();
    let (comm_env_east, comm_east) = ContractEnvSyncComm::new_pair();
    let (comm_env_west, comm_west) = ContractEnvSyncComm::new_pair();
    let (comm_env_south, comm_south) = ContractEnvSyncComm::new_pair();

    let agent_comm_map = SideMap::new(comm_north, comm_east, comm_south, comm_west);
    let env_comm_map = SideMap::new(comm_env_north, comm_env_east, comm_env_south, comm_env_west);

    let card_deal = params.cards();
    let initial_contract = Contract::new(params.parameters().clone());
    let declarer = params.parameters().declarer();
    let dummy = params.parameters().declarer().next_i(2);
    let def1 = params.parameters().declarer().next_i(1);
    let def2 = params.parameters().declarer().next_i(3);

    //let (hand_north, hand_east, hand_south, hand_west) = card_deal.destruct();

    //this must be differed when Agent has different state's type
    let initial_state_declarer = ContractAgentInfoSetSimple::new(declarer, card_deal[&declarer], initial_contract.clone(), None);
    let initial_state_def1 = ContractAgentInfoSetSimple::new(def1, card_deal[&def1], initial_contract.clone(), None);
    let initial_state_dummy = ContractDummyState::new(dummy, card_deal[&dummy], initial_contract.clone());
    let initial_state_def2 = ContractAgentInfoSetSimple::new(def2, card_deal[&def2], initial_contract.clone(), None);
    //let initial_state_declarer = ContractAgentInfoSetSimple::new(declarer, card_deal[&declarer], initial_contract.clone(), None);
    //let initial_state_def1 = ContractAgentInfoSetSimple::new(def1, card_deal[&def1], initial_contract.clone(), None);
    //let initial_state_dummy = ContractDummyState::new(dummy, card_deal[&dummy], initial_contract.clone());
    //let initial_state_def2 = ContractAgentInfoSetSimple::new(def2, card_deal[&def2], initial_contract.clone(), None);

    //policy select
    let random_policy = RandomPolicy::<ContractDP, ContractAgentInfoSetSimple>::new();
    let policy_dummy = RandomPolicy::<ContractDP, ContractDummyState>::new();

    let (comm_declarer, comm_def1, comm_dummy, comm_def2) = agent_comm_map.destruct_start_with(declarer);
    let (comm_env_declarer, comm_env_def1, comm_env_dummy, comm_env_def2) = env_comm_map.destruct_start_with(declarer);

    let mut agent_declarer = AgentGen::new(initial_state_declarer, comm_declarer, random_policy.clone() );
    let mut agent_def1 = AgentGen::new(initial_state_def1, comm_def1, random_policy.clone() );
    let mut agent_dummy = AgentGen::new( initial_state_dummy, comm_dummy, policy_dummy);
    let mut agent_def2 = AgentGen::new( initial_state_def2, comm_def2, random_policy );

    /*
    let model = RoundRobinModelBuilder::new()
        .with_env_state(ContractEnvStateMin::new(initial_contract, None))?
        .add_local_agent(Arc::new(Mutex::new(agent_declarer)), ComplexComm1024::StdSync(comm_env_declarer))?
        .add_local_agent(Arc::new(Mutex::new(agent_def1)), ComplexComm1024::StdSync(comm_env_def1))?
        .add_local_agent(Arc::new(Mutex::new(agent_dummy)), ComplexComm1024::StdSync(comm_env_dummy))?
        .add_local_agent(Arc::new(Mutex::new(agent_def2)), ComplexComm1024::StdSync(comm_env_def2))?
        //.with_remote_agent(Side::South, env_comm_south)?
        .build()?;

     */

    let mut environment = ContractEnv::new(ContractEnvStateMin::new(initial_contract, None), env_comm_map);

    thread::scope(|s|{
        s.spawn(||{
            environment.run_round_robin_with_rewards().unwrap();
        });
        s.spawn(||{
            agent_east.run_rewarded().unwrap();
        });

        s.spawn(||{
            agent_south.run_rewarded().unwrap();
        });

        s.spawn(||{
            agent_west.run_rewarded().unwrap();
        });

        s.spawn(||{
            agent_north.run_rewarded().unwrap();
        });
    });

    Ok(model)
}



