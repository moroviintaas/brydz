use std::thread;
use brydz_core::bidding::Bid;
use brydz_core::cards::trump::TrumpGen;
use brydz_core::contract::{Contract, ContractParametersGen};
use brydz_core::deal::fair_bridge_deal;
use brydz_core::player::side::{Side, SideMap};
use brydz_core::amfi::comm::ContractEnvSyncComm;
use brydz_core::amfi::env::ContractEnv;
use brydz_core::amfi::spec::ContractDP;
use brydz_core::amfi::state::{ContractAgentInfoSetSimple, ContractDummyState, ContractEnvStateMin};
use karty::hand::CardSet;
use karty::suits::Suit::Spades;
use amfiteatr_core::agent::{AgentGen, AutomaticAgent, RandomPolicy};
use amfiteatr_core::env::RoundRobinUniversalEnvironment;

pub fn tur_sim(){
    let contract = ContractParametersGen::new(Side::East, Bid::init(TrumpGen::Colored(Spades), 2).unwrap());
    let (comm_env_north, comm_north) = ContractEnvSyncComm::new_pair();
    let (comm_env_east, comm_east) = ContractEnvSyncComm::new_pair();
    let (comm_env_west, comm_west) = ContractEnvSyncComm::new_pair();
    let (comm_env_south, comm_south) = ContractEnvSyncComm::new_pair();

    let comm_association = SideMap::new(comm_env_north, comm_env_east, comm_env_south, comm_env_west);
    let initial_contract = Contract::new(contract);

    let env_initial_state = ContractEnvStateMin::new(initial_contract.clone(), None);
    let mut simple_env = ContractEnv::new(env_initial_state, comm_association);

    let card_deal = fair_bridge_deal::<CardSet>();
    let (hand_north, hand_east, hand_south, hand_west) = card_deal.destruct();

    let initial_state_east = ContractAgentInfoSetSimple::new(Side::East, hand_east, initial_contract.clone(), None);
    let initial_state_south = ContractAgentInfoSetSimple::new(Side::South, hand_south, initial_contract.clone(), None);
    let initial_state_west = ContractDummyState::new(Side::West, hand_west, initial_contract.clone());
    let initial_state_north = ContractAgentInfoSetSimple::new(Side::North, hand_north, initial_contract, None);


    let random_policy = RandomPolicy::<ContractDP, ContractAgentInfoSetSimple>::new();
    let policy_dummy = RandomPolicy::<ContractDP, ContractDummyState>::new();

    let mut agent_east = AgentGen::new(initial_state_east, comm_east, random_policy.clone() );
    let mut agent_south = AgentGen::new(initial_state_south, comm_south, random_policy.clone() );
    let mut agent_west = AgentGen::new( initial_state_west, comm_west, policy_dummy);
    let mut agent_north = AgentGen::new(initial_state_north, comm_north, random_policy );

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
    })

}