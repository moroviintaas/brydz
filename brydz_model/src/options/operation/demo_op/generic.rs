use std::thread;
use brydz_core::bidding::Bid;
use brydz_core::cards::trump::TrumpGen;
use brydz_core::contract::{Contract, ContractParametersGen};
use brydz_core::deal::fair_bridge_deal;
use brydz_core::player::side::{Side, SideMap};
use brydz_core::amfiteatr::comm::ContractEnvSyncComm;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ContractAgentInfoSetSimple, ContractDummyState, ContractEnvStateMin};
use karty::hand::CardSet;
use karty::suits::Suit::Spades;
use amfiteatr_core::agent::{AgentGen, AutomaticAgent, RandomPolicy};
use amfiteatr_core::comm::DynEndpoint;
use amfiteatr_core::error::{CommunicationError, AmfiteatrError};
use amfiteatr_core::domain::{AgentMessage, EnvironmentMessage};
use amfiteatr_core::env::RoundRobinUniversalEnvironment;
//use amfiteatr_core::env::RoundRobinModelBuilder;
use amfiteatr_net_ext::{ComplexComm};
use amfiteatr_net_ext::tcp::TcpCommK2;
use brydz_core::amfiteatr::env::ContractEnv;

pub fn test_generic_model() -> Result<(), AmfiteatrError<ContractDP>>{
    type TcpCommSim = TcpCommK2<AgentMessage<ContractDP>, EnvironmentMessage<ContractDP>, CommunicationError<ContractDP>>;
    type TcpCommSimEnv = TcpCommK2<EnvironmentMessage<ContractDP>, AgentMessage<ContractDP>, CommunicationError<ContractDP>>;
    let contract_params = ContractParametersGen::new(Side::East, Bid::init(TrumpGen::Colored(Spades), 2).unwrap());
    let (comm_env_north, comm_north) = ContractEnvSyncComm::new_pair();

    let (comm_env_east, comm_east) = ContractEnvSyncComm::new_pair();
    let (comm_env_west, comm_west) = ContractEnvSyncComm::new_pair();
    let (_comm_env_south, _comm_south) = ContractEnvSyncComm::new_pair();

    let tcp_listener = std::net::TcpListener::bind("127.0.0.1:8420").unwrap();
    let (t, r) = std::sync::mpsc::channel();

    thread::spawn(move ||{
        let (south_stream_env_side, _) = tcp_listener.accept().unwrap();
        t.send(south_stream_env_side).unwrap();
    } );

    let stream_south_agent_side = std::net::TcpStream::connect("127.0.0.1:8420").unwrap();
    let south_stream_env_side = r.recv().unwrap();
    //let env_comm_south = ComplexComm::Tcp(TcpCommSimEnv::new(south_stream_env_side)) ;
    let env_comm_south = Box::new(TcpCommSimEnv::new(south_stream_env_side));
    let agent_comm_south = ComplexComm::Tcp(TcpCommSim::new(stream_south_agent_side));

/*
    let comm_env_north = ComplexComm2048::StdSync(comm_env_north);
    let comm_env_east = ComplexComm2048::StdSync(comm_env_east);
    let comm_env_south = ComplexComm2048::StdSync(comm_env_south);
    let comm_env_west = ComplexComm2048::StdSync(comm_env_west);

    let comm_north = ComplexComm2048::StdSync(comm_north);
    let comm_east = ComplexComm2048::StdSync(comm_east);
    let comm_south = ComplexComm2048::StdSync(comm_south);
    let comm_west = ComplexComm2048::StdSync(comm_west);
*/
    let card_deal = fair_bridge_deal::<CardSet>();
    let (hand_north, hand_east, hand_south, hand_west) = card_deal.destruct();
    let initial_contract = Contract::new(contract_params);

    let initial_state_east = ContractAgentInfoSetSimple::new(Side::East, hand_east, initial_contract.clone(), None);
    let initial_state_south = ContractAgentInfoSetSimple::new(Side::South, hand_south, initial_contract.clone(), None);
    let initial_state_west = ContractDummyState::new(Side::West, hand_west, initial_contract.clone());
    let initial_state_north = ContractAgentInfoSetSimple::new(Side::North, hand_north, initial_contract.clone(), None);

    let random_policy = RandomPolicy::<ContractDP, ContractAgentInfoSetSimple>::new();
    let policy_dummy = RandomPolicy::<ContractDP, ContractDummyState>::new();

    let mut agent_east = AgentGen::new(initial_state_east, comm_east, random_policy.clone() );
    let mut agent_south = AgentGen::new(initial_state_south, agent_comm_south, random_policy.clone() );
    let mut agent_west = AgentGen::new( initial_state_west, comm_west, policy_dummy);
    let mut agent_north = AgentGen::new( initial_state_north, comm_north, random_policy );


    /*
    let mut model = RoundRobinModelBuilder::new()
        .with_env_state(ContractEnvStateMin::new(initial_contract, None))?
        .add_local_agent(Arc::new(Mutex::new(agent_east)), DynEndpoint::Std(comm_env_east))?
        //.with_local_agent(Box::new(agent_south), agent_comm_south)?
        .add_local_agent(Arc::new(Mutex::new(agent_west)), DynEndpoint::Std(comm_env_west))?
        .add_local_agent(Arc::new(Mutex::new(agent_north)), DynEndpoint::Std(comm_env_north))?
        .with_remote_agent(Side::South, DynEndpoint::Dynamic(env_comm_south))?
        .build()?;

    thread::spawn(move || {
        agent_south.run().unwrap();
    });


    model.play().unwrap();


     */


    let comms = SideMap::new(
        DynEndpoint::Std(comm_env_north), DynEndpoint::Std(comm_env_east),
        DynEndpoint::Dynamic(env_comm_south), DynEndpoint::Std(comm_env_west));
    let mut environment = ContractEnv::new(ContractEnvStateMin::new(initial_contract, None), comms);






    thread::scope(|s|{
        s.spawn(||{
            environment.run_round_robin_with_rewards().unwrap();
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






    Ok(())
}