use std::thread;
use karty::set::CardSetStd;
use karty::suits::Suit::Spades;
use amfiteatr_core::agent::{AutomaticAgent, RandomPolicy, RewardedAgent, StatefulAgent, EvaluatedInformationSet};
use amfiteatr_core::env::RoundRobinUniversalEnvironment;
use crate::bidding::Bid;
use crate::cards::trump::TrumpGen;
use crate::contract::{Contract, ContractParametersGen};
use crate::deal::fair_bridge_deal;
use crate::player::side::{Side, SideMap};
use crate::player::side::Side::*;
use crate::amfiteatr::agent::TracingContractAgent;
use crate::amfiteatr::comm::ContractEnvSyncComm;
use crate::amfiteatr::env::ContractEnv;
use crate::amfiteatr::spec::ContractDP;
use crate::amfiteatr::state::{ContractAgentInfoSetSimple, ContractDummyState, ContractEnvStateMin};

mod env_agent;

#[test]
fn random_agents_sync_comm(){
    let contract = ContractParametersGen::new(Side::East, Bid::init(TrumpGen::Colored(Spades), 2).unwrap());
    let (comm_env_north, comm_north) = ContractEnvSyncComm::new_pair();
    let (comm_env_east, comm_east) = ContractEnvSyncComm::new_pair();
    let (comm_env_west, comm_west) = ContractEnvSyncComm::new_pair();
    let (comm_env_south, comm_south) = ContractEnvSyncComm::new_pair();

    let comm_assotiation = SideMap::new(comm_env_north, comm_env_east, comm_env_south, comm_env_west);
    let initial_contract = Contract::new(contract);

    let env_initial_state = ContractEnvStateMin::new(initial_contract.clone(), None);
    let mut simple_env = ContractEnv::new(env_initial_state, comm_assotiation);

    let card_deal = fair_bridge_deal::<CardSetStd>();
    let (hand_north, hand_east, hand_south, hand_west) = card_deal.destruct();

    let initial_state_east = ContractAgentInfoSetSimple::new(East, hand_east, initial_contract.clone(), None);
    let initial_state_south = ContractAgentInfoSetSimple::new(South, hand_south, initial_contract.clone(), None);
    let initial_state_west = ContractDummyState::new(West, hand_west, initial_contract.clone());
    let initial_state_north = ContractAgentInfoSetSimple::new(North, hand_north, initial_contract.clone(), None);


    let random_policy = RandomPolicy::<ContractDP, ContractAgentInfoSetSimple>::new();
    let policy_dummy = RandomPolicy::<ContractDP, ContractDummyState>::new();

    let mut agent_east = TracingContractAgent::new(initial_state_east, comm_east, random_policy.clone() );
    let mut agent_south = TracingContractAgent::new( initial_state_south, comm_south, random_policy.clone() );
    let mut agent_west = TracingContractAgent::new( initial_state_west, comm_west, policy_dummy);
    let mut agent_north = TracingContractAgent::new( initial_state_north, comm_north, random_policy );

    thread::scope(|s|{
        s.spawn(||{
            simple_env.run_round_robin_with_rewards().unwrap();
        });
        s.spawn(||{
            agent_east.run().unwrap();
        });

        s.spawn(||{
            agent_south.run().unwrap();
        });

        s.spawn(||{
            agent_west.run().unwrap();
        });

        s.spawn(||{
            agent_north.run().unwrap();
        });
    });

    assert_eq!(agent_east.info_set().current_assessment() + agent_north.info_set().current_assessment(), 13);
    assert_eq!(agent_east.current_universal_score() + agent_north.current_universal_score(), 13);
}